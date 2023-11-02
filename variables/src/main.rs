fn main() {

    let  x = 5;
    let x = x+1;
    let x = x*2;
    println!("The value of x is: {}", x);

    // Types

    let dec: i32 = 98_222;
    let hex: i32 = 0xff;
    let bin: i32 = 0b1111_0000;

    println!("bin is {}", bin);

    //floating point 

    let x = 2.0; //f64

    //operations are standard ...
    //boolean
    let t: bool = true;


    //char type will be discussed later

    //tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y , z) = tup ;

    println!("the value of y is: {}", y);

    let y2 = tup.1;

    println!("the value of y is: {}", y2);

    //arrays are fixed length it allocates data on the stack & not on the heap

    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"] ; 

    let today = days[3];
    println!("today is {}", today);

    // functions
}
