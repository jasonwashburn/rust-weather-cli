use colored::{ColoredString, Colorize};
use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct ZipResponse {
    weather: Vec<WeatherResponse>,
    main: MainResponse,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    description: String,
}

#[derive(Deserialize, Debug)]
struct MainResponse {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    humidity: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let zip_code: i32 = match args[1].parse::<i32>() {
        Ok(zip) => zip,
        Err(_) => panic!("First argument must be a valid float!"),
    };

    let api_key = match std::env::var("OWM_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("No API Key Loaded!"),
    };

    let request_url = format!("https://api.openweathermap.org/data/2.5/weather?zip={zip_code},{country_code}&appid={api_key}",
                              zip_code = zip_code,
                              country_code = "us",
                              api_key = api_key);
    //    println!("{request_url}");

    let response = reqwest::get(&request_url).await?;
    //   println!("{:?}", &response);
    let weather_data: ZipResponse = response.json().await?;

    //    println!("{:?}", weather_data);
    println!(
        "\n\tConditions for zipcode: {}",
        zip_code.to_string().green()
    );

    let conditions = &weather_data.weather[0].description;

    println!("\tDescription:\t\t{}", colorize_conditions(&conditions));
    println!(
        "\tCurrent Temperature:\t{}",
        colorize_temperature(&convert_k_to_f(weather_data.main.temp))
    );
    println!(
        "\tCurrent Feels Like:\t{}",
        colorize_temperature(&convert_k_to_f(weather_data.main.feels_like))
    );
    println!("\tCurrent Humidity:\t{}%", &weather_data.main.humidity);
    println!(
        "\tToday's Low/High:\t{:.0}/{:.0}",
        convert_k_to_f(weather_data.main.temp_min),
        convert_k_to_f(weather_data.main.temp_max)
    );
    Ok(())
}

fn convert_k_to_f(temperature: f32) -> f32 {
    (temperature - 273.15) * (9.0 / 5.0) + 32.0
}

fn colorize_temperature(temperature: &f32) -> ColoredString {
    let formatted_temperature = format!("{:.2}", temperature);

    if temperature >= &100.0 {
        formatted_temperature.red()
    } else if temperature >= &85.0 {
        formatted_temperature.bright_red()
    } else if temperature >= &70.0 {
        formatted_temperature.truecolor(255, 128, 0)
    } else if temperature >= &55.0 {
        formatted_temperature.bright_green()
    } else if temperature >= &40.0 {
        formatted_temperature.cyan()
    } else if temperature >= &0.0 {
        formatted_temperature.blue()
    } else if temperature < &0.0 {
        formatted_temperature.bright_blue()
    } else {
        formatted_temperature.normal()
    }
}

fn colorize_conditions(conditions: &String) -> ColoredString {
    if conditions == "clear sky" {
        conditions.bright_cyan()
    } else {
        conditions.normal()
    }
}
