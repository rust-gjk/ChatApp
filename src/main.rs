// Matěj je MEGA debil

#[macro_use]
extern crate text_io;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::net::UdpSocket;


#[derive(Serialize, Deserialize)]
enum Content {
    Text(String),
    Equation(String), // Zatím string
    File(String, Vec<u8>),
    HTML(String),
}

#[derive(Serialize, Deserialize)]
enum Target {
    User(String),
    Channel(String),
    Houbi,
}

#[derive(Serialize, Deserialize)]
enum Packet {
    Message {
        sender: String,
        hash: String,
        content: Content,
        target: Target,
    },
    Subscribe {
        sender: String,
        hash: String,
        target: Target,
    },
    Register {
        username: String,
        password: String,
    },
}

fn main() {
    let mut socket = UdpSocket::bind(":12345").unwrap();
}
