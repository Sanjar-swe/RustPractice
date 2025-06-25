use crate::models::WeatherResponse;

const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
const UNITS: &str = "metric";

pub fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let full_url = format!(
        "{base}?q={city},{country}&appid={key}&units={units}",
        base = BASE_URL,
        city = city,
        country = country_code,
        key = api_key,
        units = UNITS
    );
    let response = reqwest::blocking::get(&full_url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}