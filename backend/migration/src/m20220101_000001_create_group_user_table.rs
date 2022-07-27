use entity::{group, group_user, user};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_group_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(group_user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(group_user::Column::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(group_user::Column::UserId).string().not_null())
                    .col(ColumnDef::new(group_user::Column::GroupId).string().not_null())
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
            ForeignKey::create()
                .name("FK_group_id")
                .from(group_user::Entity, group_user::Column::GroupId)
                .to(group::Entity, group::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                .to_owned()
            ).await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_user_id")
                    .from(group_user::Entity, group_user::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-group-id")
                    .table(group_user::Entity)
                    .col(group_user::Column::GroupId)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-user-id")
                    .table(group_user::Entity)
                    .col(group_user::Column::UserId)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(group_user::Entity).to_owned())
            .await?;

        Ok(())
    }
}
