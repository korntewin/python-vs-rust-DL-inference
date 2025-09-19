use crate::api::apiserver::AppData;
use crate::api::dtos::Request;
use log::debug;
use ntex::web;
use ntex::web::types::{Json, Path, State};

#[web::post("/{feature_1_id}")]
pub async fn process_feature(
    path: Path<(i64,)>,
    request: Json<Request>,
    data: State<AppData>,
) -> web::HttpResponse {
    let feature_1_id = path.into_inner().0;
    debug!(
        "Feature request received for feature_1_id: {:?}",
        feature_1_id
    );
    let response = data
        .feature_service
        .process_feature(feature_1_id, request.into_inner())
        .await;
    debug!("Feature response: {:?}", response);
    match response {
        Ok(response) => web::HttpResponse::Ok().json(&response),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}
