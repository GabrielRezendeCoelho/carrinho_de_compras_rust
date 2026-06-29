use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::errors::AppError;

pub fn hash_senha(senha: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(senha.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| {
            log::error!("Erro ao gerar hash da senha: {:?}", e);
            AppError::InternalError("Erro ao processar senha".to_string())
        })
}

pub fn verificar_senha(senha: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| {
        log::error!("Erro ao parsear hash da senha: {:?}", e);
        AppError::InternalError("Erro ao verificar senha".to_string())
    })?;

    Ok(Argon2::default()
        .verify_password(senha.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_e_verificar_senha() {
        let senha = "MinhaSenha@123";
        let hash = hash_senha(senha).unwrap();

        assert_ne!(hash, senha);

        assert!(verificar_senha(senha, &hash).unwrap());
    }

    #[test]
    fn test_senha_incorreta() {
        let hash = hash_senha("SenhaCorreta").unwrap();
        assert!(!verificar_senha("SenhaErrada", &hash).unwrap());
    }

    #[test]
    fn test_hash_diferente_a_cada_chamada() {
        let hash1 = hash_senha("MesmaSenha").unwrap();
        let hash2 = hash_senha("MesmaSenha").unwrap();

        assert_ne!(hash1, hash2);

        assert!(verificar_senha("MesmaSenha", &hash1).unwrap());
        assert!(verificar_senha("MesmaSenha", &hash2).unwrap());
    }
}
