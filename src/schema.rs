// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    mvcc_testing (id) {
        id -> Int4,
        number -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        user_uuid -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        password -> Text,
        profile_picture_url -> Nullable<Text>,
        date_of_birth -> Nullable<Date>,
        gender -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_type -> Nullable<Text>,
        is_email_verified -> Nullable<Bool>,
        is_phone_verified -> Nullable<Bool>,
        is_admin -> Nullable<Bool>,
        status -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    mvcc_testing,
    users,
);
