/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::{BTreeMap, HashMap};

#[derive(Hash, PartialEq, Debug)]
struct User {
    name: String,
    password: String,
    email: String,
}

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    let bf: BTreeMap<_, _> = frequencies.into_iter().collect();

    for (&num, frequency) in bf.iter() {
        println!("{num}");
        result.push((num, *frequency as u32));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );

    let mut user_hm: HashMap<String, User> = HashMap::new();

    user_hm.insert(
        "james_bond".to_string(),
        User {
            name: "Bond".to_string(),
            password: "007".to_string(),
            email: "007@mi6.gov.uk".to_string(),
        },
    );

    user_hm.insert(
        "Isaev".to_string(),
        User {
            name: "Max Otto von Stierlitz".to_string(),
            password: "hitlerkaput".to_string(),
            email: "stierlitz@deutsch.reich".to_string(),
        },
    );

    user_hm.iter().for_each(|(u, v)| {
        println!("User: {}", &u);
        println!("name: {} pass {} email {}", &v.name, &v.password, &v.email);
        println!("--------------");
    });
}
