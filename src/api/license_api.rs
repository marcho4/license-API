use actix_web::{get, post, web, HttpResponse};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use crate::models::license::License;
use crate::AppState;
use crate::models::api_models::{ApiResponse, LicenseData, RenewData};
use crate::models::errors::MyError;

/// Узнать информацию о лицензии
/// Готово
#[get("/get/{key}")]
async fn get_license(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let license = path.into_inner();
    let lock = data.db.lock().await;
    let ans = lock.get_license(license).await;
    match ans {
        Ok(lic) => HttpResponse::Ok().json(
            ApiResponse {
                msg: Some(String::from("success")),
                data: Some(lic),
        }),
        Err(_) => HttpResponse::InternalServerError().json(
            ApiResponse::<License> {msg: Some(String::from("Not found")), data: None }),
    }
}

/// Создание лицензии
/// Готово
#[post("/create")]
async fn add_license(license: web::Json<License>, data: web::Data<AppState>) -> HttpResponse {
    let lock = data.db.lock().await;
    let insert_result = lock.create_license(license.into_inner()).await;
    match insert_result {
        Ok(lic) => HttpResponse::Ok().json(ApiResponse {
            msg: Some(String::from("Successfully inserted")),
            data: Some(lic)
        }),
        Err(_) => HttpResponse::InternalServerError().json(
            ApiResponse::<InsertOneResult> {
                msg: Some(String::from("Error happened when inserting data")),
                data: None
            }
        ),
    }
}


/// Продление лицензии
#[post("/renew")]
async fn renew_license(data: web::Data<AppState>, req_data: web::Json<RenewData>) -> HttpResponse {
    let lock = data.db.lock().await;
    let data = req_data.into_inner();
    let update_result = lock.renew_license(data.license, data.days).await;
    match update_result {
        Ok(lic) => HttpResponse::Ok().json(ApiResponse {
            msg: Some(String::from("Success")),
            data: Some(lic)
        }),
        Err(MyError::LicenseNotActivated) => HttpResponse::InternalServerError().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("License hasn't been activated yet")),
                data: None
            }
        ),
        Err(MyError::DatabaseError) => HttpResponse::InternalServerError().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("Database error")),
                data: None
            }
        ),
        Err(MyError::LicenseExpired) => HttpResponse::BadRequest().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("License expired. Can not renew")),
                data: None
            }
        ),
        Err(MyError::LicenseNotFound) => HttpResponse::NotFound().json(
            ApiResponse::<UpdateResult> {
            msg: Some(String::from("License not found")),
            data: None
        }),

        _ => HttpResponse::InternalServerError().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("Internal error")),
                data: None
            }
        )
    }
}


/// Удаление лицензии
#[post("/delete")]
async fn delete_license(data: web::Data<AppState>, req_data: web::Json<LicenseData>) -> HttpResponse {
    let db = data.db.lock().await;
    let data = req_data.into_inner();
    let delete_result = db.delete_license(data.license).await;

    match delete_result {
        Ok(res) => HttpResponse::Ok().json(ApiResponse {
            msg: Some(String::from("Success")),
            data: Some(res),
        }),
        Err(MyError::LicenseDoesNotExist) => HttpResponse::BadRequest().json(
            ApiResponse::<DeleteResult> {
                msg: Some(String::from("License does not exist")),
                data: None
            }
        ),
        Err(MyError::DatabaseError) => HttpResponse::InternalServerError().json(
            ApiResponse::<DeleteResult> {
                msg: Some(String::from("Database error")),
                data: None
            }
        ),
        _ => HttpResponse::InternalServerError().json(
            ApiResponse::<DeleteResult> {
                msg: Some(String::from("Not found")),
                data: None
            }
        )
    }
}


#[get("/all/{wallet}")]
async fn get_all_licenses(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let wallet = path.into_inner();
    let lock = data.db.lock().await;
    let res = lock.get_licenses_by_user(wallet).await;
    match res {
        Ok(licenses) => HttpResponse::Ok().json(ApiResponse::<Vec<License>> {
            msg: Some(String::from("success")),
            data: Some(licenses),
        }),
        Err(_) => HttpResponse::InternalServerError().json(
            ApiResponse::<Vec<License>> {
                msg: Some(String::from("Not found")),
                data: None
            }
        )
    }
}

#[post("/activate")]
async fn activate(data: web::Data<AppState>, body: web::Json<RenewData>) -> HttpResponse {
    let body = body.into_inner();
    let (key, days) = (body.license, body.days);
    let db = data.db.lock().await;
    let upd_res = db.activate_license(key, days).await;
    match upd_res {
        Ok(res) => HttpResponse::Ok().json(ApiResponse {
            msg: Some(String::from("Success")),
            data: Some(res)
        }),
        Err(MyError::InvalidDuration) => HttpResponse::BadRequest().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("Invalid duration. It should be greater than 0")),
                data: None
            }
        ),
        Err(MyError::LicenseAlreadyActive) => HttpResponse::BadRequest().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("License has already been activated")),
                data: None
            }
        ),
        Err(MyError::LicenseNotFound) => HttpResponse::NotFound().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("License not found")),
                data: None
            }
        ),
        Err(MyError::DatabaseError) => HttpResponse::InternalServerError().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("Database error")),
                data: None
            }
        ),
        Err(_) => HttpResponse::InternalServerError().json(
            ApiResponse::<UpdateResult> {
                msg: Some(String::from("Internal Error Happened")),
                data: None
            }
        )
    }
}