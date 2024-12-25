pub enum MyError {
    LicenseNotFound,
    DatabaseError,
    LicenseNotActivated,
    LicenseExpired,
    LicenseAlreadyActive,
    LicenseDoesNotExist,
    UpdateError,
    InvalidDuration
}