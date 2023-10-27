use std::collections::HashMap;

fn get_letter_to_plant_hashmap<'a>() -> HashMap<char, &'a str> {
    let mut hashmap = HashMap::new();

    ["radishes", "clover", "grass", "violets"]
        .into_iter()
        .for_each(|s| {
            let first_letter = s.chars().nth(0).unwrap().to_uppercase().nth(0).unwrap();
            hashmap.insert(first_letter, s);
        });

    hashmap
}

fn get_children_list<'a>() -> Vec<&'a str> {
    vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ]
}

pub fn plants<'a>(_diagram: &'a str, _student: &str) -> Vec<&'a str> {
    // todo!("Solve kindergarten-garden exercise");
    let rows: Vec<&'a str> = _diagram.split("\n").collect();
    let row1: Vec<char> = rows[0].chars().collect();
    let row2: Vec<char> = rows[1].chars().collect();
    let children = get_children_list();
    let child_index = children
        .into_iter()
        .enumerate()
        .find(|(_i, child)| *child == _student)
        .unwrap()
        .0
        * 2;
    let letter_to_plant_hashmap = get_letter_to_plant_hashmap();
    let plants: Vec<&str> = [
        row1[child_index],
        row1[child_index + 1],
        row2[child_index],
        row2[child_index + 1],
    ]
    .into_iter()
    .map(|c| letter_to_plant_hashmap.get(&c))
    .flatten()
    .map(|c| *c)
    .collect();
    plants
}
