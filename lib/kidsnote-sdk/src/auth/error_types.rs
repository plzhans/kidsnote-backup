use num_enum::TryFromPrimitive;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("ErrorCode: {0}")]
    ErrorWithCode(AuthErrorCode),
    #[error("ErrorCode: {0} / {1}")]
    ErrorWithCodeStr(AuthErrorCode, &'static str),
    #[error("ErrorCode: {0} / {1}")]
    ErrorWithCodeString(AuthErrorCode, String),
    #[error("Error: {0}")]
    GeneralError(&'static str),
    #[error("Error: {0}")]
    GeneralErrorStr(String),
}

#[repr(u16)]
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy, TryFromPrimitive, Error)]
pub enum AuthErrorCode {
    None = 0x0000,
    Nothingtodo = 1,
    Unauthorized = 401,
    NotFound = 404,
    ServerError = 0xfffe,
    UnknownError = 0xffff,
}

impl AuthErrorCode {
    pub const fn as_u16(&self) -> u16 {
        *self as u16
    }
    pub fn from_u16(num: u16) -> Self {
        match Self::try_from_primitive(num) {
            Ok(v) => v,
            Err(_) => AuthErrorCode::UnknownError,
        }
    }
    pub const fn desc(&self) -> &str {
        if self.as_u16() == 0 {
            return "OK";
        }
        match self {
            // 할 작업이 없습니다.
            AuthErrorCode::Nothingtodo => "Nothing to do",
            // 찾을 수 없습니다.
            AuthErrorCode::NotFound => "Not Found",
            // 미인증
            AuthErrorCode::Unauthorized => "Unauthorized",
            // 서버오류가 발생했습니다.
            AuthErrorCode::ServerError => "Server Processing Error Occured",
            // 알수없는 오류가 발생했습니다.
            AuthErrorCode::UnknownError => "Unknown Error Occured",
            // OK
            AuthErrorCode::None => "OK",
        }
    }
}

impl std::fmt::Display for AuthErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc())
    }
}
