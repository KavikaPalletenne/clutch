use actix_web::{web, get, Responder, HttpRequest, HttpResponse};
use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::search::SearchResults;
use mongodb::Database;
use crate::models::Resource;

#[get("/api/search/{group_id}/{term}")]
pub async fn search(
    database: web::Data<Database>,
    index: web::Data<Index>, // Meilisearch Index
    req: HttpRequest,
) -> impl Responder {

    let group_id = req.match_info().get("group_id").unwrap().to_string();
    let search_term = req.match_info().get("term").unwrap().to_string();

    let results: SearchResults<Resource> = index.search()
        .with_query(&search_term)
        .with_filter(&*format!("group_id = {}", group_id))
        .execute()
        .await
        .unwrap();
    // let hits: Vec<Resource> = results.hits.into();

    // println!("Results: {:?}", results.hits);

    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .body(format!("Results {:?}", results))//serde_json::to_string::<Vec<Resource>>(&hits).unwrap())
}
