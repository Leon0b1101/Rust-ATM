#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
mod user;
use user::User;

use std::io;

pub struct ATM {}

impl ATM {
    pub fn printOptions() {
        println!("\n 1. Deposit \n 2. Withdraw\n 3. Balance\n 4. Exit\n");
    }
    pub fn deposit(user: &mut User) {
        let mut input = String::new();
        println!("Enter the balance that should be deposited:  \n");
        io::stdin().read_line(&mut input).expect("Error reading deposit amount");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 1,
        };
        let newBalance = user.getBalance() + input;
        &user.setBalance(newBalance);
        println!("Balance: {}", &user.getBalance());
    }
    pub fn withdraw(user: &mut User) {
        let mut amount = String::new();
        println!("Enter the balance that should be withdraw:  \n");
        io::stdin().read_line(&mut amount).expect("Error reading withdraw amount");
        let amount: u32 = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => 1,
        };
        if user.getBalance() < &amount {
            println!("Balance insufficient!");
            return;
        }
        let newBalance = user.getBalance() - amount;
        &user.setBalance(newBalance); 
        println!("Balance: {}", &user.getBalance());
    }
    pub fn showBalance(user: &User) {
        println!("Your balance is {}", user.getBalance());
    }
}

fn main() {
    let mut admin = User::new("1234512345", "Leon", 12345, 123);
    loop {
        ATM::printOptions();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading choice");
        match choice.to_string().trim() {
            "1" => ATM::deposit(&mut admin),
            "2" => ATM::withdraw(&mut admin),
            "3" => ATM::showBalance(&mut admin),
            "4" => break,
            &_ => println!("{}", choice)
        }
    }
}