// Tady jsem jenom přepsal tu strukturu, co jsme navrhli


enum Content {
    Text(String),
    Equation(String), // Zatím string
    File(String, Vec<u8>),
    HTML(String),
}

enum Target {
    User(String),
    Channel(String),
    Houbi,
}

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
    println!("Hello, world!");
}
