use actix_web::web::Data;
use anyhow::{bail, Result};
use chrono::{Duration, Utc};

use crate::errors::MyDbError;
use crate::models::{GroupResponse, NewGroupForm};
use crate::service::role::Role;
use entity::group;
use entity::group_invite;
use entity::group_user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    QueryFilter, Set,
};
use crate::service::id::generate_alphanumeric_nanoid;

/// Create new group from form.
/// Returns created group's id.
pub async fn create(
    group: NewGroupForm,
    creator_id: String, // user_id of creator
    conn: &Data<DatabaseConnection>,
) -> Result<String> {
    // TODO: Change this to get nicer ids - see invite code generation with custom alphabet
    let group_id = generate_alphanumeric_nanoid(24);
    group::ActiveModel {
        id: Set(group_id.clone()),
        name: Set(group.name),
        description: Set(group.description),
        discord_id: Set(group.discord_id),
        private: Set(group.private),
        ..Default::default()
    }
    .insert(conn.get_ref())
    .await
    .expect("Could not insert group");

    add_to_group(group_id.clone(), creator_id.clone(), conn)
        .await
        .expect("Error adding creator to group");

    // Add creator owner
    let mut owner_permissions = Vec::<String>::new();
    owner_permissions.push("owner".to_string());
    let role_id = Role::create(
        "owner".to_string(),
        group_id.clone(),
        owner_permissions,
        conn,
    )
    .await
    .expect("Error creating role \"owner\"");
    Role::assign_role(creator_id, role_id, conn)
        .await
        .expect("Error assigning creator to role \"owner\"");

    Ok(group_id)
}

/// Get group by id.
pub async fn read(group_id: String, conn: &Data<DatabaseConnection>) -> Result<GroupResponse> {
    let response: Vec<(group::Model, Vec<group_user::Model>)> =
        group::Entity::find_by_id(group_id.clone())
            .find_with_related(group_user::Entity)
            .all(conn.get_ref())
            .await?;

    if response.len() == 0 {
        bail!(MyDbError::NoSuchRow {
            id: group_id.to_string()
        });
    }

    let (g, u) = response[0].clone();
    let mut users = Vec::<String>::new();
    let mut administrators = Vec::<String>::new();
    for user in u {
        let is_admin_result = user_is_admin(group_id.clone(), user.user_id.clone(), &conn).await;
        if let Ok(is_admin) = is_admin_result {
            if is_admin {
                administrators.push(user.user_id);
                continue;
            }
        }
        users.push(user.user_id);
    }
    Ok(GroupResponse {
        id: g.id,
        name: g.name,
        description: g.description,
        discord_link: g.discord_id,
        private: g.private,
        members: users,
        administrators: administrators, // TODO: Make roles in group_user table and return admins
    })
}

// pub async fn update() {
//     todo!()
// }

/// Delete group by id.
// pub async fn delete(group_id: String, conn: &Data<DatabaseConnection>) -> Result<()> {
//     let res: DeleteResult = group::Entity::delete_by_id(group_id.clone())
//         .exec(conn.get_ref())
//         .await?;
//
//     if res.rows_affected == 0 {
//         bail!(MyDbError::NoSuchRow { id: group_id });
//     }
//
//     Ok(())
// }

///////////////////////
// Utility Functions //
///////////////////////
pub async fn generate_invite_code(
    group_id: String,
    creator_id: String,
    expiry_hours: i64, // Number of hours from now
    conn: &Data<DatabaseConnection>,
) -> Result<String> {
    let mut code = generate_alphanumeric_nanoid(10);

    while code_exists(code.clone(), &conn).await? {
        code = generate_alphanumeric_nanoid(10);
    }

    let res: Result<entity::group_invite::Model, DbErr> = group_invite::ActiveModel {
        group_id: Set(group_id),
        code: Set(code.clone()),
        expiry: Set(Utc::now()
            .checked_add_signed(Duration::hours(expiry_hours))
            .unwrap()),
        uses: Set(0),
        creator: Set(creator_id),
        ..Default::default()
    }
    .insert(conn.get_ref())
    .await;

    if let Ok(_insert_result) = res {
        return Ok(code);
    }

    bail!(MyDbError::BadInsert {
        table_name: "group_invites".to_string()
    })
}

pub async fn code_exists(code: String, conn: &Data<DatabaseConnection>) -> Result<bool> {
    let res: Option<group_invite::Model> = group_invite::Entity::find_by_code(code.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(_) = res {
        return Ok(true);
    }
    Ok(false)
}

pub async fn get_invite_code_group(
    code: String,
    conn: &Data<DatabaseConnection>,
) -> Result<String> {
    let res: Option<group_invite::Model> = group_invite::Entity::find_by_code(code.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(invite) = res {
        return Ok(invite.group_id);
    }

    bail!(MyDbError::NoSuchRow { id: code });
}

pub async fn add_to_group(
    group_id: String,
    user_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<()> {
    group_user::ActiveModel {
        user_id: Set(user_id),
        group_id: Set(group_id),
        ..Default::default()
    }
    .insert(conn.get_ref())
    .await
    .expect("Could not insert group_user");

    Ok(())
}

pub async fn join_group(
    code: String,
    user_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<()> {
    let res: Option<group_invite::Model> = group_invite::Entity::find_by_code(code.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(invite) = res {
        // Check if not expired
        if invite.expiry.timestamp() > Utc::now().timestamp() {
            group_user::ActiveModel {
                user_id: Set(user_id),
                group_id: Set(invite.group_id),
                ..Default::default()
            }
            .insert(conn.get_ref())
            .await
            .expect("Could not insert group_user");

            return Ok(());
        }
    }

    bail!(MyDbError::NoSuchRow { id: code });
}

// pub async fn add_group_admin(
//     group_id: String,
//     user_id: String,
//     conn: &Data<DatabaseConnection>,
// ) -> Result<()> {
//     group
// }

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
        bail!(MyDbError::NoSuchRow {
            id: group_id.to_string()
        });
    }

    Ok(())
}

/// Checks if a given user is in the group.
/// Returns true if they are, otherwise returns false.
pub async fn user_in_group(
    user_id: String,
    group_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<bool> {
    let res: Option<group_user::Model> = group_user::Entity::find()
        .filter(group_user::Column::GroupId.contains(&group_id.clone()))
        .filter(group_user::Column::UserId.contains(&user_id.clone()))
        .one(conn.get_ref())
        .await?;

    if let Some(_record) = res {
        return Ok(true);
    }

    Ok(false)
}

pub async fn get_user_groups(
    user_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<Vec<String>> {
    let res: Vec<group_user::Model> = group_user::Entity::find()
        .filter(group_user::Column::UserId.contains(&user_id.clone()))
        .all(conn.get_ref())
        .await?;

    let mut response = Vec::<String>::new();
    for i in res {
        response.push(i.group_id);
    }

    Ok(response)
}

pub async fn user_is_admin(
    group_id: String,
    user_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<bool> {
    let user_permissions = Role::get_user_permissions(group_id, user_id, &conn)
        .await
        .expect("Error getting user permissions");

    Ok(user_permissions.contains(&"administrator".to_string()) || user_permissions.contains(&"owner".to_string()))
}
