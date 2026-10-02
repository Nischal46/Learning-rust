pub fn init() {
    let weekly_weather_forecast = [12, 23, 34, 25, 11, 17, 28];

    // Finding the lowest and highest temps
    let lowest = weekly_weather_forecast.iter().min().unwrap();
    let highest = weekly_weather_forecast.iter().max().unwrap();

    println!("Lowest temp: {}", lowest);
    println!("Highest temp: {}", highest);
}
