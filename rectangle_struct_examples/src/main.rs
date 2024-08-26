#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height*self.width
    }
}

fn main() {
    let rect1= Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Rectangle area is {} square pixels",
        rect1.area()
    );
}
