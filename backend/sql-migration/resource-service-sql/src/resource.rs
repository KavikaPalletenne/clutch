use actix_web::{HttpResponse, Responder};

use entity::resource;
use entity::resource::Entity as Resource;
use entity::file_reference;
use entity::file_reference::Entity as FileReference;
use entity::tag;
use entity::tag::Entity as Tag;
use entity::sea_orm;
use sea_orm::Set;
use sea_orm::ActiveModelTrait;


pub async fn new() -> impl Responder {
    let conn = sea_orm::Database::connect("postgres://jcgvqsxa:lk0y4RIhtAFb4hu87EGSRxCnD_EDeBo7@rosie.db.elephantsql.com/jcgvqsxa")
        .await.unwrap();

    tag::ActiveModel {
        id: Set(123),
        text: Set("Test Tag".to_string()),
        resource_id: Set("abcd1234".to_string()),
        ..Default::default()
    }
    .insert(&conn)
    .await
    .expect("Could not insert Tag");

    HttpResponse::Ok().finish()
}
