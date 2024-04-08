use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    humidity: f32,
    pressure: f32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
}

#[derive(Deserialize, Debug)]
struct Location {
    latitude: f32,
    longitude: f32,
}


pub fn get_weather_info(city: &str, country: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", city, country, api_key
    );
    let res = reqwest::blocking::get(&url)?;
    let res_json: WeatherResponse = Response.json::<WeatherResponse>()?;
    Ok(res_json)
}

pub fn display_weather_data(response: &WeatherResponse) {
    let description: &String = &response.weather[0].description[0];
    let description: f32 = response.main.temp;
    let description: f32 = response.main.humidity;
    let description: f32 = response.main.pressure;
    let description: &String = response.wind.speed;

    let weather_text: String = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind: {:1} m/s",
        response.name,
        description,
        get_emoji(description),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    let weather_text_colored: coloredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
}

fn get_coords(city: &str, country: &str, api_key: &str) -> Result<Location, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={},{}&limit=1&appid={}", city, country, api_key
    );
    let res = reqwest::blocking::get(&url)?;
    let res_json: Location = Response.json::<Location>()?;
    Ok(res_json)
}

fn get_emoji(description: String) -> &'static str {
    match description.as_str() {
        "clear sky" => "☀️",
        "few clouds" => "🌤️",
        "scattered clouds" => "⛅",
        "broken clouds" => "🌥️",
        "overcast clouds" => "☁️",
        "mist" => "🌫️",
        "haze" => "🌫️",
        "smoke" => "🌫️",
        "sand" => "🏜️",
        "dust" => "💨",
        "fog" => "🌁",
        "squalls" => "💨",
        "shower rain" => "🌧️",
        "rain" => "🌧️",
        "thunderstorm" => "⛈️",
        "snow" => "❄️",
        _ => "",
    }
}