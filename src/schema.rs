// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    subscriptions (id) {
        id -> Int4,
        email -> Text,
        name -> Text,
        subscribed_at -> Timestamp,
    }
}
