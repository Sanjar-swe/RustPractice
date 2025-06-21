mod api;
mod models;
mod output;
mod cli;
use clap::Parser;
use cli::Args;


use dotenv;
use colored::*;

use crate::api::get_weather_info;
use crate::output::display_weather_info;



fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let api_key = dotenv::var("API_KEY").expect("API_KEY must be set");
    let country_code = &args.country;
    let response = get_weather_info(&args.city, &country_code, &api_key).unwrap();
    display_weather_info(&response, &args.format);
    
   

    println!("{}", "Welcome to Weather Station!".bright_yellow());

    
}
