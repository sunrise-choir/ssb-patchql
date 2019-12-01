use serde_json::{from_str, Value};
use ssb_patchql_core;
use ssb_patchql_core::Patchql;

#[test]
fn first() {
    let offset_log_path = "./misc/fifty_replies.offset".to_owned();
    let db_path = "/tmp/first.sqlite".to_owned();

    let patchql = Patchql::new(offset_log_path, db_path, "".to_owned(), "".to_owned());

    let process_response = patchql.query(PROCESS).unwrap();
    let response = patchql.query(THREADS_FIRST).unwrap();
    let jsn: Value = from_str(&response).unwrap();
    assert_eq!(
        jsn["data"]["threads"]["edges"][0]["node"]["root"]["text"]
            .as_str()
            .unwrap(),
        "Root 1"
    );
}

const PROCESS: &str = r##"
{
    "operationName":"process",
    "variables":{"chunkSize":10000},
    "query":
        "mutation process($chunkSize: Int) {\n  process(chunkSize: $chunkSize) {\n    __typename\n    chunkSize\n    latestSequence\n  }\n}"}
"##;

const THREADS_FIRST: &str = r##"
{"query":"{\n\tthreads(first: 1 ){\n    totalCount\n    edges{\n      cursor\n      node{        \n        root{\n          id\n          text\n          author{\n            name\n          }\n        }\n        replies{\n          id\n          text\n          author{\n            name\n          }\n        \n      \t}\n      }\n    }\n  }\n}\n","variables":null,"operationName":null}
"##;
