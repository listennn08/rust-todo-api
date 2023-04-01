// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        done -> Bool,
        created_by -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
        password -> Text,
        email -> Nullable<Text>,
        role -> Text,
        salt_version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(todos -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
