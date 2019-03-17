use crate::db::*;
use flumedb::offset_log::{LogEntry, OffsetLogIter};
use flumedb::BidirIterator;
use itertools::Itertools;

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
        //let id = self.id;
        let context = executor.context();
        let connection = context.pool.get().unwrap();

        //This is wrong
        //We're using Max of flume_seq.
        //When the db is empty, we'll get None. 
        //When there is one item in the db, we'll get 0 (it's the first seq number you get)
        //When there's more than one you'll get some >0 number
        let max_seq = get_latest(&connection)
            .unwrap()
            .map(|val| val as u64);

        println!("latest seq {:?}", max_seq);

        let log = context.log.lock().unwrap(); //block here until any other thread is done with the log.

        let num_to_skip: usize = match max_seq {
            None => 0,
            _ => 1
        };

        let starting_offset = max_seq.or(Some(0)).unwrap();
        let log_iter = LogIter{log_iter: log.iter_at_offset(starting_offset)};

        log_iter
            .skip(num_to_skip)
            .take(chunk_size as usize)
            .chunks(1000);

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
        let connection = context.pool.get().unwrap();
        get_latest(&connection).unwrap()
    }
});
