use actix_web::web::Data;
use anyhow::{bail, Result};

use crate::errors::MyDbError;
use entity::{role, role_permission, user_role};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{DatabaseConnection, DbErr, DeleteResult, Set};

pub struct Role {
    pub role_id: i64,
    pub name: String,
    pub group_id: String,
    pub permissions: Vec<String>, // keys of permissions e.g. ["resource_delete", "user_ban"]
}

pub struct RoleUsers {
    pub role_id: i64,
    pub users: Vec<String>, // Array of user ids of users with that role
}

pub struct UserPermissions {
    pub group_id: String,
    pub permissions: String,
}

static PERMISSION_KEYS: [&str; 9] = [
    "owner",
    "administrator", // HAS ALL THE POWER!!!
    "role_manage", // Can add role - need to make sure the roles they create/add to users do not have more permission than they themselves do!
    "member_ban",
    "resource_delete", // Can delete resources even if they did not create them
    "member_kick",
    "group_edit",      // Can change name and description of group
    "nickname_change", // Can change own nickname
    "invite_create",   // Can create invite links
];
// TODO: Some sort of role permission integer - permission needs to be higher/lower than a value to be able to do a task

impl Role {
    pub async fn get(id: i64, conn: &Data<DatabaseConnection>) -> Result<Role> {
        let mut res: Vec<(role::Model, Vec<role_permission::Model>)> =
            role::Entity::find_by_id(id.clone())
                .find_with_related(role_permission::Entity)
                .all(conn.get_ref())
                .await?;

        if res.len() == 0 {
            bail!(MyDbError::NoSuchRow { id: id.to_string() });
        }

        let (role, permissions) = res.remove(0);
        let mut res_permissions = Vec::<String>::new();
        for p in permissions {
            res_permissions.push(p.key);
        }

        Ok(Role {
            role_id: role.id,
            name: role.name,
            group_id: role.group_id,
            permissions: res_permissions,
        })
    }

    pub async fn get_user_permissions(
        group_id: String,
        user_id: String,
        conn: &Data<DatabaseConnection>,
    ) -> Result<Vec<String>> {

        // Get all roles in the group
        let group_roles: Vec<role::Model> = role::Entity::find_by_group(group_id.clone())
            .all(conn.get_ref())
            .await?;

        // Initialise a vector to return the permissions
        let mut full_permissions = Vec::<String>::new();

        for role in group_roles {
            // Check if user has the specific role
            let possible_user_role: Option<user_role::Model> = user_role::Entity::find_by_user_and_role_id(user_id.clone(), role.id.clone())
                .one(conn.get_ref())
                .await?;

            // If the user has the specific role, continue
            if let Some(role) = possible_user_role {
                // Get all permissions of the role
                let role_permissions: Vec<role_permission::Model> =
                    role_permission::Entity::find_by_role(role.role_id)
                        .all(conn.get_ref())
                        .await?;

                // Add each permission of the role to full_permissions vector
                // if not already in it (some roles have overlapping permissions)
                for permission in role_permissions {
                    if full_permissions.contains(&permission.key) {
                        continue;
                    }
                    full_permissions.push(permission.key);
                }
            }
        }
        Ok(full_permissions)
    }

    pub async fn get_role_users(id: i64, conn: &Data<DatabaseConnection>) -> Result<RoleUsers> {
        let mut res: Vec<(role::Model, Vec<user_role::Model>)> =
            role::Entity::find_by_id(id.clone())
                .find_with_related(user_role::Entity)
                .all(conn.get_ref())
                .await?;

        if res.len() == 0 {
            bail!(MyDbError::NoSuchRow { id: id.to_string() });
        }

        let (_role, users) = res.remove(0);
        let mut res_users = Vec::<String>::new();
        for u in users {
            res_users.push(u.user_id)
        }

        Ok(RoleUsers {
            role_id: id,
            users: res_users,
        })
    }

    pub async fn create(
        name: String,
        group_id: String,
        role_permissions: Vec<String>,
        conn: &Data<DatabaseConnection>,
    ) -> Result<i64> {
        let res: Result<entity::role::Model, DbErr> = role::ActiveModel {
            name: Set(name),
            group_id: Set(group_id),
            ..Default::default()
        }
        .insert(conn.get_ref())
        .await;

        if let Ok(insert_result) = res {
            for permission in role_permissions {
                if PERMISSION_KEYS.contains(&&*permission) {
                    let res = role_permission::ActiveModel {
                        role_id: Set(insert_result.id.clone()),
                        key: Set(permission),
                        ..Default::default()
                    }
                    .insert(conn.get_ref())
                    .await;

                    // If one permission is not inserted properly, delete whole role
                    if let Err(_) = res {
                        role::Entity::delete_by_id(insert_result.id.clone())
                            .exec(conn.get_ref())
                            .await?;

                        bail!(MyDbError::BadInsert {
                            table_name: "role_permissions".to_string()
                        })
                    }
                }
            }
            return Ok(insert_result.id);
        }

        bail!(MyDbError::BadInsert {
            table_name: "roles".to_string()
        })
    }

    pub async fn delete(role_id: i64, conn: &Data<DatabaseConnection>) -> Result<()> {
        let res: DeleteResult = role::Entity::delete_by_id(role_id.clone())
            .exec(conn.get_ref())
            .await?;

        if res.rows_affected == 0 {
            bail!(MyDbError::NoSuchRow {
                id: role_id.to_string()
            })
        }

        Ok(())
    }

    ///////////////////////
    // Utility Functions //
    ///////////////////////
    pub async fn assign_role(
        user_id: String,
        role_id: i64,
        conn: &Data<DatabaseConnection>,
    ) -> Result<()> {
        let res = user_role::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            ..Default::default()
        }
        .insert(conn.get_ref())
        .await;

        if let Ok(_) = res {
            return Ok(());
        }
        bail!(MyDbError::BadInsert {
            table_name: "user_roles".to_string()
        })
    }

    pub async fn add_role_permissions(
        role_id: i64,
        role_permissions: Vec<String>,
        conn: &Data<DatabaseConnection>,
    ) -> Result<()> {
        let res: Vec<(role::Model, Vec<role_permission::Model>)> =
            role::Entity::find_by_id(role_id.clone())
                .find_with_related(role_permission::Entity)
                .all(conn.get_ref())
                .await?;

        if res.len() > 0 {
            bail!(MyDbError::NoSuchRow {
                id: role_id.to_string()
            })
        }

        // Delete current permissions
        role_permission::Entity::delete_by_role_id(role_id.clone())
            .exec(conn.get_ref())
            .await?;

        // Insert new permissions with new values
        for permission in role_permissions {
            if PERMISSION_KEYS.contains(&&*permission) {
                let res = role_permission::ActiveModel {
                    role_id: Set(role_id.clone()),
                    key: Set(permission),
                    ..Default::default()
                }
                .insert(conn.get_ref())
                .await;

                if let Err(_) = res {
                    bail!(MyDbError::BadUpdate {
                        id: role_id.to_string(),
                        table_name: "role_permissions".to_string()
                    })
                }
            }
        }

        bail!(MyDbError::NoSuchRow {
            id: role_id.to_string()
        })
    }
}
