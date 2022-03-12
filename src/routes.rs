use crate::module::*;
use crate::schema::url_table;
use crate::MainDbConn;
use crate::BASE_URL;
use diesel::{prelude::*, QueryDsl, RunQueryDsl};
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket::{get, post, options , response::Debug};

// api
// 1. get /<str>  Redirect
// 2. post        add url
// 3. get /all    get All url

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    // "欢迎使用短链接服务!".to_string()
    NamedFile::open("index.html").await.ok()
}

#[get("/<in_str>")]
pub async fn get_url(db: MainDbConn, in_str: String) -> Redirect {
    dbg!(&in_str);
    let in_id = base62::decode(in_str).unwrap();
    let ans: Option<Json<Url>> = db
        .run(move |conn| {
            url_table::table
                .filter(url_table::id.eq(in_id as i32))
                .first(conn)
        })
        .await
        .map(Json)
        .ok();
    match ans {
        Some(res) => Redirect::to(res.into_inner().redirect_url),
        _ => Redirect::to("https://www.ftls.xyz/404.html"),
    }
}

#[get("/api/<in_str>")]
pub async fn api_test_url(in_str: String) -> Value {
    // -2147483648,                         2147483647
    //  let b62 = base62::encode(abc); //u32 4294967295 u128 340282366920938463463374607431768211455
    json!({"success": base62::decode(in_str).unwrap().to_string() })
}

#[options("/")]
pub async fn add_url_options() -> Value { json!({"success":1}) }

#[post("/", format = "json", data = "<in_url>")]
pub async fn add_url(db: MainDbConn, in_url: Json<UrlInsert>) -> Value {
    let article_in = in_url.clone();
    let ans = db
        .run(move |conn| {
            diesel::insert_into(url_table::table)
                .values(&article_in)
                .execute(conn)
        })
        .await;
    let id: Option<Json<Url>> = db
        .run(move |conn| {
            url_table::table
                .filter(url_table::redirect_url.eq(in_url.into_inner().redirect_url))
                .first(conn)
        })
        .await
        .map(Json)
        .ok();
    let short_url = BASE_URL.to_owned() + &base62::encode(id.unwrap().into_inner().id as u32);
    match ans {
        Ok(res) => {
            json!({"success": res,"short_url":short_url})
        }
        _ => {
            json!({"success": 0,"short_url":short_url})
        }
    }
}

// 查 all
#[get("/all")]
pub async fn get_all(db: MainDbConn) -> Result<Json<Vec<Url>>> {
    let all = db
        .run(move |conn| url_table::table.load::<Url>(conn))
        .await?;

    Ok(Json(all))
}
