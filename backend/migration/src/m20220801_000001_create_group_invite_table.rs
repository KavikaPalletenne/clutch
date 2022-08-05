use entity::{group, group_invite};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220801_000001_create_group_invite_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(group_invite::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(group_invite::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(group_invite::Column::GroupId).string().not_null())
                    .col(ColumnDef::new(group_invite::Column::Code).string().not_null())
                    .col(ColumnDef::new(group_invite::Column::Expiry).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(group_invite::Column::Uses).big_integer().not_null())
                    .col(ColumnDef::new(group_invite::Column::Creator).string().not_null())
                    .to_owned()
            ).await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("FK_group_id")
                    .table(group_invite::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_group_id")
                    .from(group_invite::Entity, group_invite::Column::GroupId)
                    .to(group::Entity, group::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-group-invites-group-id")
                    .table(group_invite::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-group-invite-group-id")
                    .table(group_invite::Entity)
                    .col(group_invite::Column::GroupId)
                    .to_owned()
            ).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-group-invites-code")
                    .table(group_invite::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-group-invite-code")
                    .table(group_invite::Entity)
                    .col(group_invite::Column::Code)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(group_invite::Entity).to_owned())
            .await?;

        Ok(())
    }
}
