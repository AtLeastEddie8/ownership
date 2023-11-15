fn main() {

    let s = "hello";
    println!("{s}");

    {
        let r: &str = "HELOO";
        println!("{r}");
    }
    //out of scope
    //println!("{r}");
    
    //The String Type 
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // println!("{s}"); //same as above but no grapple

    //this can be used to reassign the value of a string once it is mutable 
    s = String::from("yo");
    println!("{s}");
    //Variables and Data Interacting with Move
    
    let x = 5;
    let _y = x;

    let s1 = String::from("Elephant");
    let s2 = s1;
    //this wont work bc s1 was moved to s2, therefore s1 no longer exists
    //println!("s1 = {}, s2 = {}", s1, s2);

    //this works bc tha value is cloned (copied) instead of being moved
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    let f = String::from("connection");
    moved_owner(f);
    //this wont work bc the ownership was moved to the function call
    //and once the fcuntion went out of scope the value no longer existed
    //println!("{} is the moved string no longer existant", f);

    let number = 34;
    making_copy(number);
    println!("{} is the original number still existant", number);
    
    {
        let mut f2 = String::from("secured");
        f2.push_str(" for now");
        println!("The value of f2 prior to move is '{}'", f2);
        //moved so f2 not valid anymore
        let f3 = f2;
        //f2 made valid again by making a new string, f3 now invalid
        let f2 = f3;
        //f2 made invalid again bc of fxn call
        println!("f2 is once again '{}'", f2);
        let f5 = moved_owner_and_returned(f2);
        println!("f5 is '{}'", f5);

        let f4 = String::from("closing");
        //f2 made valid once again from f4, f4 now invalid
        let f2 = String::from(moved_owner_and_returned(f4));
        println!("The value of f2 is now '{}'", f2);
        //will get error bc f4 was moved and its being borrowed
        //println!("The value of f4 is now '{}'", f4);
    }
    //example where instead of move the fxn borrows and the string is not invalid 
    {
        //ping-ponging ownership f2->f3->f2
        let f2 = String::from("Vampire");
        println!("f2 originally '{}'", f2);
        let f3 = f2;
        let f2 = f3;
        println!("f2 is still '{}'", f2);
        //new mutable string that will change value and then NOT pass ownership
        let mut f4 = f2;
        println!("f4 before change is '{}'", f4);
        f4 = String::from("Wolf");
        //wont work bc of prev command
        //println!("f2 is '{}', ", f2);
        let f2 = moved_owner_and_returned_borrow(&f4);
        println!("f2 revived again as '{}' and f4 is '{}' bc of mut", f2, f4);
    }
    //tuple part of 4.1 - 4.2
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }

    
}

fn moved_owner(a_string: String){
    println!("'{}' was the string moved here, will go out of scope and original non-existant", a_string);
}

fn making_copy(num: i32){
    println!("'{}' is the number copied, it will exist after still", num);
}

fn moved_owner_and_returned(a_string: String)->String{
    println!("In this function the ownership of '{}' returns bc of 'return'", a_string);
    return a_string;
}

fn moved_owner_and_returned_borrow(a_string: &String)->&String{
    println!("In this function the ownership of '{}' was borrowed", a_string);
    return a_string;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}