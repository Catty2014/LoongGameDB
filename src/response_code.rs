pub enum ResponseCode {
    Success = 0,
    NotImplemented = 998,
    SystemInternalError = 999,
    LoginCsrfViolation = 1001,
    DatabaseConnectionError = 2001,
    SonicDBConnectionError = 3001,
}

impl Into<u32> for ResponseCode {
    fn into(self) -> u32 {
        self as u32
    }
}
