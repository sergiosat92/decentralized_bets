//! 📦 DATABASE CONNECTION & MIGRATIONS
//! ===================================
//!
//! This module handles PostgreSQL database setup, connection pooling,
//! and schema migrations using SeaORM and SeaORM Migration.
//!
//! ## Key Features
//! - Automatic database creation if it doesn't exist.
//! - Connection pool setup with configurable options.
//! - Automatic execution of schema migrations.
//!
//! ## Components
//! - `init_database`: Entry point for setting up and migrating the database.
//! - `create_pool`: Initializes a connection pool.
//! - `create_database_if_not_exists`: Attempts to create the DB before connecting.
//! - `run_migrations`: Runs SeaORM schema migrations.
//! - `Migrator`: Registers all available migrations.
//!
//! ## Notes
//! - Reads configuration from the global `CONFIG` cache.
//! - Logs setup steps and errors using the centralized logger.
//! - Fails fast on critical errors (e.g., connection or migration issues).

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr, Statement};
use log::LevelFilter;
use sea_orm_migration::MigratorTrait;
use sea_orm::RuntimeErr::Internal;
use url::Url;
use std::time::Duration;

use crate::infrastructure::{database::{cache::CONFIG, connection::migrator::Migrator}, observability::logs::logger};

pub mod migrator {
    use sea_orm_migration::prelude::*;

    use crate::infrastructure::database::migrations::m20230715_000001_create_users_table;

    /// 🔧 SeaORM Migrator Definition
    /// 
    /// Defines the list of migrations to be executed on startup.
    pub struct Migrator;

    #[async_trait::async_trait]
    impl MigratorTrait for Migrator {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![
                Box::new(m20230715_000001_create_users_table::Migration),
            ]
        }
    }
}

/// Runs registered migrations on the provided connection.
async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await?;
    Ok(())
}

/// Creates the PostgreSQL database if it doesn't exist.
async fn create_database_if_not_exists() -> Result<(), DbErr> {
    let db_url = CONFIG.secrets.database_url();
    let url = Url::parse(db_url).map_err(|e| DbErr::Conn(Internal(e.to_string())))?;
    let db_name = url.path_segments()
        .and_then(|mut segments| segments.next_back())
        .ok_or_else(|| DbErr::Conn(Internal("Invalid database URL: no database name".to_string())))?;

    let mut server_url = url.clone();
    server_url.set_path("/postgres");
    let server_url_str = server_url.to_string();

    let server_conn = Database::connect(&server_url_str).await?;

    let result = server_conn.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        format!(r#"CREATE DATABASE "{}""#, db_name),
    )).await;

    match result {
        Ok(_) => logger(tracing::Level::INFO, "create_database_if_not_exists", "✅ Database created successfully"),
        Err(e) if e.to_string().contains("already exists") =>
            logger(tracing::Level::INFO, "create_database_if_not_exists", "ℹ️ Database already exists. Skipping creation"),
        Err(e) => {
            logger(tracing::Level::ERROR, "create_database_if_not_exists", &format!("❌ Failed to create database: {}", e));
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Creates a connection pool to the PostgreSQL database.
async fn create_pool() -> Result<DatabaseConnection, DbErr> {
    let db_url = CONFIG.secrets.database_url();
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(40)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(3))
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Debug);

    Database::connect(opt).await
}

/// Initializes the database connection and runs migrations.
///
/// This is the main entry point called during app startup.
pub async fn init_database() -> DatabaseConnection {
    if let Err(e) = create_database_if_not_exists().await {
        logger(tracing::Level::ERROR, "init_database", &format!("❌ Failed to create database: {}", e));
        std::process::exit(1);
    }

    let pool = match create_pool().await {
        Ok(pool) => {
            logger(tracing::Level::INFO, "main", "✅ Database pool created successfully");
            pool
        },
        Err(e) => {
            logger(tracing::Level::ERROR, "main", &format!("❌ Failed to create database pool: {}", e));
            std::process::exit(1);
        }
    };

    if let Err(e) = run_migrations(&pool).await {
        logger(tracing::Level::ERROR, "main", &format!("❌ Database migrations failed: {}", e));
        std::process::exit(1);
    }

    logger(tracing::Level::INFO, "main", "✅ Database migrations completed successfully");
    pool
}
