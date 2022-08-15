use actix_web::web::Data;
use anyhow::{bail, Result};
use chrono::Utc;

use crate::errors::MyDbError;
use crate::models::{FileReference, Resource, ResourceForm};
use crate::service::group;
use crate::service::id::{generate_alphanumeric_nanoid, generate_snowflake};
use entity::file_reference;
use entity::resource;
use entity::sea_orm;
use entity::tag;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use sea_orm::{DatabaseConnection, DeleteResult, Set};

/// Inserts a new resource in the DB, along with files and tags.
/// Returns created resource's id (nanoid)
pub async fn create(resource: ResourceForm, conn: &Data<DatabaseConnection>) -> Result<String> {
    // let conn = sea_orm::Database::connect("postgres://jcgvqsxa:lk0y4RIhtAFb4hu87EGSRxCnD_EDeBo7@rosie.db.elephantsql.com/jcgvqsxa")
    //     .await.unwrap();

    let resource_id = generate_alphanumeric_nanoid(22);
    resource::ActiveModel {
        id: Set(resource_id.clone()),
        user_id: Set(resource.user_id),
        group_id: Set(resource.group_id),
        title: Set(resource.title),
        description: Set(resource.description),
        subject: Set(resource.subject),
        last_edited_at: Set(Utc::now()),
        ..Default::default()
    }
    .insert(conn.get_ref())
    .await
    .expect("Could not insert resource");

    // Insert files into DB
    if let Some(form_resources) = resource.files {
        let mut file_references = Vec::<file_reference::ActiveModel>::new();
        for f in form_resources {
            file_references.push(file_reference::ActiveModel {
                name: Set(f.name),
                size: Set(f.size),
                resource_id: Set(resource_id.clone()),
                ..Default::default()
            })
        }
        file_reference::Entity::insert_many(file_references)
            .exec(conn.get_ref())
            .await
            .expect(
                format!(
                    "Could not insert file references for resource: {}",
                    resource_id.clone()
                )
                .as_str(),
            );
    }

    // Insert tags into DB
    if let Some(form_tags) = resource.tags {
        if form_tags.clone().len() != 0 {
            let mut tags = Vec::<tag::ActiveModel>::new();
            for t in form_tags {
                tags.push(tag::ActiveModel {
                    text: Set(t),
                    resource_id: Set(resource_id.clone()),
                    ..Default::default()
                })
            }
            tag::Entity::insert_many(tags)
                .exec(conn.get_ref())
                .await
                .expect(
                    format!(
                        "Could not insert tags for resource: {}",
                        resource_id.clone()
                    )
                    .as_str(),
                );
        }
    }

    Ok(resource_id)
}

/// Read a resource by id.
pub async fn read(resource_id: String, conn: &Data<DatabaseConnection>) -> Result<Resource> {
    //TODO: Use custom joins to also include tags - https://www.sea-ql.org/SeaORM/docs/advanced-query/custom-joins
    let mut response: Vec<(resource::Model, Vec<file_reference::Model>)> =
        resource::Entity::find_by_id(resource_id.clone())
            .find_with_related(file_reference::Entity)
            .all(conn.get_ref())
            .await?;

    if response.len() == 0 {
        bail!(MyDbError::NoSuchRow {
            id: resource_id.to_string()
        });
    }

    let (resource, files) = response.remove(0);
    let mut res_files = Vec::<FileReference>::new();
    for f in files {
        res_files.push(FileReference {
            name: f.name,
            size: f.size,
        });
    }

    Ok(Resource {
        id: resource.id,
        user_id: resource.user_id,
        group_id: resource.group_id,
        title: resource.title,
        description: resource.description,
        subject: resource.subject,
        tags: Option::from(Vec::<String>::new()),
        files: Option::from(res_files),
        last_edited_at: resource.last_edited_at,
    })
}

/// Updates resource by id.
/// NOT IMPLEMENTED YET
// pub async fn update(// resource_id: String,
//     // data: ResourceForm,
//     // conn: &Data<DatabaseConnection>,
// ) -> Result<()> {
//     // TODO: Make an update feature
//     Ok(())
// }

