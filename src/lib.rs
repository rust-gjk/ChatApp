#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(Serialize, Deserialize)]
pub enum Content {
    Text(String),
    Equation(String),
    File(String, Vec<u8>),
    HTML(String),
}

#[derive(Serialize, Deserialize)]
pub enum Target {
    User(String),
    Channel(String),
    Houbi,
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
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
