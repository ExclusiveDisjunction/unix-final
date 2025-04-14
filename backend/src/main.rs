

fn main() {
    println!("Hello, world!");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Still alive!");
    }
}
