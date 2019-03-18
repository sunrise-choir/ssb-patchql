use serde_json::Value;

pub fn insert_branches(connection: &SqliteConnection, message: &SsbMessage, message_key_id: i64) {
    unimplemented!();
//    if let Some(branches_value) = message.value.content.get("branch") {
//        let mut insert_branch_stmt = connection
//            .prepare_cached(
//                "INSERT INTO branches_raw (link_from_key_id, link_to_key_id) VALUES (?, ?)",
//            )
//            .unwrap();
//
//        let branches = match branches_value {
//            Value::Array(arr) => arr
//                .iter()
//                .map(|value| value.as_str().unwrap().to_string())
//                .collect(),
//            Value::String(branch) => vec![branch.as_str().to_string()],
//            _ => Vec::new(),
//        };
//
//        branches
//            .iter()
//            .map(|branch| find_or_create_key(connection, branch).unwrap())
//            .for_each(|link_to_key_id| {
//                insert_branch_stmt
//                    .execute(&[&message_key_id, &link_to_key_id])
//                    .unwrap();
//            })
//    }
}


