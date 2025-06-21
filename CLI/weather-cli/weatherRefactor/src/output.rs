use crate::models::WeatherResponse;
use colored::*;

pub fn display_weather_info(response: &WeatherResponse, format: &str){
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
    // println!("{}", weather_text_colored);

    match format {
        "json" => {
            let json: String = serde_json::to_string_pretty(&response).unwrap();
            println!("{}", json);
        },
        "plain" => {
            println!("{}", weather_text);
        },
        "human" => {
            println!("{}", weather_text_colored);
        }
        _ => {
            // Если формат неизвестен, выводим plain
            println!("{}", weather_text);
        }
    }
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