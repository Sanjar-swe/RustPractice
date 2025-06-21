use crate::models::WeatherResponse;

const API_URL: &str = "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}";

pub fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
    let full_url= format!(API_URL, 
    city, country_code, api_key
    );
    let response: reqwest::blocking::get(&full_url);
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}