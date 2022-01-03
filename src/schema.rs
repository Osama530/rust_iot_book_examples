table! {
    book (id) {
        id -> Int4,
        title -> Varchar,
        auther -> Varchar,
        published -> Bool,
    }
}

table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        auther -> Varchar,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    book,
    books,
);
