//#![allow(warnings)]
use std::collections::HashMap;
use std::io;
use std::collections::hash_map::RandomState;

fn say_hello() {
    println!("Hello!\nEnter your list of words: ");
}

fn get_input() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect(
        "Failed to get words!",
    );

    guess
}

fn create_hash_map(user_input: &String) -> HashMap<&str, i32, RandomState> {
    let mut map: HashMap<&str, i32, RandomState> = HashMap::new();

    for word in user_input.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    map
}

fn get_most_frequent(map: HashMap<&str, i32, RandomState>) -> String {
    let mut max_val = i32::MIN;
    let mut max_val_key = String::from("Nothing!");

    for (k, v) in map {
        if v > max_val {
            max_val = v;
            max_val_key = String::from(k);
        }
    }
    max_val_key
}

fn main() {
    say_hello();
    let mut user_input = String::from(get_input());
    user_input = user_input.replace(",", "");
    let hash_map = create_hash_map(&user_input);

    let most_frequent = get_most_frequent(hash_map);

    println!("Most frequent word is \"{}\"", most_frequent);

}