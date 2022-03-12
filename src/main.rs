#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod module;
mod routes;
mod schema;

use rocket_sync_db_pools::database;
use routes::*;

pub const BASE_URL: &str = "http://127.0.0.1:8000/";

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

#[database("sqlite_main")]
pub struct MainDbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDbConn::fairing())
        .mount("/", routes![index])
        .mount("/", routes![api_test_url])
        // api
        .mount("/", routes![get_url, add_url])
        .mount("/", routes![add_url_options])
        // get all test use
        .mount("/", routes![get_all])
        .attach(CORS)
}


pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Setting access control allow origin");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  
    }
}