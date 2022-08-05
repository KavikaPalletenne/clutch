use entity::{role, group};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220728_000001_create_role_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(role::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(role::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(role::Column::Name).string().not_null())
                    .col(ColumnDef::new(role::Column::GroupId).string().not_null())
                    .to_owned()
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("FK_group_id")
                    .table(role::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_group_id")
                    .from(role::Entity, role::Column::GroupId)
                    .to(group::Entity, group::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        // manager
        //     .drop_index(
        //         Index::drop()
        //             .name("idx-group-id")
        //             .table(role::Entity)
        //             .to_owned()
        //     ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-role-group-id")
                    .table(role::Entity)
                    .col(role::Column::GroupId)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(role::Entity).to_owned())
            .await?;

        Ok(())
    }
}
