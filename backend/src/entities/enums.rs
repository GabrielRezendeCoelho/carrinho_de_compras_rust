
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusPedido {
    #[serde(rename = "pendente")]
    Pendente,
    #[serde(rename = "confirmado")]
    Confirmado,
    #[serde(rename = "enviado")]
    Enviado,
    #[serde(rename = "entregue")]
    Entregue,
    #[serde(rename = "cancelado")]
    Cancelado,
}

impl StatusPedido {

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pendente" => Ok(StatusPedido::Pendente),
            "confirmado" => Ok(StatusPedido::Confirmado),
            "enviado" => Ok(StatusPedido::Enviado),
            "entregue" => Ok(StatusPedido::Entregue),
            "cancelado" => Ok(StatusPedido::Cancelado),
            outro => Err(format!("Status inválido: {}", outro)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            StatusPedido::Pendente => "pendente",
            StatusPedido::Confirmado => "confirmado",
            StatusPedido::Enviado => "enviado",
            StatusPedido::Entregue => "entregue",
            StatusPedido::Cancelado => "cancelado",
        }
    }

    /// Verifica se o pedido pode ser cancelado.
    /// Regra de negócio: só pode cancelar se estiver pendente ou confirmado.
    pub fn pode_cancelar(&self) -> bool {
        matches!(self, StatusPedido::Pendente | StatusPedido::Confirmado)
    }
}

impl fmt::Display for StatusPedido {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_str_valido() {
        assert_eq!(StatusPedido::from_str("pendente").unwrap(), StatusPedido::Pendente);
        assert_eq!(StatusPedido::from_str("cancelado").unwrap(), StatusPedido::Cancelado);
    }

    #[test]
    fn test_status_from_str_invalido() {
        assert!(StatusPedido::from_str("inexistente").is_err());
    }

    #[test]
    fn test_pode_cancelar() {
        assert!(StatusPedido::Pendente.pode_cancelar());
        assert!(StatusPedido::Confirmado.pode_cancelar());
        assert!(!StatusPedido::Enviado.pode_cancelar());
        assert!(!StatusPedido::Entregue.pode_cancelar());
        assert!(!StatusPedido::Cancelado.pode_cancelar());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", StatusPedido::Pendente), "pendente");
    }
}
