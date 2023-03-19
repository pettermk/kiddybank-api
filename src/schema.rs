// @generated automatically by Diesel CLI.

diesel::table! {
    kids (id) {
        id -> Int4,
        name -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        ts -> Timestamp,
        change -> Float8,
        kid_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        external_id -> Text,
    }
}

diesel::joinable!(kids -> users (user_id));
diesel::joinable!(transactions -> kids (kid_id));

diesel::allow_tables_to_appear_in_same_query!(
    kids,
    transactions,
    users,
);
