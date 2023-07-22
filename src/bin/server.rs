use actix_web::{
    get,
    http::StatusCode,
    web::{self, Query},
    App, HttpResponse, HttpServer,
};
use chrono::{TimeZone, Utc};
use rust::model::{read_csv_to_hashmap, Geotag, GeotagReal};
use rustc_hash::FxHasher;
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, error::Error, sync::Arc, hash::BuildHasherDefault};

type Hasher = BuildHasherDefault<FxHasher>;
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tag_map = read_csv_to_hashmap("./data/output.csv").unwrap();
    let shared_tag_map = Arc::new(tag_map);
    println!("Listening on http://localhost:{}...", PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_tag_map.clone()))
            .service(handle)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await?;

    Ok(())
}

#[derive(Deserialize)]
struct GetParameter {
    tag: String,
}

// GET http://localhost:8080/?tag=hoge で動く
#[get("/")]
async fn handle(
    params: Query<GetParameter>,
    data: web::Data<Arc<HashMap<String, Vec<Geotag>, Hasher>>>,
) -> Result<HttpResponse, actix_web::Error> {
    dbg!(&params.tag);

    let tag = data.get(&params.tag).unwrap();
    let modified_tag: Vec<GeotagReal> = tag
        .iter()
        .map(|geotag| GeotagReal {
            date: Utc
                .timestamp(geotag.date.into(), 0)
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            lat: geotag.lat,
            lon: geotag.lon,
            url: format!(
                "http://farm{}.static.flickr.com/{}.jpg",
                &geotag.url[0..1],
                &geotag.url[1..]
            ),
        })
        .collect();

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .json(json!({"tag": params.tag,  "results": modified_tag})))
}
