use crate::shared;

pub async fn part1() -> isize {
    let rez = shared::get_advent_of_code_input("3").await;

    let input = shared::map_input(rez, |value| -> Result<Vec<char>, &str> {
        Ok(value.chars().collect::<Vec<char>>())
    });

    let binary = vec!["1"; input.first().unwrap().len()].join("");
    let sum_of_both: isize = isize::from_str_radix(&binary, 2).unwrap();

    let mut sum: Vec<i32> = vec![0; input.first().unwrap().len()];

    for bits in input.into_iter() {
        for (bit, poz) in bits.iter().zip(sum.iter_mut()) {
            match bit {
                '1' => *poz = *poz + 1,
                '0' => *poz = *poz - 1,
                _ => println!("PROBLEM"),
            }
        }
    }

    let result = sum
        .iter()
        .map(|value| -> &str {
            match value.is_positive() {
                true => "1",
                false => "0",
            }
        })
        .collect::<Vec<&str>>()
        .join("");

    let gamma_rate = isize::from_str_radix(&result, 2).unwrap();
    let epsilon_rate = sum_of_both - gamma_rate;

    return gamma_rate * epsilon_rate;
}
