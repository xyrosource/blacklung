extern crate futures;
extern crate tokio_core;
extern crate slog;

pub mod errors {
    error_chain!{}
}

pub mod console;

use self::errors::*;

use self::futures::{Future, Stream};
use self::tokio_core::io::{copy, Io};
use self::tokio_core::net::TcpListener;
use self::tokio_core::reactor::Core;

use slog::Logger;

pub fn start(root_logger: &Logger, port: u16) -> Result<()> {
    // Create the event loop that will drive this server
    let mut core = Core::new().chain_err(|| "Failed to create core")?;
    let handle = core.handle();

    // Going via the format macro is less cluttered than the IPAddr constructors.
    let full_address: &str = &format!("127.0.0.1:{}", port);
    // Bind the server's socket
    let addr = full_address.parse().chain_err(|| "Invalid server address")?;
    let sock = TcpListener::bind(&addr, &handle).chain_err(|| "Failed to bind socket")?;

    let server_logger = root_logger.new(o!("server" => full_address.to_string()));
    info!(server_logger, "Listening.");

    // Pull out a stream of sockets for incoming connections
    let server = sock.incoming().for_each(|(sock, _)| {
        let client_logger = server_logger.new(o!(
                "client" => sock.peer_addr()?.to_string()
                ));
        info!(client_logger, "Client connected.");

        // We need to clone the logger, as we move it into the error logging future..
        let error_logger = client_logger.clone();

        // Split up the reading and writing parts of the
        // socket
        let (reader, writer) = sock.split();

        // A future that echos the data and returns how
        // many bytes were copied...
        let bytes_copied = copy(reader, writer);


        // ... after which we'll print what happened
        let handle_conn = bytes_copied.map(move |amt| {
            debug!(client_logger, "Data sent"; "bytes" => amt);
        }).map_err(move |err| {
            error!(error_logger, "IO error"; "err" => err.to_string())
        });

        // Spawn the future as a concurrent task
        handle.spawn(handle_conn);

        Ok(())
    });

    use self::console::Console;
    use self::futures::sync::mpsc::channel;
    use self::futures::stream::iter;
    use self::futures::Sink;
    use std::thread;
    use std::io::{Error, ErrorKind};
    let (sender, receiver) = channel(0);
    let error_logger = root_logger.clone();
    let child = thread::spawn(move || {
        let console = Console::new();
        if let Err(_) = sender.send_all(iter(console.map(|cmd| Ok(cmd)))).wait() {
            error!(error_logger, "Failed to send command");
            return;
        }
    });

    let console = receiver.for_each(|cmd| {
        info!(root_logger, "Command {:?}", cmd);
        Ok(())
    }).map_err(|_| Error::new(ErrorKind::Other, "Failed to process command"));

    // Spin up the server on the event loop
    if let Err(_) = core.run(server.select(console)) {
        bail!("Failed to start event loop");
    }

    info!(root_logger, "Event loop terminated");

    if let Err(_) = child.join() {
        bail!("Failed to join console thread");
    }

    Ok(())
}
