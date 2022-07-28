use entity::{user_role, user, role};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220728_000001_create_user_role_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user_role::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user_role::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(user_role::Column::UserId).string().not_null())
                    .col(ColumnDef::new(user_role::Column::RoleId).big_integer().not_null())
                    .to_owned()
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("FK_user_id")
                    .table(user_role::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_user_id")
                    .from(user_role::Entity, user_role::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("FK_role_id")
                    .table(user_role::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_role_id")
                    .from(user_role::Entity, user_role::Column::RoleId)
                    .to(role::Entity, role::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-user-id")
                    .table(user_role::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-user-id")
                    .table(user_role::Entity)
                    .col(user_role::Column::UserId)
                    .to_owned()
            ).await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx-role-id")
                    .table(user_role::Entity)
                    .to_owned()
            ).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-role-id")
                    .table(user_role::Entity)
                    .col(user_role::Column::RoleId)
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
