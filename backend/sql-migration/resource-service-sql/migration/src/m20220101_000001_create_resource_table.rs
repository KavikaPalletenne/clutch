use entity::resource::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_resource_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .string()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Column::UserId).string().not_null())
                    .col(ColumnDef::new(Column::GroupId).string().not_null())
                    .col(ColumnDef::new(Column::Title).string().not_null())
                    .col(ColumnDef::new(Column::Description).text())
                    .col(ColumnDef::new(Column::Subject).string().not_null())
                    .col(ColumnDef::new(Column::LastEditedAt).timestamp_with_time_zone().not_null()) // TODO: Find out how to auto add current timestamp
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
