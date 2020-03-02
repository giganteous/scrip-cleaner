#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
use schema::*;

pub mod models;
use models::*;

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

    let when = scripconditions(&conn);
    let r: Vec<(Scrip, Vec<ObjectScrip>)> = scrips.into_iter().zip(grouped).collect::<Vec<_>>();
    for (key, _value) in r {
        println!("scrip {}\ndesc: {}", key.id, key.description);
        //for q in value { println!("  on queue id {}", q.objectid); }
    }
}

use std::collections::HashMap;

fn scripactions(conn: &PgConnection) -> HashMap<i32, ScripAction> {
    let mut r: HashMap<i32, ScripAction> = HashMap::new();
    let scripactions = scripactions::table.load::<ScripAction>(conn).expect("some");
    for a in scripactions {
        r.insert(a.id, a);
    }
    r
}

fn scripconditions(conn: &PgConnection) -> HashMap<i32, ScripCondition> {
    let mut r: HashMap<i32, ScripCondition> = HashMap::new();
    let rows = scripconditions::table
        .load::<ScripCondition>(conn)
        .expect("some");
    for a in rows {
        r.insert(a.id, a);
    }
    r
}
