use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {

    BadRequest(String),

    Unauthorized(String),

    Forbidden(String),

    NotFound(String),

    Conflict(String),

    InternalError(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub erro: String,
    pub codigo: u16,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl AppError {
    /// Retorna o Status HTTP correspondente ao tipo de erro.
    /// Demonstra Pattern Matching com match.
    fn status(&self) -> Status {
        match self {
            AppError::BadRequest(_) => Status::BadRequest,
            AppError::Unauthorized(_) => Status::Unauthorized,
            AppError::Forbidden(_) => Status::Forbidden,
            AppError::NotFound(_) => Status::NotFound,
            AppError::Conflict(_) => Status::Conflict,
            AppError::InternalError(_) => Status::InternalServerError,
        }
    }

    /// Extrai a mensagem de erro. Demonstra Borrowing com referências.
    fn message(&self) -> &str {
        match self {
            AppError::BadRequest(msg)
            | AppError::Unauthorized(msg)
            | AppError::Forbidden(msg)
            | AppError::NotFound(msg)
            | AppError::Conflict(msg)
            | AppError::InternalError(msg) => msg,
        }
    }
}

/// Implementação do trait Responder do Rocket para conversão automática
/// de AppError em respostas HTTP JSON.
impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let status = self.status();
        let error_response = ErrorResponse {
            erro: self.message().to_string(),
            codigo: status.code,
        };

        Response::build_from(Json(error_response).respond_to(req)?)
            .status(status)
            .ok()
    }
}

/// Conversão automática de diesel::result::Error para AppError.
/// Demonstra implementação do trait From<> para conversão ergonômica com `?`.
impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Recurso não encontrado".to_string())
            }
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info,
            ) => AppError::Conflict(format!(
                "Registro duplicado: {}",
                info.message()
            )),
            _ => {
                log::error!("Erro de banco de dados: {:?}", err);
                AppError::InternalError("Erro interno do banco de dados".to_string())
            }
        }
    }
}

/// Conversão de erros de JWT para AppError.
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        log::error!("Erro de JWT: {:?}", err);
        AppError::Unauthorized("Token inválido ou expirado".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(AppError::BadRequest("".into()).status(), Status::BadRequest);
        assert_eq!(AppError::Unauthorized("".into()).status(), Status::Unauthorized);
        assert_eq!(AppError::Forbidden("".into()).status(), Status::Forbidden);
        assert_eq!(AppError::NotFound("".into()).status(), Status::NotFound);
        assert_eq!(AppError::Conflict("".into()).status(), Status::Conflict);
        assert_eq!(AppError::InternalError("".into()).status(), Status::InternalServerError);
    }

    #[test]
    fn test_error_message() {
        let err = AppError::BadRequest("campo obrigatório".to_string());
        assert_eq!(err.message(), "campo obrigatório");
    }

    #[test]
    fn test_error_display() {
        let err = AppError::NotFound("produto não encontrado".to_string());
        assert_eq!(format!("{}", err), "Not Found: produto não encontrado");
    }
}
