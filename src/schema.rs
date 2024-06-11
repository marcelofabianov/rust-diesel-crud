// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 255]
        fullname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        disabled_at -> Nullable<Timestamp>,
    }
}
