use entity::tag;
use entity::resource;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_resource_tag_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(tag::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(tag::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(tag::Column::Text).string().not_null())
                    .col(ColumnDef::new(tag::Column::ResourceId).big_unsigned().not_null())
                    .to_owned()
            ).await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("FK_resource_id")
                    .table(tag::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
            ForeignKey::create()
                .name("FK_resource_id")
                .from(tag::Entity, tag::Column::ResourceId)
                .to(resource::Entity, resource::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                .to_owned()
            ).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-resource-id")
                    .table(tag::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-resource-tag-resource-id")
                    .table(tag::Entity)
                    .col(tag::Column::ResourceId)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(tag::Entity).to_owned())
            .await?;

        Ok(())
    }
}
