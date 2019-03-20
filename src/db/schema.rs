table! {
    abouts (id) {
        id -> Nullable<Integer>,
        link_from_key_id -> Nullable<Integer>,
        link_to_author_id -> Nullable<Integer>,
        link_to_key_id -> Nullable<Integer>,
    }
}

table! {
    authors (id) {
        id -> Nullable<Integer>,
        author -> Text,
        is_me -> Nullable<Bool>,
    }
}

table! {
    blob_links (id) {
        id -> Nullable<Integer>,
        link_from_key_id -> Integer,
        link_to_blob_id -> Integer,
    }
}

table! {
    blobs (id) {
        id -> Nullable<Integer>,
        blob -> Text,
    }
}

table! {
    branches (id) {
        id -> Nullable<Integer>,
        link_from_key_id -> Integer,
        link_to_key_id -> Integer,
    }
}

table! {
    contacts (id) {
        id -> Nullable<Integer>,
        author_id -> Integer,
        contact_author_id -> Integer,
        is_decrypted -> Bool,
        state -> Nullable<Integer>,
    }
}

table! {
    keys (id) {
        id -> Nullable<Integer>,
        key -> Text,
    }
}

table! {
    links (id) {
        id -> Nullable<Integer>,
        link_from_key_id -> Integer,
        link_to_key_id -> Integer,
    }
}

table! {
    mentions (id) {
        id -> Nullable<Integer>,
        link_from_key_id -> Integer,
        link_to_author_id -> Integer,
    }
}

table! {
    messages (flume_seq) {
        flume_seq -> Nullable<BigInt>,
        key_id -> Nullable<Integer>,
        seq -> Nullable<Integer>,
        received_time -> Nullable<Double>,
        asserted_time -> Nullable<Double>,
        root_key_id -> Nullable<Integer>,
        fork_key_id -> Nullable<Integer>,
        author_id -> Nullable<Integer>,
        content_type -> Nullable<Text>,
        content -> Nullable<Text>,
        is_decrypted -> Nullable<Bool>,
    }
}

table! {
    votes (id) {
        id -> Nullable<Integer>,
        link_from_author_id -> Integer,
        link_to_key_id -> Integer,
        value -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    abouts, authors, blob_links, blobs, branches, contacts, keys, links, mentions, messages, votes,
);
