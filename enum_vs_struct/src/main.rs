/*
Advantages - 
- enum is more concise
- the name of each enum variant that we define also becomes a 
function that constructs an instance of the enum.
- each variant can have different types and amounts of associated data. 
-  can put any kind of data inside an enum variant: strings, numeric types, structs, enums
*/

// ===================================

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

// ----same code can be represented in consise format in enums-----

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

// ====================================

/*
- each variant can have different types and amounts of associated data. 
Ex - If we wanted to store V4 addresses as four u8 values 
but still express V6 addresses as one String value, 
we wouldn’t be able to with a struct. Enums handle this case with ease
*/

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));


// ==============================

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
This enum has four variants with different types:

Quit has no data associated with it at all.
Move has named fields like a struct does.
Write includes a single String.
ChangeColor includes three i32 values.
*/

fn main() {
    println!("Hello, world!");
}
