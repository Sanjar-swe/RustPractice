use crate::models::WeatherResponse;
use std::error::Error;

const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
const UNITS: &str = "metric";

pub fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let full_url = format!(
        "{base}?q={city},{country}&appid={key}&units={units}",
        base = BASE_URL,
        city = city,
        country = country_code,
        key = api_key,
        units = UNITS
    );
    let response = reqwest::blocking::get(&full_url)?;
    if !response.status().is_success(){
      return Err(format!("HTTP Error: {}", response.status()).into());
    }

    let weather = response.json::<WeatherResponse>()?;
       
    Ok(weather)
}