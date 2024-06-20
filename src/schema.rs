// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        userid -> Int8,
        #[max_length = 128]
        fieldname -> Varchar,
        #[max_length = 128]
        fieldtype -> Varchar,
        fieldval -> Nullable<Text>,
        timestamp -> Int8,
    }
}
