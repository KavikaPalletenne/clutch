use actix_web::{web, get, Responder, HttpRequest, HttpResponse};
use jsonwebtoken::DecodingKey;
use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::search::SearchResults;
use mongodb::Database;
use crate::group::check_user_in_group;
use crate::middleware::authorize;
use crate::models::Resource;

#[get("/api/search/{group_id}/{term}")]
pub async fn search(
    index: web::Data<Index>, // Meilisearch Index
    database: web::Data<Database>, // Database connection to check permissions
    decoding_key: web::Data<DecodingKey>,
    req: HttpRequest,
) -> impl Responder {

    let group_id = req.match_info().get("group_id").unwrap().to_string();
    let search_term = req.match_info().get("term").unwrap().to_string();

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req, decoding_key.get_ref()).await;


    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if !check_user_in_group(authorized.user_id.unwrap(), group_id.clone(), &database).await {
        return HttpResponse::Unauthorized().body("Logged in user does not have permission to search requested group.");
    }
    //////////////////////////////////////////////////////////////////////////


    let results: SearchResults<Resource> = index.search()
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
        .body(serde_json::to_string::<Vec<Resource>>(&resources).unwrap())//serde_json::to_string::<Vec<Resource>>(&hits).unwrap())
}

/// Prevents blank searches from front-end returning a 404 not found code.
#[get("/api/search/{group_id}/")]
pub async fn search_blank(
    _req: HttpRequest,
) -> impl Responder {

    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(serde_json::to_string::<Vec<Resource>>(&Vec::<Resource>::new()).unwrap())//serde_json::to_string::<Vec<Resource>>(&hits).unwrap())
}
