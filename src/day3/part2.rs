use crate::shared;

fn greater(highest_count_number: char, match_char: char) -> bool {
    if highest_count_number == 'n' {
        return match_char == '1';
    }

    highest_count_number == match_char
}

fn lower(highest_count_number: char, match_char: char) -> bool {
    if highest_count_number == 'n' {
        return match_char == '0';
    }

    highest_count_number != match_char
}

fn find_numbers(
    binary_numbers: &Vec<Vec<char>>,
    index: usize,
    filter: fn(a: char, b: char) -> bool,
) -> Vec<Vec<char>> {
    let result = binary_numbers
        .into_iter()
        .map(|binary| match binary[index] {
            '1' => 1,
            '0' => -1,
            _ => {
                println!("PROBLEM");
                0
            }
        })
        .reduce(|sum, binary| sum + binary)
        .unwrap();

    let filter_char = match result > 0 {
        true => '1',
        false => {
            if result < 0 {
                '0'
            } else {
                'n'
            }
        }
    };

    binary_numbers
        .into_iter()
        .filter(|binary| filter(filter_char, binary[index]))
        .cloned()
        .collect()
}

pub async fn part2() -> isize {
    let rez = shared::get_advent_of_code_input("3").await;

    let mut input = shared::map_input(rez, |value| -> Result<Vec<char>, &str> {
        Ok(value.chars().collect::<Vec<char>>())
    });

    let mut filter: Vec<Vec<char>> = input.clone();

    let mut index = 0;
    while filter.len() > 1 {
        filter = find_numbers(&filter, index, greater);
        index += 1;
    }

    let mut index = 0;
    while input.len() > 1 {
        input = find_numbers(&input, index, lower);
        index += 1;
    }

    let oxygen_generator_rating: isize =
        isize::from_str_radix(&filter[0].clone().into_iter().collect::<String>(), 2).unwrap();
    let co2_scrubber_rating: isize =
        isize::from_str_radix(&input[0].clone().into_iter().collect::<String>(), 2).unwrap();

    oxygen_generator_rating * co2_scrubber_rating
}