/// Deletes a resource by id.
pub async fn delete(resource_id: String, conn: &Data<DatabaseConnection>) -> Result<()> {
    let res: DeleteResult = resource::Entity::delete_by_id(resource_id.clone())
        .exec(conn.get_ref())
        .await?;

    if res.rows_affected == 0 {
        bail!(MyDbError::NoSuchRow {
            id: resource_id
        });
    }

    Ok(())
}

///////////////////////
// Utility Functions //
///////////////////////
pub async fn get_resource_by_group(
    group_id: String,
    per_page: i32,
    page_num: i32,
    conn: &Data<DatabaseConnection>,
) -> Result<Vec<Resource>> {
    let response: Vec<resource::Model> = resource::Entity::find()
        .filter(resource::Column::GroupId.eq(group_id.as_str()))
        .order_by_desc(resource::Column::LastEditedAt)
        .paginate(conn.get_ref(), per_page.try_into().unwrap())
        .fetch_page(page_num.try_into().unwrap()) //TODO: Find out how to paginate and join in one go to reduce DB roundtrips
        .await?;

    if response.len() == 0 {
        bail!(MyDbError::NoSuchRow {
            id: group_id.to_string()
        });
    }

    let mut response_vector =
        Vec::<(resource::Model, Vec<file_reference::Model>, Vec<tag::Model>)>::new();

    for r in response {
        let files: Vec<file_reference::Model> = r
            .find_related(file_reference::Entity)
            .all(conn.get_ref())
            .await?;

        let tags: Vec<tag::Model> = r.find_related(tag::Entity).all(conn.get_ref()).await?;

        response_vector.push((r, files, tags));
    }

    let mut resources = Vec::<Resource>::new();
    for item in response_vector {
        let (resource, files, tags) = item;
        let mut res_files = Vec::<FileReference>::new();
        for f in files {
            res_files.push(FileReference {
                name: f.name,
                size: f.size,
            });
        }

        let mut res_tags = Vec::<String>::new();
        for t in tags {
            res_tags.push(t.text);
        }

        resources.push(Resource {
            id: resource.id,
            user_id: resource.user_id,
            group_id: resource.group_id,
            title: resource.title,
            description: resource.description,
            subject: resource.subject,
            tags: Option::from(res_tags),
            files: Option::from(res_files),
            last_edited_at: resource.last_edited_at,
        })
    }

    Ok(resources)
}

// pub async fn get_resource_by_user(
//     user_id: String,
//     // per_page: i32,
//     // page_num: i32,
//     conn: &Data<DatabaseConnection>,
// ) -> Result<Vec<Resource>> {
//     let mut response: Vec<(resource::Model, Vec<file_reference::Model>)> = resource::Entity::find()
//         .filter(resource::Column::UserId.contains(user_id.as_str()))
//         .find_with_related(file_reference::Entity)
//         .all(conn.get_ref())
//         // .paginate(conn.get_ref(), per_page.try_into().unwrap())
//         // .fetch_page(page_num.try_into().unwrap()) //TODO: Find out how to paginate and join
//         .await?;
//
//     if response.len() == 0 {
//         bail!(MyDbError::NoSuchRow {
//             id: user_id.to_string()
//         });
//     }
//
//     let mut resources = Vec::<Resource>::new();
//     for i in 0..response.len() {
//         let (resource, files) = response.remove(i);
//         let mut res_files = Vec::<FileReference>::new();
//         for f in files {
//             res_files.push(FileReference {
//                 name: f.name,
//                 size: f.size,
//             });
//         }
//
//         resources.push(Resource {
//             id: resource.id.to_string(),
//             user_id: resource.user_id,
//             group_id: resource.group_id,
//             title: resource.title,
//             description: resource.description,
//             subject: resource.subject,
//             tags: Option::from(Vec::<String>::new()),
//             files: Option::from(res_files),
//             last_edited_at: resource.last_edited_at,
//         })
//     }
//
//     Ok(resources)
// }

pub async fn user_can_view_resource(
    user_id: String,
    resource_id: String,
    conn: &Data<DatabaseConnection>,
) -> Result<bool> {
    let res: Option<resource::Model> = resource::Entity::find_by_id(resource_id.clone())
        .filter(resource::Column::UserId.contains(&user_id.clone()))
        .one(conn.get_ref())
        .await?;

    if let Some(record) = res {
        if group::user_in_group(user_id.clone(), record.group_id, conn).await? {
            return Ok(true);
        }
    }

    Ok(false)
}
