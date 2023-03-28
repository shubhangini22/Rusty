fn main() {
    let x=9;
    //this is an immutable variable by default in rust
    println!("{x}");
    x=10;
    println!("{x}");
    //cannot assign variable twice to an immutable variable
}
