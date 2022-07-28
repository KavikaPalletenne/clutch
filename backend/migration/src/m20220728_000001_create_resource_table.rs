use entity::resource;
use entity::user;
use entity::group;
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
                    .table(resource::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(resource::Column::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(resource::Column::UserId).string().not_null())
                    .col(ColumnDef::new(resource::Column::GroupId).string().not_null())
                    .col(ColumnDef::new(resource::Column::Title).string().not_null())
                    .col(ColumnDef::new(resource::Column::Description).text())
                    .col(ColumnDef::new(resource::Column::Subject).string().not_null())
                    .col(ColumnDef::new(resource::Column::LastEditedAt).timestamp_with_time_zone().not_null()) // TODO: Find out how to auto add current timestamp
                    .to_owned()
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("FK_user_id")
                    .table(resource::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_user_id")
                    .from(resource::Entity, resource::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("FK_group_id")
                    .table(resource::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_group_id")
                    .from(resource::Entity, resource::Column::GroupId)
                    .to(group::Entity, group::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-user-id")
                    .table(resource::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-user-id")
                    .table(resource::Entity)
                    .col(resource::Column::UserId)
                    .to_owned()
            ).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-group-id")
                    .table(resource::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-group-id")
                    .table(resource::Entity)
                    .col(resource::Column::GroupId)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(resource::Entity).to_owned())
            .await
    }
}
