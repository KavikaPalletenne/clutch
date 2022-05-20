use entity::group;
use entity::resource;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_group_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(group::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(group::Column::Id)
                            .string()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(group::Column::Name).string().not_null())
                    .col(ColumnDef::new(group::Column::Description).string().not_null())
                    .col(ColumnDef::new(group::Column::DiscordId).string().not_null())
                    .to_owned()
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(group::Entity).to_owned())
            .await?;

        Ok(())
    }
}
