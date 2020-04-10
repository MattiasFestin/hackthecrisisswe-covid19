table! {
    areas (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int8,
        area_code -> Text,
        parent_area -> Uuid,
    }
}

table! {
    base_table (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int8,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    transactions (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int8,
        transaction_type_id -> Int8,
        transaction_direction_id -> Int8,
        what -> Text,
        priority -> Int8,
    }
}

table! {
    transactions_constraints (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int8,
        transactions_id -> Uuid,
        op -> Int8,
        name -> Text,
        unit -> Text,
        value -> Float4,
    }
}

table! {
    user_areas (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int8,
        user -> Uuid,
        area -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int8,
        username -> Text,
        hash -> Text,
        roles -> Json,
    }
}

joinable!(user_areas -> areas (area));
joinable!(user_areas -> users (user));

allow_tables_to_appear_in_same_query!(
    areas,
    base_table,
    spatial_ref_sys,
    transactions,
    transactions_constraints,
    user_areas,
    users,
);
