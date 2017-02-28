extern crate futures;
extern crate tokio_core;

pub mod errors {
    error_chain!{}
}

use self::errors::*;

use self::futures::{Future, Stream};
use self::tokio_core::io::{copy, Io};
use self::tokio_core::net::TcpListener;
use self::tokio_core::reactor::Core;

pub fn start() -> Result<()> {
    // Create the event loop that will drive this server
    let mut core = Core::new().chain_err(|| "Failed to create core")?;
    let handle = core.handle();

    // Bind the server's socket
    let addr = "127.0.0.1:12345".parse().chain_err(|| "Invalid server address")?;
    let sock = TcpListener::bind(&addr, &handle).chain_err(|| "Failed to bind socket")?;

    // Pull out a stream of sockets for incoming connections
    let server = sock.incoming().for_each(|(sock, _)| {
        // Split up the reading and writing parts of the
        // socket
        let (reader, writer) = sock.split();

        // A future that echos the data and returns how
        // many bytes were copied...
        let bytes_copied = copy(reader, writer);

        // ... after which we'll print what happened
        let handle_conn = bytes_copied.map(|amt| {
            println!("wrote {} bytes", amt)
        }).map_err(|err| {
            println!("IO error {:?}", err)
        });

        // Spawn the future as a concurrent task
        handle.spawn(handle_conn);

        Ok(())
    });

    // Spin up the server on the event loop
    core.run(server).chain_err(|| "Failed to start event loop")?;

    Ok(())
}
