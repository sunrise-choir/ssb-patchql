use crate::db::*;
use flumedb::offset_log::{LogEntry, OffsetLogIter};
use flumedb::BidirIterator;
use itertools::Itertools;

use crate::db::models::append_item;
use diesel::prelude::*;
use diesel::result::Error;

use private_box::SecretKey;

#[derive(Default)]
pub struct DbMutation {}

#[derive(GraphQLObject)]
struct ProcessResults {
    chunk_size: i32,
    latest_sequence: Option<f64>,
}

struct LogIter<T> {
    log_iter: OffsetLogIter<T>,
}
impl<T> Iterator for LogIter<T> {
    type Item = LogEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.log_iter.next()
    }
}

graphql_object!(DbMutation: Context |&self| {
    field process(&executor, chunk_size = 100: i32) -> ProcessResults {
        //TODO: get the secret key from env
        let secret_key_bytes = &vec![0];
        let secret_key = SecretKey::from_slice(secret_key_bytes).unwrap_or_else(|| {
            let empty_slice = vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0,
            ];
            SecretKey::from_slice(&empty_slice[0..32]).unwrap()
        });
        let keys = vec![secret_key];


        let context = executor.context();
        let connection = context.connection.lock().unwrap();

        //We're using Max of flume_seq.
        //When the db is empty, we'll get None.
        //When there is one item in the db, we'll get 0 (it's the first seq number you get)
        //When there's more than one you'll get some >0 number
        let max_seq = get_latest(&connection)
            .unwrap()
            .map(|val| val as u64);

        let log = context.log.lock().unwrap(); //block here until any other thread is done with the log.

        let num_to_skip: usize = match max_seq {
            None => 0,
            _ => 1
        };

        let starting_offset = max_seq.unwrap_or(0);
        let log_iter = LogIter{log_iter: log.iter_at_offset(starting_offset)};

        log_iter
            .skip(num_to_skip)
            .take(chunk_size as usize)
            .chunks(10000)
            .into_iter()
            .for_each(|chunk|{
                //We use iter tools to set an upper bound on the size of chunks we process here.
                //It avoids collecting into a vec and consuming way too much memory if the caller
                //tries to process the entire log.
                connection.transaction::<_, Error, _>(||{
                    chunk
                        .for_each(|log_entry|{
                            append_item(&(*connection), &keys, log_entry.offset, &log_entry.data).unwrap();
                        });
                        Ok(())
                }).unwrap();

            });

        let new_latest = get_latest(&connection).unwrap();
        ProcessResults{chunk_size, latest_sequence: new_latest}
    }
});

#[derive(Default)]
pub struct Db {}

graphql_object!(Db: Context |&self| {
    field latest(&executor) -> Option<f64> {
        //let id = self.id;
        let context = executor.context();
        let connection = context.connection.lock().unwrap();
        get_latest(&connection).unwrap()
    }
});
