table! {
    objectscrips (id) {
        id -> Int4,
        scrip -> Int4,
        stage -> Varchar,
        objectid -> Int4,
        sortorder -> Int4,
        creator -> Int4,
        created -> Nullable<Timestamp>,
        lastupdatedby -> Int4,
        lastupdated -> Nullable<Timestamp>,
    }
}

table! {
    queues (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        correspondaddress -> Nullable<Varchar>,
        commentaddress -> Nullable<Varchar>,
        creator -> Int4,
        created -> Nullable<Timestamp>,
        lastupdatedby -> Int4,
        lastupdated -> Nullable<Timestamp>,
        disabled -> Int4,
        subjecttag -> Nullable<Varchar>,
        lifecycle -> Nullable<Varchar>,
        sortorder -> Int4,
        sladisabled -> Int4,
    }
}

table! {
    scripactions (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        execmodule -> Varchar,
        argument -> Nullable<Varchar>,
        creator -> Int4,
        created -> Nullable<Timestamp>,
        lastupdatedby -> Int4,
        lastupdated -> Nullable<Timestamp>,
    }
}

table! {
    scripconditions (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        execmodule -> Varchar,
        argument -> Nullable<Varchar>,
        applicabletranstypes -> Nullable<Varchar>,
        creator -> Int4,
        created -> Nullable<Timestamp>,
        lastupdatedby -> Int4,
        lastupdated -> Nullable<Timestamp>,
    }
}

table! {
    scrips (id) {
        id -> Int4,
        description -> Varchar,
        scripcondition -> Int4,
        scripaction -> Int4,
        customisapplicablecode -> Nullable<Text>,
        custompreparecode -> Nullable<Text>,
        customcommitcode -> Nullable<Text>,
        template -> Varchar,
        creator -> Int4,
        created -> Nullable<Timestamp>,
        lastupdatedby -> Int4,
        lastupdated -> Nullable<Timestamp>,
        disabled -> Int2,
    }
}

table! {
    templates (id) {
        id -> Int4,
        queue -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        content -> Nullable<Text>,
        lastupdated -> Nullable<Timestamp>,
        lastupdatedby -> Int4,
        creator -> Int4,
        created -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    objectscrips,
    queues,
    scripactions,
    scripconditions,
    scrips,
    templates,
);
