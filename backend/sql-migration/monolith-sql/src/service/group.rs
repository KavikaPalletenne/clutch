use anyhow::{Result, bail};
use actix_web::web::Data;

use entity::group;
use entity::group_user;
use nanoid::nanoid;
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, DeleteResult};
use crate::service::hashing::hash;
use crate::errors::MyDbError;
use crate::models::{Group, NewGroupForm};

/// Create new group from form.
/// Returns created user's id.
pub async fn create(
    group: NewGroupForm,
    creator: String, // user_id of creator
    conn: &Data<DatabaseConnection>
) -> Result<String> {
    let group_id = nanoid!().to_string();
    group::ActiveModel{
        id: Set(group_id.clone()),
        name: Set(group.name),
        description: Set(group.description),
        discord_id: Set(group.discord_id),
        ..Default::default()
    }.insert(conn.get_ref())
        .await
        .expect("Could not insert group");

    // TODO: Join creator to group

    Ok(group_id)
}

/// Get group by id.
pub async fn read(
    group_id: String,
    conn: &Data<DatabaseConnection>
) -> Result<Group> {
    let response: Option<group::Model> = group::Entity::find_by_id(group_id.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(g) = response {
        return Ok(Group {
            id: g.id,
            name: g.name,
            description: g.description,
            discord_id: g.discord_id,
        });
    }

    bail!(MyDbError::NoSuchRow { id: group_id })
}

pub async fn update() {
    // TODO: Implement this.
}

/// Delete group by id.
pub async fn delete(
    group_id: String,
    conn: &Data<DatabaseConnection>
) -> Result<()> {
    let res: DeleteResult = group::Entity::delete_by_id(group_id.clone())
        .exec(conn.get_ref())
        .await?;

    if res.rows_affected == 0 {
        return bail!(MyDbError::NoSuchRow { id: group_id });
    }

    Ok(())
}

///////////////////////
// Utility Functions //
///////////////////////
pub async fn join_group(
    group_id: String,
    user_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<()> {
    group_user::ActiveModel {
        user_id: Set(user_id),
        group_id: Set(group_id),
        ..Default::default()
    }.insert(conn.get_ref())
        .await
        .expect("Could not insert group_user");

    Ok(())
}

pub async fn leave_group(
    user_id: String,
    group_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<()> {
    let res: DeleteResult = group_user::Entity::delete_many()
        .filter(group_user::Column::UserId.contains(user_id.as_str()))
        .filter(group_user::Column::GroupId.contains(group_id.clone().as_str()))
        .exec(conn.get_ref())
        .await?;

    if res.rows_affected == 0 {
        return bail!(MyDbError::NoSuchRow { id: group_id.to_string() });
    }

    Ok(())
}
