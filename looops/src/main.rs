fn main() {

    let mut number =3;

    while number !=0 {
        println!("{}!", number);

        number = number -1;
    }

    println!("LIFTOFF!!!");
    

    // for loops to iter

    let a = [10, 20, 30, 40, 50]; 

    for elem in a.iter() {
        println!("the value is: {}", elem);
    }
}
