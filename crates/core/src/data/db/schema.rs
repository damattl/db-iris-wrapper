// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Text,
        train_id -> Text,
        valid_from -> Nullable<Timestamp>,
        valid_to -> Nullable<Timestamp>,
        priority -> Nullable<Int2>,
        category -> Nullable<Text>,
        code -> Nullable<Int4>,
        timestamp -> Timestamp,
        m_type -> Nullable<Text>,
        last_updated -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    stations (id) {
        id -> Int4,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
        name -> Text,
        ds100 -> Text,
    }
}

diesel::table! {
    status_codes (code) {
        code -> Int2,
        c_type -> Nullable<Text>,
        long_text -> Text,
    }
}

diesel::table! {
    stops (id) {
        id -> Text,
        train_id -> Text,
        station_id -> Int4,
        arrival_platform -> Nullable<Text>,
        arrival_planned -> Nullable<Timestamp>,
        arrival_planned_path -> Nullable<Text>,
        arrival_changed_path -> Nullable<Text>,
        departure_platform -> Nullable<Text>,
        departure_planned -> Nullable<Timestamp>,
        departure_planned_path -> Nullable<Text>,
        departure_changed_path -> Nullable<Text>,
        arrival_current -> Nullable<Timestamp>,
        departure_current -> Nullable<Timestamp>,
    }
}

diesel::table! {
    trains (id) {
        id -> Text,
        operator -> Nullable<Text>,
        category -> Text,
        number -> Text,
        line -> Nullable<Text>,
        date -> Date,
    }
}

diesel::joinable!(messages -> trains (train_id));
diesel::joinable!(stops -> stations (station_id));
diesel::joinable!(stops -> trains (train_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    stations,
    status_codes,
    stops,
    trains,
);
