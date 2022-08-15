use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub user_id: String,
    pub role_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
    belongs_to = "super::role::Entity",
    from = "Column::RoleId",
    to = "super::role::Column::Id"
    )]
    Role,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Entity {
    pub fn find_by_user_and_role_id(user_id: String, role_id: i64) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(user_id))
            .filter(Column::RoleId.eq(role_id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
