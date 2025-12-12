mod solutions;

fn main() {
    println!("[+] Calling Day 1 solution function");
    match solutions::day_1() {
        Ok(n) => println!("Solution is {}", n),
        Err(s) => println!("{}", s),
    }
}
