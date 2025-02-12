// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Text,
        major -> Varchar,
        date_of_birth -> Text,
        pronouns -> Varchar,
        gender -> Varchar,
        degree_type -> Text,
        college_year -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
