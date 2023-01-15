use std::net::{UdpSocket, SocketAddr};

const ADDR:&str = "127.0.0.1";
const PORT:u16 = 4242;

// #[derive(Clone,Copy, Debug, PartialEq)]
pub struct Host{
    pub socket: UdpSocket,
    pub clients :  Vec<SocketAddr>
}

impl Host {
    pub fn new() {
    }
}



// fn main() -> std::io::Result<()> {
//     // for UDP4
//     // let socket = UdpSocket::bind("[::]:2000")?;  // for UDP4/6
//     let socket = UdpSocket::bind(format!("{}:{}",ADDR, PORT))?; 
//     let mut buf = [0; 2048];

//     println!("Creating server : {:?}.Listening....", socket);
//     println!("Waiting messages:");
//     loop {
//         println!();
//         // (bite_slice, address where it came from)
//         let (amt, src) = socket.recv_from(&mut buf).expect("incoming message failed");
//         let incoming_message = String::from_utf8_lossy(&mut buf[..amt]);
//         println!("client <{}>: {:?}", src, incoming_message);
        
//         if incoming_message == "connect" {
//             let message  = format!("Successful connection with {}:{}",ADDR,PORT);
//             socket.send_to(message.as_bytes(), &src)?;
//         }else{
//             // Redeclare `buf` as slice of the received data
//             // and send data back to origin.
//             let buf = &mut buf[..amt];
//             socket.send_to(&buf, &src)?;
//         }
//     }
// }
