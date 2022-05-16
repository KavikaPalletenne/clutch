use actix_web::web::Data;
use anyhow::{bail, Result};

use crate::errors::MyDbError;
use crate::models::{AuthUser, NewUserForm, UpdateUserForm, User};
use crate::service::hashing::hash;
use entity::group;
use entity::user;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set};

/// Create new user from form.
/// Returns created user's id.
pub async fn create(user: NewUserForm, conn: &Data<DatabaseConnection>) -> Result<String> {
    let user_id = nanoid!().to_string();
    user::ActiveModel {
        id: Set(user_id.clone()),
        username: Set(user.username),
        email: Set(user.email),
        password: Set(hash(user.password)),
        discord_id: Set(Option::None),
        ..Default::default()
    }
    .insert(conn.get_ref())
    .await
    .expect("Could not insert user");

    Ok(user_id)
}

/// Get user by id.
pub async fn read(user_id: String, conn: &Data<DatabaseConnection>) -> Result<User> {
    let response: Option<user::Model> = user::Entity::find_by_id(user_id.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(u) = response {
        return Ok(User {
            id: u.id,
            username: u.username,
            email: u.email,
            discord_id: u.discord_id,
        });
    }

    bail!(MyDbError::NoSuchRow { id: user_id })
}

/// Update username and email of given user.
pub async fn update(
    user_id: String,
    data: UpdateUserForm,
    conn: &Data<DatabaseConnection>,
) -> Result<()> {
    let response: Option<user::Model> = user::Entity::find_by_id(user_id.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(u) = response {
        let mut u: user::ActiveModel = u.into();

        if data.username.ne(&u.username.clone().unwrap()) {
            u.username = Set(data.username.clone());
        }
        if data.email.ne(&u.email.clone().unwrap()) {
            u.email = Set(data.email.clone());
        }

        let new: user::Model = u.update(conn.get_ref()).await?;
        if new.username.ne(&data.username) || new.email.ne(&data.email) {
            return bail!(MyDbError::BadUpdate {
                id: user_id,
                table_name: "users".to_string()
            });
        }

        return Ok(());
    }

    bail!(MyDbError::NoSuchRow { id: user_id })
}

/// Delete user by id.
pub async fn delete(user_id: String, conn: &Data<DatabaseConnection>) -> Result<()> {
    let res: DeleteResult = user::Entity::delete_by_id(user_id.clone())
        .exec(conn.get_ref())
        .await?;

    if res.rows_affected == 0 {
        return bail!(MyDbError::NoSuchRow { id: user_id });
    }

    Ok(())
}

///////////////////////
// Utility Functions //
///////////////////////
pub async fn username_exists(username: String, conn: &Data<DatabaseConnection>) -> Result<bool> {
    let res: Option<user::Model> = user::Entity::find_by_username(username.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(user) = res {
        return Ok(true);
    }

    Ok(false)
}

pub async fn email_exists(email: String, conn: &Data<DatabaseConnection>) -> Result<bool> {
    let res: Option<user::Model> = user::Entity::find_by_email(email.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(user) = res {
        return Ok(true);
    }

    Ok(false)
}

pub async fn get_by_email(email: String, conn: &Data<DatabaseConnection>) -> Result<AuthUser> {
    let res: Option<user::Model> = user::Entity::find_by_email(email.clone())
        .one(conn.get_ref())
        .await?;

    if let Some(user) = res {
        return Ok(AuthUser {
            id: user.id,
            username: user.username,
            email: user.email,
            password: user.password,
            discord_id: user.discord_id
        });
    }

    bail!(MyDbError::NoSuchRow { id: email })
}
