table! {
    expenses (id) {
        id -> Uuid,
        description -> Varchar,
        amount -> Float4,
        payment_method -> Varchar,
        created -> Timestamp,
        last_updated -> Timestamp,
    }
}
