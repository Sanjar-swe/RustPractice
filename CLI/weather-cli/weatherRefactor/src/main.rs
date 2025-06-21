mod api;
mod models;
mod output;
mod cli;
use clap::Parser;
use cli::Args;


use std::io;
use dotenv;
use colored::*;

use crate::api::get_weather_info;
use crate::output::display_weather_info;



fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let api_key = match &args.api_key{
    Some(k) => k.to_string(),
    None => std::env::var("API_KEY").expect("API_KEY not set")
    };
    dotenv::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY not found in .env file");

    println!("{}", "Welcome to Weather Station!".bright_yellow());

    
}
