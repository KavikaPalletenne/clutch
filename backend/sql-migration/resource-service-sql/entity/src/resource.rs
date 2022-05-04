use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::file_reference::Entity")]
    FileReference,
    #[sea_orm(has_many = "super::tag::Entity")]
    Tag,
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

impl ActiveModelBehavior for ActiveModel {}
