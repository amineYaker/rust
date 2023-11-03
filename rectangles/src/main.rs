fn main() {

    let length1 = 50;
    let width1 = 20;

    let rec1 = Rectangle { length: length1, width: width1};
    println!("the area of the rectangle is: {}", area(&rec1));

    //debug format
    //println!(" rect1 is {:?}", rec1);

    //pretty debug
    println!(" rect1 is {:#?}", rec1.area());

    let rec2 = Rectangle{ length: 46, width: 18};

    println!(" can rectangle 2 fit into rectangle 1 ? \n {} ", rec1.can_hold(&rec2));

    //calling associated function
    let square = Rectangle::square(33);

    println!("the area of our square is :{}",square.area());
}

//function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    //method
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    //associated function
    fn square(size :u32) -> Rectangle {
        Rectangle{length: size, width: size}
    }
}