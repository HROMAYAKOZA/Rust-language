use std::io::stdin;


fn add_one(mut item: i32) -> i32{
    item+=1;
    item
}


fn main() {
    println!("Hello from part2");
    let x: i32 = 2343;
    // x=1112;
    let x = String::from("value");
    println!("x = {x}");
    println!("y = {y}");
    let y: i32 = 1;
    {
        let y: i32 = 2;
        println!("y = {y}");
    }
    println!("y = {y}");
    let mut guess = String::from("");
    stdin().readline(&mut guess).unwrap();
    guess = guess.trim().to_string();
}
