use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    const PORT:u16 = 4242;
    let addr = format!("127.0.0.1:{}", PORT);
    let socket = UdpSocket::bind(addr)?; // for UDP4
    // let socket = UdpSocket::bind("[::]:2000")?;  // for UDP4/6
    let mut buf = [0; 2048];

    println!("Creating server : {:?}.Listening....", socket);
    loop {
        println!();
        println!("Waiting messages:");
        // receiving from connected server
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("amt : {} , src : {}", amt,src);
        let filled_buf = &mut buf[..amt];
        println!("message: {:?}", String::from_utf8_lossy(filled_buf));
        
        // Redeclare `buf` as slice of the received data
	    // and send data back to origin.
        let buf = &mut buf[..amt];
        let messagef= ["Server: ".as_bytes(),buf].concat();
        socket.send_to(&messagef, &src)?;
    }
}
