// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

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
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    blocked_users (id) {
        id -> Int4,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        user_tag -> Varchar,
        #[max_length = 255]
        blocked_by -> Varchar,
        #[max_length = 255]
        blocked_by_tag -> Varchar,
        reason -> Nullable<Text>,
        created_at -> Timestamptz,
        #[max_length = 255]
        guild_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    guild_configs (id) {
        id -> Int4,
        #[max_length = 255]
        guild_id -> Varchar,
        #[max_length = 255]
        modmail_category_id -> Nullable<Varchar>,
        #[max_length = 255]
        log_channel_id -> Nullable<Varchar>,
        randomize_names -> Nullable<Bool>,
        auto_close_hours -> Nullable<Int4>,
        welcome_message -> Nullable<Text>,
        moderator_role_ids -> Nullable<Array<Nullable<Text>>>,
        blocked_words -> Nullable<Array<Nullable<Text>>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    macros (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        content -> Text,
        quick_access -> Nullable<Bool>,
        #[max_length = 255]
        guild_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    messages (id) {
        id -> Uuid,
        #[max_length = 255]
        author_id -> Varchar,
        #[max_length = 255]
        author_tag -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        attachments -> Nullable<Jsonb>,
        #[max_length = 255]
        guild_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    notes (id) {
        id -> Uuid,
        thread_id -> Nullable<Int4>,
        #[max_length = 255]
        author_id -> Varchar,
        #[max_length = 255]
        author_tag -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        #[max_length = 255]
        guild_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    servers (id) {
        id -> Int4,
        #[max_length = 255]
        guild_id -> Varchar,
        #[max_length = 255]
        guild_name -> Varchar,
        is_premium -> Bool,
        max_threads -> Nullable<Int4>,
        max_macros -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    thread_messages (thread_id, message_id) {
        thread_id -> Int4,
        message_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_derive_enum::DbEnum;

    threads (id) {
        id -> Int4,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        thread_id -> Varchar,
        is_open -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 20]
        urgency -> Nullable<Varchar>,
        #[max_length = 255]
        guild_id -> Varchar,
    }
}

diesel::joinable!(notes -> threads (thread_id));
diesel::joinable!(thread_messages -> messages (message_id));
diesel::joinable!(thread_messages -> threads (thread_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    blocked_users,
    guild_configs,
    macros,
    messages,
    notes,
    servers,
    thread_messages,
    threads,
);
