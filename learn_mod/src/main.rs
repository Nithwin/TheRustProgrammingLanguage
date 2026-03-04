mod player {
    fn calculate_secret_stats() {
        println!("Secret Calculated");
    }
    pub fn jump() {
        println!("The payer jumps!");
    }
}

fn main() {
    player::jump();
    println!("Hello, world!");
}
