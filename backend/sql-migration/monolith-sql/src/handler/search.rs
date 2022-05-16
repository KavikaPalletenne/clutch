use crate::models::Resource;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::search::SearchResults;

#[get("/api/search/{group_id}/{term}")]
pub async fn search(
    index: web::Data<Index>, // Meilisearch Index
    req: HttpRequest,
) -> impl Responder {
    let group_id = req.match_info().get("group_id").unwrap().to_string();
    let search_term = req.match_info().get("term").unwrap().to_string();

    let results: SearchResults<Resource> = index
        .search()
        .with_query(&search_term)
        .with_filter(&*format!("group_id = {}", group_id))
        .execute()
        .await
        .unwrap();

    let mut resources = Vec::<Resource>::new();

    for hit in results.hits.iter() {
        resources.push(hit.result.clone());
    }

    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(serde_json::to_string::<Vec<Resource>>(&resources).unwrap()) //serde_json::to_string::<Vec<Resource>>(&hits).unwrap())
}

/// Prevents blank searches from front-end returning a 404 not found code.
#[get("/api/search/{group_id}/")]
pub async fn search_blank(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(serde_json::to_string::<Vec<Resource>>(&Vec::<Resource>::new()).unwrap())
    //serde_json::to_string::<Vec<Resource>>(&hits).unwrap())
}
