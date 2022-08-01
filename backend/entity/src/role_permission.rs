use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::DeleteMany;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "role_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub role_id: i64,
    pub key: String, // e.g. "resource_delete, user_ban"
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::role::Entity",
    from = "Column::RoleId",
    to = "super::role::Column::Id"
    )]
    Role,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Entity {
    pub fn find_by_role_and_permission(role_id: i64, permission: String) -> Select<Entity> {
        Self::find().filter(Column::RoleId.eq(role_id))
            .filter(Column::Key.eq(permission))
    }

    pub fn find_by_role(role_id: i64) -> Select<Entity> {
        Self::find().filter(Column::RoleId.eq(role_id))
    }

    pub fn delete_by_role_id(role_id: i64) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::RoleId.eq(role_id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
