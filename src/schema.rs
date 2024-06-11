// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        notes -> Nullable<Text>,
        overall -> Nullable<Float8>,
        psychomotor -> Nullable<Float8>,
        energy -> Nullable<Float8>,
        mood -> Nullable<Float8>,
        thoughts_slowed_racing -> Nullable<Float8>,
        concentration_difficulty -> Nullable<Float8>,
        time_submitted -> Int8,
    }
}
