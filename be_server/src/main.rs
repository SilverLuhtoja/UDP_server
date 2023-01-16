use std::net::UdpSocket;

const ADDR:&str = "127.0.0.1";
const PORT:u16 = 4242;

fn main() -> std::io::Result<()> {
    // for UDP4
    // let socket = UdpSocket::bind("[::]:2000")?;  // for UDP4/6
    let socket = UdpSocket::bind(format!("{}:{}",ADDR, PORT))?; 
    let mut buf = [0; 2048];

    //TODO
    //1. create new maze
    //2. position clients with random positions




    println!("Creating server : {:?}.Listening....", socket);
    println!("Waiting messages:");
    loop {
        println!();
        // (bite_slice, address where it came from)
        let (amt, src) = socket.recv_from(&mut buf).expect("incoming message failed");
        let incoming_message = String::from_utf8_lossy(&mut buf[..amt]);
        println!("client <{}>: {:?}", src, incoming_message);

        //TODO
        //if client is connected, then send the map and positions 
        
        if incoming_message == "connect" {
            let message  = format!("Successful connection with {}:{}",ADDR,PORT);
            socket.send_to(message.as_bytes(), &src)?;
        }else if incoming_message == "stop"{
            println!("CLIENT LEAVING");
        }else {
            // Redeclare `buf` as slice of the received data
            // and send data back to origin.
            let buf = &mut buf[..amt];
            socket.send_to(&buf, &src)?;

        }
    }
}

