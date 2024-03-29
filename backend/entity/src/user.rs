use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::DeleteMany;

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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
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

    pub fn find_by_discord_id(discord_id: String) -> Select<Entity> {
        Self::find().filter(Column::DiscordId.eq(discord_id))
    }

    pub fn find_by_email(email: String) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn delete_by_id(id: String) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
