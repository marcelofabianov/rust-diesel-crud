// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 255]
        nome -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        disabled_at -> Nullable<Timestamp>,
    }
}
