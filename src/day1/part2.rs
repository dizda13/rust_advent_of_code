use crate::shared;

pub async fn part2() -> i32 {
    let rez = shared::get_advent_of_code_input("1").await;

    let input = shared::map_input(rez, |value| return value.parse::<i32>()).await;

    let mut counter = -1;
    let mut prev_value = -1;
    for n in 0..(input.len() - 2) {
        let value = input.get(n).unwrap() + input.get(n + 1).unwrap() + input.get(n + 2).unwrap();
        if value > prev_value {
            counter += 1;
        }
        prev_value = value;
    }

    return counter;
}
