extern crate azure_sdk_for_rust;

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::storage::client::Client;

use azure_sdk_for_rust::azure::storage::container::Container;
use azure_sdk_for_rust::azure::storage::container::LIST_CONTAINER_OPTIONS_DEFAULT;

use azure_sdk_for_rust::azure::storage::blob::{Blob, LIST_BLOB_OPTIONS_DEFAULT};

fn main() {
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT")
        .expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY")
        .expect("Set env variable STORAGE_MASTER_KEY first!");

    let mut core = Core::new()?;

    let client = Client::new(&core.handle(), &account, &master_key)?;

    let future = Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT).map(|iv| {
        println!("List containers returned {} containers.", iv.len());
        for cont in iv.iter() {
            println!("\t{}", cont.name);
        }
    });

    core.run(future)?;

    let future = Blob::list(&client, "mindarpa", &LIST_BLOB_OPTIONS_DEFAULT).map(|iv| {
        println!("List blob returned {} blobs.", iv.len());
        for cont in iv.iter() {
            println!(
                "\t{}\t{} MB",
                cont.name,
                cont.content_length / (1024 * 1024)
            );
        }
    });

    core.run(future)?;

    Ok(())
}
