fn main() {
    println!("Hello, world!");
    let a = add(1, 2);
    print!("{}", a);
    let ab = Rectangle::new(1, 2);
    let area = ab.area();
    print!("{}", area);
}

fn add(a: i8, b: i8) -> i8 {
    return a + b;
}

struct Rectangle {
    length: i8,
    breadth: i8,
}

impl Rectangle {
    fn area(&self) -> i8 {
        self.length * self.breadth
    }
    // Another method to create a new rectangle
    fn new(length: i8, breadth: i8) -> Rectangle {
        Rectangle { length, breadth }
    }
}
