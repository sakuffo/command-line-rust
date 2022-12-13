#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn just_cause(&self) -> &str{
        return "let slip the dogs of war";
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 28,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("And also {:?}", rect1.just_cause());

}
