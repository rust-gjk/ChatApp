#![feature(type_ascription)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

#[macro_use]
extern crate text_io;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate ChatApp;

use std::net::UdpSocket;

mod client;
mod server;

fn main() {
	println!("start server?");
	match (read!(): String).to_lowercase().as_ref() {
		"yes" | "y" => server::start(),
		_ => client::start(),
	}
}
