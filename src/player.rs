use std::io;

pub struct Player {
    pub name: String,
    pub health: u32,
    pub damage: u32,
}

impl Player {
    pub fn new() -> Player {
        let mut _input = String::new();

        println!("Enter your character's name: ");
        io::stdin()
            .read_line(&mut _input)
            .expect("Could not read input...");
        println!();

        let new_player = Player {
            name: _input.trim().to_string(),
            health: 100,
            damage: 10,
        };
        
        new_player
    }

    pub fn display_stats(&self) {
        println!("Name: {}", self.name);
        println!("Health: {}", self.health);
        println!("Damage: {}", self.damage);
        println!();
    }
    
    pub fn damage(&mut self, amount: u32) {
        self.health -= amount;
    }
}