#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
use schema::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "scrips"]
pub struct Scrip {
    pub id: i32,
    pub description: Option<String>,
    pub scripcondition: i32,
    pub scripaction: i32,
    pub customisapplicablecode: Option<String>,
    pub custompreparecode: Option<String>,
    pub customcommitcode: Option<String>,
    pub disabled: i32,
    pub template: String,
    creator: i32,
    created: Option<NaiveDateTime>,
    lastupdatedby: i32,
    lastupdated: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "objectscrips"]
#[belongs_to(Scrip, foreign_key = "scrip")]
pub struct ObjectScrip {
    pub id: i32,
    pub scrip: i32, // Scrip.id
    pub stage: String,
    pub objectid: i32, // Queue.id
    pub sortorder: i32,
    creator: i32,
    created: Option<NaiveDateTime>,
    lastupdatedby: i32,
    lastupdated: Option<NaiveDateTime>,
}

fn main() {
    dotenv::dotenv().ok();
    use std::env;
    let database_url = env::var("DATABASE_URL").expect("it to be set");
    let conn = PgConnection::establish(&database_url).expect("yeah!");

    let scrips = scrips::table
        .filter(scrips::disabled.eq(0))
        .load::<Scrip>(&conn)
        .expect("many rows");

    let objscrips = ObjectScrip::belonging_to(&scrips)
        .load::<ObjectScrip>(&conn)
        .expect("Loading queues that have me");

    let grouped: Vec<Vec<ObjectScrip>> = objscrips.grouped_by(&scrips);

    let r: Vec<(Scrip, Vec<ObjectScrip>)> = scrips.into_iter().zip(grouped).collect::<Vec<_>>();
    for (key, value) in r {
        println!(
            "scrip {} ({})",
            key.id,
            match key.description {
                Some(x) => x,
                None => "<none>".to_string(),
            }
        );
        for q in value {
            println!("  on queue id {}", q.objectid);
        }
    }
}
