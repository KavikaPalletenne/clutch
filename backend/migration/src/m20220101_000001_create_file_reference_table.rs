use entity::file_reference;
use entity::resource;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_file_reference_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(file_reference::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(file_reference::Column::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(file_reference::Column::Name).string().not_null())
                    .col(ColumnDef::new(file_reference::Column::Size).integer().not_null())
                    .col(ColumnDef::new(file_reference::Column::ResourceId).big_unsigned().not_null())
                    .to_owned()
            )
            .await;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_resource_id")
                    .from(file_reference::Entity, file_reference::Column::ResourceId)
                    .to(resource::Entity, resource::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await;
        manager
            .create_index(
                Index::create()
                    .name("idx-resource-id")
                    .table(file_reference::Entity)
                    .col(file_reference::Column::ResourceId)
                    .to_owned()
            ).await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(file_reference::Entity).to_owned())
            .await?;

        Ok(())
    }
}
