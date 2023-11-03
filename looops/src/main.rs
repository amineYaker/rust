use core::num;

fn main() {

    //let mut number =3;

    // while number !=0 {
    //     println!("{}!", number);

    //     number = number -1;

    for number in (1..4).rev() {
        
        println!("{}!", number );
    } 
    println!("LIFTOFF!!!");
    

    // for loops to iter

    let a = [10, 20, 30, 40, 50]; 

    for elem in a.iter() {
        println!("the value is: {}", elem);
    }


    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
    
        loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
    println!("End count = {count}");


    let a = [5; 10];

    let mut sum = 0;

    for x in a {

        sum += x;

    }

    println!("{sum}");

}
