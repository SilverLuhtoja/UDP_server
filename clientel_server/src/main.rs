use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::str;

const ADDR:&str = "127.0.0.1";
const SERVER_PORT:u16 = 4242;
const CLIENT_PORT:u16 = 34254;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("{}:{}",ADDR, CLIENT_PORT))?;
    println!("Establishing UDP socket address at : {:?}", socket);
    try_to_connect(&socket); // "fake handshake"

    println!("\nwrite to server:");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if &line == "BYE" {
            break;
        }
        socket.send(line.as_bytes()).expect("Error on send");
        read_incoming_messages(&socket);
    }
    Ok(())
}

// this is basically UDP handshake, to see if anything is even otherside to receive messages
fn try_to_connect(socket : &UdpSocket){
    let addr = &format!("{}:{}",ADDR, SERVER_PORT);
    socket.connect(addr).expect("connect function failed");
    // socket.send_to("connect".as_bytes(),&addr).unwrap();
    socket.send("connect".as_bytes()).expect("message sending failed"); // when connected, can just send not send_to
    read_incoming_messages(socket);
}
 
fn read_incoming_messages(socket :&UdpSocket) {
    let mut buf = [0; 2048];
	let (amt, _src) = socket.recv_from(&mut buf).expect("incoming message failed");
    let filled_buf = &mut buf[..amt];
    let incoming_message = String::from_utf8_lossy(filled_buf).into_owned();
    println!("SERVER --> {:?}", incoming_message);
}
