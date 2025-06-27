mod player;
use std::process::{exit, ExitCode};

use getch_rs::{Getch,Key};

use crate::player::Player;

fn main() {
    let mut player = player::Player::new();

    loop {
        hub(&mut player);
    }
}

fn hub(player: &mut Player) {
    println!("Welcome back!");
    println!("What would you like to do?");
    println!("1) Fight");
    println!("2) View Stats");
    println!("3) Exit");
    println!();

    let key = Getch::new();

    match key.getch() {
        Ok(Key::Char('1')) => println!("WIP!\n"),
        Ok(Key::Char('2')) => player.display_stats(),
        Ok(Key::Char('3')) => exit(0),
        Ok(_key) => print!(""),
        Err(e) => println!("{e}"),
    }
}