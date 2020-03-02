use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "scrips"]
#[belongs_to(ScripAction, foreign_key = "scripaction")]
#[belongs_to(ScripCondition, foreign_key = "scripcondition")]
pub struct Scrip {
    pub id: i32,
    pub description: String,
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

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "scripactions"]
pub struct ScripAction {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub execmodule: String,
    pub argument: Option<String>,
    creator: i32,
    created: Option<NaiveDateTime>,
    lastupdatedby: i32,
    lastupdated: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "scripconditions"]
pub struct ScripCondition {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub execmodule: String,
    pub argument: Option<String>,
    pub applicabletranstypes: Option<String>,
    creator: i32,
    created: Option<NaiveDateTime>,
    lastupdatedby: i32,
    lastupdated: Option<NaiveDateTime>,
}
