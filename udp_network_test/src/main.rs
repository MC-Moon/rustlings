use std::net::UdpSocket;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, UDP Test");
    let socket = UdpSocket::bind("192.168.47.255:12345")?;

    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        let len = socket.recv(&mut buffer)?;
        let slice = std::str::from_utf8(&buffer[0..len]).unwrap();

        println!("{slice:?}");
    }
    Ok(())
}
