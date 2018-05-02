use std::sync::Mutex;
use std::net::UdpSocket;
use std::thread;
use std::str;

pub fn start() {
	println!("zadejte IP serveru: ");
	let mut server_adresa: String = read!();
	server_adresa.push_str(":12345");
    let send_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind a socket");
    send_socket.connect(&server_adresa).expect("couldn't connect to server");
	let recv_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind a socket");
    recv_socket.connect(&server_adresa).expect("couldn't connect to server");
    recv_socket.send(b"connected");

    thread::spawn(move || read(recv_socket));
    loop {
    	let zprava: String = read!();
    	send_socket.send(zprava.as_bytes());
    }
}


fn read(socket: UdpSocket) {
	let mut buf = [0; 8000];

	loop {
		let size = socket.recv(&mut buf).expect("failed to receive data");
		println!("{}", unsafe { str::from_utf8_unchecked(&buf[..size]) })
	}
}
