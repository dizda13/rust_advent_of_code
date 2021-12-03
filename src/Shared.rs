use reqwest::Error;
use std::env;

pub async fn get_advent_of_code_input(day: &str) -> Result<String, Error> {
    let token = match env::var("ADVENT_TOKEN") {
        Ok(token) => token,
        Err(e) => panic!("{}", e),
    };
    let request_url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let client = reqwest::Client::new();
    let _response = client
        .get(request_url)
        .header("Cookie", format!("{}{}", "session=", &token))
        .send()
        .await?
        .text()
        .await?;

    Ok(_response)
}

pub async fn map_input<T, E: std::fmt::Debug>(
    response: Result<String, Error>,
    mapper: fn(String) -> Result<T, E>,
) -> Vec<T> {
    let input = response
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .map(mapper)
        .filter(|value| value.is_ok())
        .map(|value| value.unwrap())
        .collect::<Vec<T>>();

    return input;
}
