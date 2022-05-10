use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::DeleteMany;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: String,  // owner
    pub group_id: String, // group it belongs to
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub subject: String,
    // pub tags: Option<Vec<String>>, // Tags are optional
    // pub files: Option<Vec<FileReference>>, // URL to the data (stored on server or on something like AWS S3)
    pub last_edited_at: DateTime<Utc>,
}

impl Entity {
    pub fn find_by_id(id: i64) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_group_id(group_id: String) -> Select<Entity> {
        Self::find().filter(Column::GroupId.eq(group_id))
    }

    pub fn delete_by_id(id: i64) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::file_reference::Entity")]
    FileReference,
    #[sea_orm(has_many = "super::tag::Entity")]
    Tag,
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id"
    )]
    Group,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::file_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileReference.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
