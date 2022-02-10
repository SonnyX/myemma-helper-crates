#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ApiErrorKind {
    /// An internal error
    /// HTTP Status code: 500
    ServerError,
    /// HTTP Status code: 400
    BadRequest,
    /// HTTP Status code: 401
    Unauthorized,
    /// HTTP Status code: 403
    Forbidden,
    /// HTTP Status code: 404
    NotFound,
    /// HTTP Status code: 422
    UnprocessableEntity,
    /// Error message that never leaves server.
    /// HTTP Status code: 500
    PrivateError,
}

impl Default for ApiErrorKind {
    fn default() -> Self {
        ApiErrorKind::PrivateError
    }
}

impl ApiErrorKind {
    pub fn is_server_error(&self) -> bool {
        *self == ApiErrorKind::ServerError || *self == ApiErrorKind::PrivateError
    }
}

impl From<u16> for ApiErrorKind {
    fn from(kind: u16) -> Self {
        use ApiErrorKind::*;
        match kind {
            500 => ServerError,
            400 => BadRequest,
            401 => Unauthorized,
            403 => Forbidden,
            404 => NotFound,
            422 => UnprocessableEntity,
            _ => PrivateError,
        }
    }
}

impl From<ApiErrorKind> for u16 {
    fn from(kind: ApiErrorKind) -> Self {
        use ApiErrorKind::*;
        match kind {
            ServerError => 500,
            BadRequest => 400,
            Unauthorized => 401,
            Forbidden => 403,
            NotFound => 404,
            UnprocessableEntity => 422,
            PrivateError => 500,
        }
    }
}
