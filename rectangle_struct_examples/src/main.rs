#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self, rectangle: &Rectangle) -> bool{
        self.width > rectangle.width && self.height > rectangle.height
    }
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height*self.width
    }

    fn width(&self) -> bool{
        self.width > 0
    }

    //Associated function: To call it use Rectangle::square(3), by instance
    //Associated functions don't need an instance of the type to work with
    //All the methods are associated functions, but not always the opposite
    fn square(size: u32) -> Self{
        Rectangle{
            width: size,
            height: size,
        }
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

    println!(
        "A created square from the Rectangle Struct: {:#?}",
        Rectangle::square(3)
    )
}
