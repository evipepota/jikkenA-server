use actix_web::{
    get,
    http::StatusCode,
    web::{self, Query},
    App, HttpResponse, HttpServer,
};
use rust::model::{tesat, Geotag};
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, error::Error, sync::Arc};

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tag_map = tesat();
    println!("hash insert done!");
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
    params: Query<GetParameter>, // http://example.com?tag=hoge の hoge が入ってる
    data: web::Data<Arc<HashMap<String, Vec<Geotag>>>>,
) -> Result<HttpResponse, actix_web::Error> {
    dbg!(&params.tag);

    // 前処理データである tag.json を読み込む
    // [hint] サーバー起動時に読み込んでそれを再利用すれば良さそう
    // actix_web だと web::Data<T> を使ってデータを保持することができる
    // それをしないと多分 tag.json の load だけでタイムアウトします
    // ref: https://actix.rs/docs/application/#state

    // tag 名が一致する tag を探索する
    // [hint] これ O(1) でできそう
    
    let tag = data.get(&params.tag).unwrap();
    let modified_tag: Vec<Geotag> = tag
    .iter()
    .map(|geotag| Geotag {
        date: geotag.date.clone(),
        lat: geotag.lat,
        lon: geotag.lon,
        url: format!("http://farm9.static.flickr.com/{}.jpg", geotag.url),
    })
    .collect();

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .json(json!({"tag": params.tag,  "results": modified_tag})))
}

