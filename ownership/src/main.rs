fn main() {
    //RULES
    // 1- each value in rust has a variable that's called its owner
    // 2- there can be only one owner at a time
    // 3- when the owner goes out of scope the value will be dropped

    // using the heap 
    // this uses Boxe
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);

    // x is in the stack it has fixed length
    let x = 5;
    let y = x; // MOVING DEEP COPY int implements the COPY TRAIT

    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1; // we copy the pointer only not the data in the stack
   // println!("{}", s1); doesn't work because s1 has been moved, solution would be to clone (i.e deep copy)


   //functions 

   let hello = String::from("Hello"); 

   let x = 5;

   takes_ownership(hello);

   //hello is no longer valid

   makes_copy(x);
   // its okay because x was copied
   let y = x;

   let mut hello = takes_and_gives_back(gives_ownership());

   println!("{}", calculate_length(&hello));   


   // mutable references rule: You can only have one mutable reference to a particular piece of data in a particular scope
   // prevents data races

   {
    let r1 = &mut hello;
    r1.push_str(", world");
   }

   let r2 = &mut hello;

   println!("{}", r2);


   //Rules of references

   // 1- At any given time you can have either but not both of: 
   // * one mutable reference
   // * any number of immutable references



   //4.3 SLICES

   let s = String::from("hello world");

   let hello = &s[0..5];

   let hello = &s[..5];

   println!("{}", first_word(&s));
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

   //reference we borrow it
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]


    //data should never be aliased & mutated at the same time

    // the RWO rule :

    // R: Read  : Data can be copied to a different place
    // W: Write : Data can be mutated in place
    // O: Owned : Data can be moved or dropped


    //by default variables have R.O permissions and if they're declared mut they have W aswell
    // references can temporarly remove those permissions


    //mutable reference => allow mutation prevent aliasing
    //immutable reference => allow aliasing prevent mutation
}