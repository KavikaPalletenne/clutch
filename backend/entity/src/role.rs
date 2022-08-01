use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::DeleteMany;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
    pub group_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::group::Entity",
    from = "Column::GroupId",
    to = "super::group::Column::Id"
    )]
    Group,
    #[sea_orm(has_many = "super::role_permission::Entity")]
    RolePermission,
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRole,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermission.def()
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl Entity {
    pub fn find_by_group(group_id: String) -> Select<Entity> {
        Self::find().filter(Column::GroupId.eq(group_id))
    }

    pub fn delete_by_id(id: i64) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
