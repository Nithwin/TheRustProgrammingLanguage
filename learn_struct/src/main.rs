
struct Enemy {
    name:String,
    damage: i32,
}

impl Enemy {
    fn battle_cry(&self) {
        print!("The {} to take {}", self.name, self.damage);
    }

    fn weaken(&mut self, amount: i32){
        self.damage = self.damage - amount;
        print!("Weakned!")
    }
}


fn main(){
    let mut goblin = Enemy {
        name: String::from("Goblin"),
        damage : 15
    };
    goblin.battle_cry();
    goblin.weaken(3);
    print!("Hello This is Nithwin");
}





// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//       let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
    
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
//     println!("Hello, world!");
// }
