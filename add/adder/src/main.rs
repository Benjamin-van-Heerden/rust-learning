use add_one;

fn main() {
    let num = 10;
    let num_add_one = add_one::add_one(num);
    println!("Hello, workspace");
    println!("Num was {}, now it is {}", num, num_add_one);
}
