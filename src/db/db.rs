use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use crate::models::license::License;
use crate::models::errors::MyError;
use chrono::Utc;
use futures::stream::StreamExt;

const SECONDS_IN_DAY: i64 = 24 * 60 * 60;

pub struct DbRepo {
    col: Collection<License>,
}

impl DbRepo {
    pub async fn new() -> DbRepo {
        let uri = dotenv::var("MONGOURI").expect("MONGOURI missing");
        let client = Client::with_uri_str(&uri).await.unwrap();
        let db = client.database("rustDB");
        let col = db.collection::<License>("license");
        DbRepo {col}
    }
    pub async fn create_license(&self, license: License) -> Result<InsertOneResult, MyError>{
        let res = self.col.insert_one(license).await;

        match res {
            Ok(res) => Ok(res),
            Err(_e) => Err(MyError::DatabaseError)
        }
    }
    pub async fn delete_license(&self, license: String) -> Result<DeleteResult, MyError> {
        let delete_result = self.col.delete_one(doc! { "license": license }).await;

        match delete_result {
            Ok(delete_result) => {
                if delete_result.deleted_count == 1 {
                    Ok(delete_result)
                } else {
                    Err(MyError::LicenseDoesNotExist)
                }
            },
            Err(_) => Err(MyError::DatabaseError)
        }
    }
    pub async fn get_license(&self, license: String) -> Result<License, MyError> {
        let curr = self.col.find_one(doc! { "license": license }).await;

        match curr {
            Ok(ans) => {
                match ans {
                    Some(license) => Ok(license),
                    None => Err(MyError::LicenseNotFound),
                }
            }

            Err(_) => { Err(MyError::DatabaseError)}
        }
    }
    pub async fn get_licenses_by_user(&self, wallet: String) -> Result<Vec<License>, Error> {
        let mut licenses = self.col.find(doc! { "wallet": wallet }).await?;
        let mut ans: Vec<License> = Vec::new();
        while let Some(result) = licenses.next().await {
            match result {
                Ok(license) => ans.push(license),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Ok(ans)
    }
    pub async fn renew_license(&self, license: String, days: i64) -> Result<UpdateResult, MyError> {
        // Получаем текущую лицензию, если нет, то пропускаем
        let license = self.get_license(license.clone()).await?;

        // Если лицензия не активирована
        if license.activated == false || license.expiration.is_none() {
            return Err(MyError::LicenseNotActivated)
        }

        // В течение трёх дней после просрочки можно продлить лицензию
        let expiration_delta = license.expiration.unwrap() + SECONDS_IN_DAY * 3;

        if expiration_delta < Utc::now().timestamp() {
            return Err(MyError::LicenseExpired)
        }

        let new_expiration = license.expiration.unwrap() + days * SECONDS_IN_DAY;
        let filter = doc! {"license": license.license };
        let update = doc! {"$set": { "expiration": Option::from(new_expiration) }};

        // Обновляем подписку
        let update_result = self.col.update_one(filter, update).await;

        match update_result {
            Ok(res) => Ok(res),
            Err(_e) => Err(MyError::DatabaseError)
        }
    }
    pub async fn activate_license(&self, license: String, days: i64) -> Result<UpdateResult, MyError> {
        // Input validation
        if days <= 0 {
            return Err(MyError::InvalidDuration);
        }

        // Fetch license details
        let license = self.get_license(license.clone()).await?;

        // Check if already activated
        if license.activated {
            return Err(MyError::LicenseAlreadyActive);
        }

        let expiration = Utc::now().timestamp() + days * SECONDS_IN_DAY;

        // Prepare update operation
        let filter = doc! {
            "license": license.license,
            "activated": false
        };

        let update = doc! {"$set": {
            "expiration": Option::from(expiration),
            "activated": true
        }};

        let update_result = self.col.update_one(filter, update).await;

        match update_result {
            Ok(result) => {
                match result.modified_count {
                    1 => Ok(result),
                    0 => Err(MyError::LicenseNotFound),
                    _ => Err(MyError::DatabaseError)
                }
            }
            Err(db_error) => {
                // Log the actual database error here
                log::error!("Database error during license activation: {:?}", db_error);
                Err(MyError::DatabaseError)
            }
        }
    }
}