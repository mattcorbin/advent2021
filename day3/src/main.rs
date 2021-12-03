use std::fs;
use std::cmp::Ordering;

fn calculate_most_common(input: &Vec<&str>) ->  [char; 12] {
    let mut values: [[usize; 2]; 12] = [[0; 2]; 12];
    let mut most_common: [char; 12] = ['0'; 12];
    for line in input {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '0' => {values[idx][0] += 1},
                '1' => {values[idx][1] += 1},
                _ => panic!("unexpected number")
            }
        }
    }
    for i in 0..values.len() {
        match values[i][0].cmp(&values[i][1]) {
            Ordering::Less => most_common[i] = '1',
            Ordering::Equal => most_common[i] = '1',
            Ordering::Greater =>  most_common[i] = '0',
        }
    }
    most_common
}

fn calculate_least_common(input: &Vec<&str>) ->  [char; 12] {
    let mut values: [[usize; 2]; 12] = [[0; 2]; 12];
    let mut most_common: [char; 12] = ['0'; 12];
    for line in input {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '0' => {values[idx][0] += 1},
                '1' => {values[idx][1] += 1},
                _ => panic!("unexpected number")
            }
        }
    }
    for i in 0..values.len() {
        match values[i][0].cmp(&values[i][1]) {
            Ordering::Less => most_common[i] = '0',
            Ordering::Equal => most_common[i] = '0',
            Ordering::Greater =>  most_common[i] = '1',
        }
    }
    most_common
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    let mut values: [[usize; 2]; 12] = [[0; 2]; 12];
    for line in input.split("\n") {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '0' => {values[idx][0] += 1},
                '1' => {values[idx][1] += 1},
                _ => panic!("unexpected number")
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for value in values {
        match value[0].cmp(&value[1]) {
            Ordering::Less => {
                gamma.push('1');
                epsilon.push('0');
            },
            Ordering::Equal => panic!("should not be equal"),
            Ordering::Greater => {
                gamma.push('0');
                epsilon.push('1');
            },
        }
    }

    let mut most_common = calculate_most_common(&input.split("\n").collect());
    let mut available_oxygen: Vec<&str> = input.split("\n").collect();
    let oxygen: String;
    let mut current_idx: usize = 0;
    loop {
        let mut new_available: Vec<&str> = Vec::new();
        for line in available_oxygen {
            if line.as_bytes()[current_idx] as char == most_common[current_idx] {
                new_available.push(line);
            }
        }
        if new_available.len() == 1 {
            oxygen = new_available[0].to_string();
            break
        } else {
            current_idx += 1;
            most_common = calculate_most_common(&new_available);
            available_oxygen = new_available.clone();
        }
    }

    let mut least_common:[char; 12] = calculate_least_common(&input.split("\n").collect());
    let mut available_carbon_dioxide: Vec<&str> = input.split("\n").collect();
    let carbon_dioxide: String;
    let mut current_idx: usize = 0;
    loop {
        let mut new_available: Vec<&str> = Vec::new();
        for line in available_carbon_dioxide {
            if line.as_bytes()[current_idx] as char == least_common[current_idx] {
                new_available.push(line);
            }
        }
        if new_available.len() == 1 {
            carbon_dioxide = new_available[0].to_string();
            break
        } else {
            current_idx += 1;
            least_common = calculate_least_common(&new_available,);
            available_carbon_dioxide = new_available;
        }
    }

    let gamma_decimal = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_decimal = usize::from_str_radix(&epsilon, 2).unwrap();    
    let oxygen_decimal = usize::from_str_radix(&oxygen, 2).unwrap();
    let carbon_dioxide_decimal = usize::from_str_radix(&carbon_dioxide, 2).unwrap();
    println!("power consumption: {}", gamma_decimal * epsilon_decimal);
    println!("life support: {}", oxygen_decimal * carbon_dioxide_decimal);
}
