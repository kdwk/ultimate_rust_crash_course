fn main() {
    let x = 23;
    {
        let x = 56;
        println!("{}", x);
    }
    println!("{}", x);
}
