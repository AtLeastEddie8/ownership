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
    println!("{} is the original number still existant", number)
    
    
}

fn moved_owner(a_string: String){
    println!("{} was the string moved here, will go out of scope and original non-existant", a_string);
}

fn making_copy(num: i32){
    println!("{} is the number copied, it will exist after still", num);
}
