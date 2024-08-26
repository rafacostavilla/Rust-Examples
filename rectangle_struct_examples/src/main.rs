#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height*self.width
    }

    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool{
        self.width > rectangle.width && self.height > rectangle.height
    }
}

fn main() {
    let rect1= Rectangle {
        width: 30,
        height: 50,
    };
    
    
    let rect2= Rectangle {
        width: 20,
        height: 40,
    };
    
    let rect3= Rectangle {
        width: 20,
        height: 60,
    };

    println!(
        "Rectangle area is {} square pixels",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; It is {}", rect1.width);
    }

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));
}
