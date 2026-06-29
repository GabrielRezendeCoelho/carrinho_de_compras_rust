use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginacaoParams {

    pub pagina: Option<i64>,

    pub por_pagina: Option<i64>,
}

impl PaginacaoParams {

    pub fn pagina(&self) -> i64 {
        self.pagina.unwrap_or(1).max(1)
    }

    pub fn por_pagina(&self) -> i64 {
        self.por_pagina.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        (self.pagina() - 1) * self.por_pagina()
    }
}

#[derive(Debug, Serialize)]
pub struct RespostaPaginada<T: Serialize> {
    pub dados: Vec<T>,
    pub total: i64,
    pub pagina: i64,
    pub por_pagina: i64,
    pub total_paginas: i64,
}

impl<T: Serialize> RespostaPaginada<T> {

    pub fn new(dados: Vec<T>, total: i64, pagina: i64, por_pagina: i64) -> Self {
        let total_paginas = if total == 0 {
            1
        } else {
            (total as f64 / por_pagina as f64).ceil() as i64
        };

        RespostaPaginada {
            dados,
            total,
            pagina,
            por_pagina,
            total_paginas,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginacao_defaults() {
        let params = PaginacaoParams {
            pagina: None,
            por_pagina: None,
        };
        assert_eq!(params.pagina(), 1);
        assert_eq!(params.por_pagina(), 10);
        assert_eq!(params.offset(), 0);
    }

    #[test]
    fn test_paginacao_custom() {
        let params = PaginacaoParams {
            pagina: Some(3),
            por_pagina: Some(20),
        };
        assert_eq!(params.pagina(), 3);
        assert_eq!(params.por_pagina(), 20);
        assert_eq!(params.offset(), 40);
    }

    #[test]
    fn test_paginacao_limites() {
        let params = PaginacaoParams {
            pagina: Some(-5),
            por_pagina: Some(500),
        };
        assert_eq!(params.pagina(), 1);
        assert_eq!(params.por_pagina(), 100);
    }

    #[test]
    fn test_resposta_paginada() {
        let resposta = RespostaPaginada::new(vec![1, 2, 3], 25, 1, 10);
        assert_eq!(resposta.total_paginas, 3);
        assert_eq!(resposta.dados.len(), 3);
    }

    #[test]
    fn test_resposta_paginada_vazia() {
        let resposta: RespostaPaginada<i32> = RespostaPaginada::new(vec![], 0, 1, 10);
        assert_eq!(resposta.total_paginas, 1);
    }
}
