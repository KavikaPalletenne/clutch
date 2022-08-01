use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "group_invites")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub group_id: String,
    pub code: String,
    pub expiry: DateTime<Utc>,
    pub uses: i64,
    pub creator: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::group::Entity",
    from = "Column::GroupId",
    to = "super::group::Column::Id"
    )]
    Group,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Entity {
    pub fn find_by_code(code: String) -> Select<Entity> {
        Self::find().filter(Column::Code.eq(code))
    }

    pub fn find_by_group_id(group_id: String) -> Select<Entity> {
        Self::find().filter(Column::GroupId.eq(group_id))
    }

    pub fn delete_by_code(code: String) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Code.eq(String))
    }
}

impl ActiveModelBehavior for ActiveModel {}
