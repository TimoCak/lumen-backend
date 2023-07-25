// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        author -> Varchar,
        created_at -> Nullable<Timestamp>,
        title -> Varchar,
        text -> Varchar,
        likes -> Nullable<Int4>,
        dislikes -> Nullable<Int4>,
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
    users,
);
