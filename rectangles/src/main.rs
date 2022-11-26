fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area());
    println!("rect is {:?}",rect);
    println!("Can rect hold rect 2? {}", rect.can_hold(&rect2));
    println!("can rect3 hold rect ? {}", rect3.can_hold(&rect));
}
//this is so we can print it in the println above
//for more detailed use {:#?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self{
            width: size,
            height: size,
        }
    }
}
