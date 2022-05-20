use entity::user;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .string()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(user::Column::Email).string().not_null())
                    .col(ColumnDef::new(user::Column::Password).string().not_null())
                    .col(ColumnDef::new(user::Column::DiscordId).string()) // Nullable
                    .to_owned()
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await?;

        Ok(())
    }
}
