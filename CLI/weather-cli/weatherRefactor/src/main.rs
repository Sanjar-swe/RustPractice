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

    
   

    println!("{}", "Welcome to Weather Station!".bright_yellow());

    
}
