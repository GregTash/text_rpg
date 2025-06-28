mod player;
mod enemy;
use crate::player::Player;
use crate::enemy::Enemy;

use std::process::exit;

use getch_rs::{Getch,Key};

fn main() {
    let mut player = player::Player::new();
    let mut enemy = enemy::Enemy::new();

    loop {
        hub(&mut player, &mut enemy);
    }
}

fn hub(player: &mut Player, enemy: &mut Enemy) {
    println!("Welcome back!");
    println!("What would you like to do?");
    println!("1) Fight");
    println!("2) View Stats");
    println!("3) Exit");
    println!();

    let key = Getch::new();

    match key.getch() {
        Ok(Key::Char('1')) => combat(player, enemy),
        Ok(Key::Char('2')) => player.display_stats(),
        Ok(Key::Char('3')) => exit(0),
        Ok(_key) => print!(""),
        Err(e) => println!("{e}"),
    }
}

fn combat(player: &mut Player, enemy: &mut Enemy) {
    let key = Getch::new();
    enemy.health = 20;

    loop {
        enemy.display_stats();

        println!("What would you like to do?");
        println!("1) Attack");
        println!("2) View Stats");
        println!();

        match key.getch() {
            Ok(Key::Char('1')) => enemy.damage(player.damage),
            Ok(Key::Char('2')) => player.display_stats(),
            Ok(_key) => print!(""),
            Err(e) => println!("{e}"),
        }

        player.damage(enemy.damage);

        if enemy.health <= 0 {
            break;
        }

        if player.health <= 0 {
            exit(0);
        }
    }
}