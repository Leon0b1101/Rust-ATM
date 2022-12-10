#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[derive(Debug)]
pub struct User {
    CardNumber: String,
    Name: String,
    Pin: i32,
    Balance: u32,
}

impl User {
    pub fn new(card_number: &str, name: &str, pin: i32, balance: u32) -> User {
        User {
            CardNumber: card_number.to_string(),
            Name: name.to_string(),
            Pin: pin,
            Balance: balance
        }
    }
    pub fn getCardNumber(&self) -> &String {
        &self.CardNumber
    }
    pub fn getName(&self) -> &String {
        &self.Name
    }
    pub fn getPin(&self) -> &i32 {
        &self.Pin
    }
    pub fn getBalance(&self) -> &u32 {
        &self.Balance
    }
    pub fn setCardNumber(&mut self, card_number: &str) {
        self.CardNumber = card_number.to_string();
    }
    pub fn setName(&mut self, name: &str) {
        self.Name = name.to_string();
    }
    pub fn setPin(&mut self, pin: &i32) {
        self.Pin = *pin;
    }
    pub fn setBalance(&mut self, balance: u32) {
        self.Balance = balance;
    }
}