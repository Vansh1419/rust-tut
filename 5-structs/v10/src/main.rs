#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}
// Implementation block of area 
impl Rectangle{
    fn area_impl(&self)->u32{
        self.width * self.height
    }
    fn width(&self)->bool{
        self.width>0
    }
    // getter
    fn get_width(&self)->u32{
        self.width
    }
    fn get_height(&self)->u32{
        self.height
    }

    // associated function
    fn square(size:u32)->Self{
        Self{
            width:size,
            height:size,
        }
    }
}
fn main() {
    let rect1 = Rectangle{width:30,height:50};
    println!("Rectangle {:?}",area(&rect1));
    println!("Rectangle {:?}",rect1);
    println!("Rectangle via impl {:?}",rect1.area_impl());
    println!("Rectangle via impl {:?}",rect1.width());
    println!("Hello, world!");

    let sq1 = Rectangle::square(10);
    println!("Square via impl {:?}",sq1.area_impl());

}

fn area(rectangle : &Rectangle)-> u32{
    rectangle.width * rectangle.height
}