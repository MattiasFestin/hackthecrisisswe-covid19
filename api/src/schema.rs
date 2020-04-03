table! {
    base_table (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int4,
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
    transaction_direction (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    transaction_types (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
        row_version -> Int4,
        username -> Text,
        hash -> Text,
        roles -> Json,
    }
}

allow_tables_to_appear_in_same_query!(
    base_table,
    spatial_ref_sys,
    transaction_direction,
    transaction_types,
    users,
);
