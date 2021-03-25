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

fn main() {
    say_hello();
    let mut user_input = String::from(get_input());
    user_input = user_input.replace(",", "");
    let hash_map = create_hash_map(&user_input);

    println!("{:?}", hash_map);

}