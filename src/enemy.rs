pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub damage: u32,
}

impl Enemy {
    pub fn new() -> Enemy {
        let new_enemy = Enemy {
            name: String::from("Wolf"),
            health: 20,
            damage: 10,
        };

        new_enemy
    }

    pub fn display_stats(&self) { 
        println!("Enemy: {}", self.name);
        println!("Health: {}", self.health);
        println!("Damage: {}", self.damage);
    }

    pub fn damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }
}