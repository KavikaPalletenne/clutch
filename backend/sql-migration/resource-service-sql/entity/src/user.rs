use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String, // Hashed (argon2 hashing)
    pub discord_id: Option<String>,
}

pub enum Relation {
    #[sea_orm(has_many = "super::resource::Entity")]
    Resource,
    #[sea_orm(has_many = "super::group_user::Entity")]
    GroupUser,
}

impl Related<super::resource::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Resource.def()
    }
}

impl Related<super::group_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupUser.def()
    }
}

impl Entity {
    pub fn find_by_id(id: String) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_username(username: String) -> Select<Entity> {
        Self::find().filter(Column::Username.eq(username))
    }

    pub fn find_by_discord_id(group_id: String) -> Select<Entity> {
        Self::find().filter(Column::DiscordId.eq(group_id))
    }

    pub fn delete_by_id(id: String) -> Select<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
