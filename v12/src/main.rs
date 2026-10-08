// ip addresses
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
// #[derive(Debug)]
// struct IpAddr {
//     kind:IpAddrKind,
//     address: String
// }

#[derive(Debug)]
struct QuitMessage;

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct WriteMessage(String);

#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32);

// Implementation with enums
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

// optional enum
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // let home = IpAddr{
    //     kind:IpAddrKind::V4,
    //     address:String::from("127.0.0.1")
    // };
    // println!("{:?}",home);

    // let loopback = IpAddr{
    //     kind:IpAddrKind::V6,
    //     address:String::from("::1")
    // };
    // println!("{:?}",loopback);

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("{:?}", home);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("{:?}", loopback);

    let m = Message::Write(String::from("Hello world"));
    let x = Message::Move { x: 3, y: 4 };
    let y = Message::ChangeColor(0, 0, 0);
    let z = Message::Quit;

    println!("{:?}", m);
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    m.call();
    x.call();
    y.call();
    z.call();
}
