use crate::shared;

pub async fn part1() -> i32 {
    let rez = shared::get_advent_of_code_input("1").await;
    let mut input = shared::map_input(rez, |value| value.parse::<i32>()).await;
    let mut counter = -1;
    let mut prev_value = -1;

    for value in input.iter_mut() {
        if *value > prev_value {
            counter += 1;
        }
        prev_value = *value;
    }
    return counter;
}
