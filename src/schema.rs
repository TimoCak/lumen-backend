// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        thread_id -> Int4,
        answer_id -> Nullable<Int4>,
        author -> Varchar,
        created_at -> Nullable<Timestamp>,
        title -> Varchar,
        text -> Varchar,
        likes -> Nullable<Int4>,
        dislikes -> Nullable<Int4>,
    }
}

diesel::table! {
    threads (id) {
        id -> Int4,
        author -> Varchar,
        created_at -> Nullable<Timestamp>,
        title -> Varchar,
        text -> Varchar,
        likes -> Nullable<Int4>,
        dislikes -> Nullable<Int4>,
        categories -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    threads,
    users,
);
