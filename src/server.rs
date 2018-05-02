use std::net::UdpSocket;

pub fn start() {
    let mut socket = UdpSocket::bind("0.0.0.0:12345").unwrap();
    let mut buf = [0; 8000];
    let mut ips = Vec::new();

    loop {
    	let (size, addr) = socket.recv_from(&mut buf).expect("failed to receive data");
    	ips.push(addr);

    	for ip in ips.iter().filter(|x| **x != addr) {
    		let _ = socket.send_to(&buf[..size], ip);
    	}
    }
}
