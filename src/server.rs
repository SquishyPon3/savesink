// Temp constants for logging into a device
const PORT: &str = "";
const MACHINE: &str = "";
const USER: &str = "";
const DOMAIN: &str = "";


use std::net::TcpListener;
use cursive::reexports::time::ext;
use rpassword::read_password;
use secstr::SecStr;
use std::io::Write;

extern crate secstr;

// I want to get ssh working for communicating with savesink on another machine
// need to find a proper library for handling it.

pub fn start_server() {
    println!("Server Online!");
}

pub fn connect() {
    println!("Successfuly connected to {MACHINE}\\{USER}@{DOMAIN}!");
}