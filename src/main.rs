use std::{io::{Read, Write}, net::TcpListener};

use clap::Parser as _;

#[derive(clap::Parser)]
struct Cli {
    listen_port: u16,
}

fn main() {
    let options = Cli::parse();

    let listener = TcpListener::bind(("0.0.0.0", options.listen_port)).unwrap();
    
    let (mut stream, peer_address) = listener.accept().unwrap();

    dbg!(peer_address);

    // TODO: Welcome the user by telling them their IP.
    let mut buf = [0u8; 10];
    loop {
        // We use `read` but `write_all` - this is intentional.
        dbg!(String::from_utf8_lossy(&buf));
        let n_bytes_read = stream.read(&mut buf).unwrap();
        if n_bytes_read == 0 {
            println!("Read 0 bytes - peer closed the connection.");
            break;
        }
        dbg!(String::from_utf8_lossy(&buf[..n_bytes_read]));
        stream.write_all(&buf[..n_bytes_read]).unwrap();
    }
}
