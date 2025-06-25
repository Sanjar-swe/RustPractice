mod api;
mod models;
mod output;
mod cli;

use clap::Parser;
use dotenv;
use colored::*;

use crate::api::get_weather_info;
use crate::output::display_weather_info;
use cli::Args;

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    // println!("{:?}", args);

    let api_key = match &args.api_key {
        Some(k)=> k.clone(),
        None => dotenv::var("API_KEY")
            .map_err(|_| "API_KEY must be set via --api-key or in .env")?,
    };

    let response = get_weather_info(&args.city, &args.country, &api_key)?;

    display_weather_info(&response, &args.format);

    println!("{}", "Welcome to Weather Station!".bright_yellow());

    Ok(())
}