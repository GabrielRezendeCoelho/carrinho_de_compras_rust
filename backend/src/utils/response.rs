use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MensagemResponse {
    pub mensagem: String,
}

impl MensagemResponse {

    pub fn new(mensagem: impl Into<String>) -> Json<Self> {
        Json(MensagemResponse {
            mensagem: mensagem.into(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct DadosResponse<T: Serialize> {
    pub dados: T,
}

impl<T: Serialize> DadosResponse<T> {

    pub fn new(dados: T) -> Json<Self> {
        Json(DadosResponse { dados })
    }
}
