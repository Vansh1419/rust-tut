#[derive(Debug)]
struct User{
    name : String,
    email : String,
    is_active : bool,
    age : u8
}

// tupple struct
#[derive(Debug)]
struct Color(i32,i32,i32);
#[derive(Debug)]
struct Point(i32,i32,i32);

// unit like struct
#[derive(Debug)]
struct UnitLike;

fn main() {
    let mut user1 = User{
        name : String::from("John doe"),
        email : String::from("deo@email.com"),
        is_active: true,
        age : 23
    };
    println!("{} {} {} {}",user1.name, user1.email, user1.age, user1.is_active);
    println!("{:?}",user1);
    user1.name =String::from("Jane doe");
    println!("{:?}",user1);
    let user2 = build_user(
        String::from("User2"),
        String::from("u2@email.com"),
        true,
        24
    );
    println!("{:?}",user2);

    let user3 = User{
        name:String::from("user3"),
        email:user2.email.clone(), // heap
        age:user2.age, // stack so no need to clone
        is_active:false
    };
    println!("{:?}",user3);
    println!("{:?}",user2); // there is a borrowing error
    
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("{:?}",black);
    println!("{:?}",origin);

    let unit = UnitLike;
    println!("UnitLike : {:?}",unit);

}

// create a function to build a user
fn build_user(name:String, email:String, is_active:bool, age:u8)-> User{
    User{
        name,
        email,
        is_active,
        age
    }
}