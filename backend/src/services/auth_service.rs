use validator::Validate;

use crate::config::app_config::AppConfig;
use crate::config::database::DbConn;
use crate::errors::AppError;
use crate::models::usuario::NovoUsuario;
use crate::repositories::usuario_repository;
use crate::schemas::auth_dto::{AuthResponse, LoginRequest, RegisterRequest, UsuarioPublico};
use crate::utils::{jwt, password};

pub fn registrar(conn: &mut DbConn, dto: RegisterRequest) -> Result<AuthResponse, AppError> {

    dto.validate().map_err(|e| {
        AppError::BadRequest(format!("Dados inválidos: {}", e))
    })?;

    if let Some(_) = usuario_repository::buscar_por_email(conn, &dto.email)? {
        return Err(AppError::Conflict("Email já cadastrado".to_string()));
    }

    let senha_hash = password::hash_senha(&dto.senha)?;

    let novo_usuario = NovoUsuario {
        nome: dto.nome,
        email: dto.email,
        senha_hash,
    };
    let usuario = usuario_repository::criar(conn, novo_usuario)?;

    let config = AppConfig::carregar();
    let token = jwt::gerar_token(
        usuario.id,
        &usuario.email,
        &config.jwt_secret,
        config.jwt_expiration_hours,
    )?;

    Ok(AuthResponse {
        token,
        usuario: UsuarioPublico {
            id: usuario.id,
            nome: usuario.nome,
            email: usuario.email,
            criado_em: usuario.criado_em.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        },
    })
}

pub fn login(conn: &mut DbConn, dto: LoginRequest) -> Result<AuthResponse, AppError> {

    dto.validate().map_err(|e| {
        AppError::BadRequest(format!("Dados inválidos: {}", e))
    })?;

    let usuario = usuario_repository::buscar_por_email(conn, &dto.email)?
        .ok_or_else(|| AppError::Unauthorized("Credenciais inválidas".to_string()))?;

    let senha_valida = password::verificar_senha(&dto.senha, &usuario.senha_hash)?;
    if !senha_valida {
        return Err(AppError::Unauthorized("Credenciais inválidas".to_string()));
    }

    let config = AppConfig::carregar();
    let token = jwt::gerar_token(
        usuario.id,
        &usuario.email,
        &config.jwt_secret,
        config.jwt_expiration_hours,
    )?;

    Ok(AuthResponse {
        token,
        usuario: UsuarioPublico {
            id: usuario.id,
            nome: usuario.nome,
            email: usuario.email,
            criado_em: usuario.criado_em.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        },
    })
}

pub fn buscar_perfil(conn: &mut DbConn, user_id: i32) -> Result<UsuarioPublico, AppError> {
    let usuario = usuario_repository::buscar_por_id(conn, user_id)?;

    Ok(UsuarioPublico {
        id: usuario.id,
        nome: usuario.nome,
        email: usuario.email,
        criado_em: usuario.criado_em.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    })
}
