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
    
}
