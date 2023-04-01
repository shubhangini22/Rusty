fn main(){
    let s=String::from("Hello World!");

    //we have to calculate length of the string
    let len=func(&s);
    println!("{len}");
}
fn func(s1: &String)->usize{
   s1.len()
   //we have to return length
}
//references are immutable  by default
// we can't modify something which is passed by reference
//by adding mut keyword we can allow mutability to references as well