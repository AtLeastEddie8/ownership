fn main() {

    let s = "hello";
    println!("{s}");

    {
        let r: &str = "HELOO";
        println!("{r}");
    }
    //out of scope
    //println!("{r}");

}
