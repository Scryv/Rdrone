use std::net::UdpSocket;
use std::str;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3232").unwrap();
    println!("Waiting for input");

    let mut buf = [0; 2048];
    loop {
        let (bts, src) = socket.recv_from(&mut buf).unwrap();

        let sbuf = &mut buf[..bts];
        println!("{} AND {:?}", src, str::from_utf8(&sbuf));
    }
}
