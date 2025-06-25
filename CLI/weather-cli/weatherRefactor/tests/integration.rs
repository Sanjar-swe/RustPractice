use weather_cli::api::get_weather_info;
use dotenv::dotenv;
use std::env;

#[test]
fn test_succesfull_city(){
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in .env");

    let result = get_weather_info("Tashkent", "UZ", &api_key);
    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
}

#[test]
fn test_city_not_found_404(){
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in .env");

    let result = get_weather_info("abCity", "DreamPlace", &api_key);

    assert!(result.is_err(), "Expected error, but got: {:?}", result.ok());
}
