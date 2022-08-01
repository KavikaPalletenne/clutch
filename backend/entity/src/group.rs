use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::DeleteMany;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub description: String,
    pub discord_id: String,
    pub private: bool, // If the group is private, or open to non-members
}

impl Entity {
    pub fn find_by_id(id: String) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_discord_id(group_id: String) -> Select<Entity> {
        Self::find().filter(Column::DiscordId.eq(group_id))
    }

    pub fn delete_by_id(id: String) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::resource::Entity")]
    Resource,
    #[sea_orm(has_many = "super::group_user::Entity")]
    GroupUser,
    #[sea_orm(has_many = "super::group_invite::Entity")]
    GroupInvite
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

impl Related<super::group_invite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupInvite.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
