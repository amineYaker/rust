fn main() {
    println!("Hello, world!");
    my_function(7);

    let x = 5;
    let y = {
        let x = 3;
        x +1 //this is an expression
        // x + 1; is a statement that won't compile
    };

    println!("the value of y is: {} ", y);
    
    let res = five();
    println!("the result of five() is : {}", res);
}

fn my_function(x: i32) {
    println!("x is : {}", x)
}

fn five() -> i32 {
    let x = 42;
    5
}


//statements vs expressions