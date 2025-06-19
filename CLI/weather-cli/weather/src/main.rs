use std::io;
use serde::Deserialize;
use colored::*;
use dotenv;
//Struct to Deserialize the JSON response from openweatherMAP API
#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main, 
    wind: Wind, 
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather{
    description: String,
}

// Struct to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main{
    temp: f64, 
    humidity: f64,
    pressure: f64,
}



// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind{
    speed: f64,
}

//Function to get weather information from openweatherMAP API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
    let url= format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric", 
    city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"        // Снежинка
    } else if temperature < 10.0 {
        "🌥️"        // Облачно
    } else if temperature < 20.0 {
        "⛅"         // Солнце с облаком
    } else if temperature < 30.0 {
        "🌤️"        // Солнечно с лёгкой облачностью
    } else {
        "🔥"        // Жара
    }
}
// Function to display the weather information
fn display_weather_info(response: &WeatherResponse){
    // Extract the weather information from the response
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;
    // formatting weather information into a string
    let weather_text: String = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}C,
        > Humidity: {:.1}%,
        > Pressure: {:.1}hPa, 
        > Wind speed {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature, 
        humidity,
        pressure,
        wind_speed,        
    );
    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
    _   => weather_text.normal(),
    };
    // Print the colored weather information 
    println!("{}", weather_text_colored);
    // Function to get emoji based on temperature
}
fn main() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY not found in .env file");

    println!("{}", "Welcome to Weather Station!".bright_yellow());

    loop{
            // Reading City
            println!("{}", "Please enter the name of the City: ".bright_green());
            let mut city: String = String::new();
            io::stdin().read_line(&mut city).expect("Failed to read input !");
            let city: &str = city.trim();
            // exit
            if city.eq_ignore_ascii_case("exit") {
                println!("Goodbye 👋");
                break;
            }

            // Reading Country
            println!("{}", "Please enter the Country code: (e.g., US for United States):".bright_green());
            let mut country: String = String::new();
            io::stdin().read_line(&mut country).expect("Failed to read input !");
            let city: &str = city.trim();

            match get_weather_info(&city, &country, &api_key){
                Ok(response) => display_weather_info(&response),
                Err(err) => eprintln!("Error in getting weather information: {}", err),
            }
        }
    }
