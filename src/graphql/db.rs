use crate::db::*;
use flumedb::BidirIterator;
use itertools::Itertools;
use juniper::FieldResult;
use std::env;

use crate::db::models::append_item;
use diesel::prelude::*;
use diesel::result::Error;

use private_box::SecretKey;

#[derive(Default)]
pub struct DbMutation {}

/// The result of running a process mutation, giving the number of messages processed (the
/// chunkSize) and the new lastest flume_sequence. TBD if the flume seq should just be an opaque
/// cursor.
#[derive(GraphQLObject)]
struct ProcessResults {
    /// The number of entries inserted into the db (although not all of them will be useful to the
    /// application)
    chunk_size: i32,
    /// The most recent sequence number processed from the offset log. The offset log is the source
    /// of truth that this db is built off. This is unlikely to be used by an application and may
    /// be removed in the future.
    latest_sequence: Option<f64>,
}

graphql_object!(DbMutation: Context |&self| {
    description: "Mutations available to change the state of the db"

    /// This db will lag behind the offset log and needs calls to `process` to bring the db up to
    /// date. At first this might seem annoying and that the db should do this automatically. But
    /// this is a conscious design decision to give the app control of when cpu is used. This is
    /// very important on resource constrained devices, or even just when starting up the app. This
    /// is a major pain point in the javascript flume-db implementation that we're avoiding by
    /// doing this.
    field process(&executor, chunk_size = 100: i32) -> FieldResult<ProcessResults> {
        //TODO: get the secret key from env
        let secret_key_string =
            env::var("SSB_SECRET_KEY").expect("SSB_SECRET_KEY environment variable must be set");

        let secret_key_bytes = base64::decode(&secret_key_string)
            .unwrap_or(vec![0u8]);

        let secret_key = SecretKey::from_slice(&secret_key_bytes).unwrap_or_else(|| {
            warn!("Could not parse valid ssb-secret for decryption. Messages will not be decrypted");
            SecretKey::from_slice(&[0; 64]).unwrap()
        });
        let keys = [secret_key];

        let context = executor.context();
        let connection = context.connection.lock()?;

        //We're using Max of flume_seq.
        //When the db is empty, we'll get None.
        //When there is one item in the db, we'll get 0 (it's the first seq number you get)
        //When there's more than one you'll get some >0 number
        let max_seq = get_latest(&connection)?
            .map(|val| val as u64);

        let log = context.log.lock()?; //block here until any other thread is done with the log.

        let num_to_skip: usize = match max_seq {
            None => 0,
            _ => 1
        };

        let starting_offset = max_seq.unwrap_or(0);

        log.iter_at_offset(starting_offset)
            .forward()
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

        let new_latest = get_latest(&connection)?;
        Ok(ProcessResults{chunk_size, latest_sequence: new_latest})
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
