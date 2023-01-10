use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::env;
use std::str;

fn main() -> std::io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("Usage {} hostname", args[0]);
    //     std::process::exit(1);
    // }
    // let hostname = &args[1];

    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    println!("Creating connection : {:?}", socket);
   
    println!();
    println!("write to server:");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
	let line = line.unwrap();
	println!("Line read from stdin '{}'", line);
	if &line == "BYE" {
	    break;
	}

	socket.send_to(line.as_bytes(), "127.0.0.1:4242").expect("Error on send");

	let mut buf = [0; 2048];
	let (amt, _src) = socket.recv_from(&mut buf)?;
    let filled_buf = &mut buf[..amt];
    println!("From server: {:?}", String::from_utf8_lossy(filled_buf));
    }
    Ok(())
}
