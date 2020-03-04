#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use schema::*;
use std::collections::HashMap;

type DubbelingMap = HashMap<(i32, i32, String), i32>;
type RewriteMap = HashMap<i32, i32>;
type GroupedScrips = Vec<(Scrip, Vec<ObjectScrip>)>;

fn main() {
    dotenv::dotenv().ok();
    use std::env;
    let database_url = env::var("DATABASE_URL").expect("it to be set");
    let conn = PgConnection::establish(&database_url).expect("yeah!");

    if false {
        ontdubbelen(&conn);
    }

    if false {
        update_descriptions(&conn);
    }
}

fn grouped_scrips(conn: &PgConnection) -> GroupedScrips {
    let scrips = scrips::table
        .filter(scrips::disabled.eq(0))
        .order(scrips::id)
        .load::<Scrip>(conn)
        .expect("many rows");

    let objscrips = ObjectScrip::belonging_to(&scrips)
        .load::<ObjectScrip>(conn)
        .expect("Loading queues that have me");

    let grouped: Vec<Vec<ObjectScrip>> = objscrips.grouped_by(&scrips);

    scrips.into_iter().zip(grouped).collect::<Vec<_>>()
}

fn scripactions(conn: &PgConnection) -> HashMap<i32, String> {
    let mut r: HashMap<i32, String> = HashMap::new();
    let scripactions = scripactions::table.load::<ScripAction>(conn).expect("some");
    for a in scripactions {
        r.insert(a.id, a.name);
    }
    r
}

fn scripconditions(conn: &PgConnection) -> HashMap<i32, String> {
    let mut r: HashMap<i32, String> = HashMap::new();
    let rows = scripconditions::table
        .load::<ScripCondition>(conn)
        .expect("some");
    for a in rows {
        r.insert(a.id, a.name);
    }
    r
}

fn update_descriptions(conn: &PgConnection) {
    let mut when = scripconditions(conn);
    let mut what = scripactions(conn);

    let mut generate_name = |scrip: &Scrip| -> Option<String> {
        let condition = when.get(&scrip.scripcondition)?;
        //.or_insert(format!("<condition {}>", scrip.scripcondition));
        let action = what.get(&scrip.scripaction)?;
        //.or_insert(format!("<action {}>", scrip.scripaction));
        Some(format!("{} {}", condition, action))
    };

    conn.transaction::<_, diesel::result::Error, _>(|| {
        for (scrip, objs) in grouped_scrips(conn) {
            if objs.len() > 0 && scrip.description.contains("Imported from RT") {
                if let Some(generated) = generate_name(&scrip) {
                    println!(
                        "--> suggesting rename\nFROM: {}\nTO  : {}\n===",
                        &scrip.description, generated
                    );
                    /*
                    diesel::update(scrips::table.find(scrip.id))
                        .set(scrips::description.eq(generated))
                        .execute(conn)
                        .expect("update to succeed");
                    */
                }
            }
        }
        Ok(())
    })
    .ok();
}

// map scripaction+scripcondition+template to distinct (lowest) primary
// key.
// NB: If scripaction = custom, we don't
#[allow(dead_code)]
fn ontdubbelen(conn: &PgConnection) {
    let scrips = scrips::table
        .filter(scrips::disabled.eq(0))
        .order(scrips::id)
        .load::<Scrip>(conn)
        .expect("many rows");

    let mut r: DubbelingMap = HashMap::new();
    let mut rewrite: RewriteMap = HashMap::new();
    for s in scrips {
        if s.scripaction == 15 || s.scripcondition == 10 {
            // userdefined
            continue;
        }
        if s.customisapplicablecode.is_some()
            || s.custompreparecode.is_some()
            || s.customcommitcode.is_some()
        {
            continue;
        }
        let key = (s.scripcondition, s.scripaction, s.template.clone());
        if let Some(current) = r.get(&key) {
            rewrite.insert(s.id, *current);
            continue;
        }
        r.insert(key, s.id);
    }

    if rewrite.len() > 0 {
        conn.transaction::<_, diesel::result::Error, _>(|| {
            for (oldid, newid) in rewrite {
                let target = objectscrips::table.filter(objectscrips::scrip.eq(oldid));
                match target.count().get_result::<i64>(conn) {
                    Ok(c) if c > 0 => {
                        let q = diesel::update(target).set(objectscrips::scrip.eq(newid));
                        println!("{}", diesel::debug_query::<Pg, _>(&q));
                        q.execute(conn).expect("update to succeed");
                    }
                    _ => {}
                }
            }
            Ok(()) // == COMMIT
        })
        .ok();
    }
}
