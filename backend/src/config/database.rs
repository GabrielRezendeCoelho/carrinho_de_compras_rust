use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub type DbConn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn inicializar_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL deve estar definida no .env");

    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Falha ao criar pool de conexões com o banco de dados")
}

pub fn obter_conexao(pool: &DbPool) -> Result<DbConn, String> {
    pool.get().map_err(|e| {
        log::error!("Erro ao obter conexão do pool: {:?}", e);
        format!("Erro ao conectar ao banco de dados: {}", e)
    })
}

pub fn executar_migrations(conn: &mut SqliteConnection) {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Falha ao executar migrations do banco de dados");

    log::info!("Migrations executadas com sucesso");
}
