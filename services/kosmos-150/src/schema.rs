// @generated automatically by Diesel CLI.

diesel::table! {
    flights (id) {
        id -> Int4,
        spaceship_id -> Int4,
        from_spaceport_id -> Int4,
        to_spaceport_id -> Int4,
        departure -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        flight_id -> Int4,
        occupied_seat -> Int4,
        comment -> Nullable<Text>,
    }
}

diesel::table! {
    spaceports (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        star_system -> Varchar,
        #[max_length = 50]
        location -> Varchar,
    }
}

diesel::table! {
    spaceships (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        seats_number -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        password -> Varchar,
    }
}

diesel::joinable!(flights -> spaceships (spaceship_id));
diesel::joinable!(orders -> flights (flight_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    flights,
    orders,
    spaceports,
    spaceships,
    users,
);
