use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub public_id: Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub disabled_at: Option<NaiveDateTime>,
}

impl NewUser {
    pub fn new(public_id: Uuid, fullname: String, email: String, password: String) -> Self {
        let now = Utc::now().naive_utc();
        NewUser {
            public_id,
            fullname,
            email,
            password,
            created_at: now,
            updated_at: now,
            disabled_at: None,
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub public_id: Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub disabled_at: Option<chrono::NaiveDateTime>,
}
