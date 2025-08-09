use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::FirstName).string())
                    .col(ColumnDef::new(User::LastName).string())
                    .col(
                        ColumnDef::new(User::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(User::IsLocked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::FailedLoginAttempts)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(User::LockoutUntil).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::VerificationToken).string())
                    .col(ColumnDef::new(User::VerificationTokenExpires).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::PasswordResetToken).string())
                    .col(ColumnDef::new(User::PasswordResetExpires).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::LastLogin).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::DeletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(User::Role)
                            .string()
                            .not_null()
                            .default("user"),
                    )
                    .to_owned(),
            )
            .await?;

        // Crear índices adicionales
        manager
            .create_index(
                Index::create()
                    .name("idx_users_role")
                    .table(User::Table)
                    .col(User::Role)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_is_active")
                    .table(User::Table)
                    .col(User::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Email,
    Username,
    PasswordHash,
    FirstName,
    LastName,
    IsVerified,
    IsActive,
    IsLocked,
    FailedLoginAttempts,
    LockoutUntil,
    VerificationToken,
    VerificationTokenExpires,
    PasswordResetToken,
    PasswordResetExpires,
    LastLogin,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Role,
}