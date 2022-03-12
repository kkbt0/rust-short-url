use crate::schema::url_table;
use diesel::Insertable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Url {
    pub id: i32,
    pub redirect_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "url_table"]
pub struct UrlInsert {
    pub redirect_url: String,
}
