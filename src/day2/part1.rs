use crate::shared;

pub async fn part1() -> i128 {
    let rez = shared::get_advent_of_code_input("2").await;

    let input = shared::map_input(rez, |value: String| -> Result<(String, i128), &str> {
        let mut parts = value.split(" ").into_iter();
        let direction = parts.next().unwrap().to_string();
        let string_steps = parts.next();
        if string_steps == None {
            return Err("Empty");
        }
        let steps = string_steps.unwrap().parse::<i128>().unwrap();
        let rez = (direction, steps);
        return Ok(rez);
    })
    .await;

    let mut depth: i128 = 0;
    let mut horizont: i128 = 0;

    for (direction, steps) in input {
        match &*direction {
            "forward" => horizont = horizont + steps,
            "backward" => horizont = horizont - steps,
            "up" => depth = depth - steps,
            "down" => depth = depth + steps,
            &_ => println!("PROBLEM"),
        }
    }

    return depth * horizont;
}
