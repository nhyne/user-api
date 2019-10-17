
table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        salt -> Varchar,
        password -> Varchar,
    }
}
