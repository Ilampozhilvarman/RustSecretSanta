use rand::seq::SliceRandom;
use std::io;

const END_KEYWORD: &str = "END";

struct Person {
    name: String,
    giving_to: Option<String>,
}

fn main() {
    println!();
    let mut list_of_people: Vec<Person> = Vec::new();
    println!("!Enter one name per line. Type {END_KEYWORD} when you're done!");
    println!("Print a list of people: ");
    println!("{}", "-".repeat(50));
    let succes = get_all_people_input(&END_KEYWORD, &mut list_of_people);
    if !succes {
        eprintln!("You inputed one empty name.");
        return;
    }
    if list_of_people.len() < 2 {
        eprintln!("You need at least 2 people for a Secret Santa.");
        return;
    }
    list_of_people.shuffle(&mut rand::rng());
    assign_all_secret_santas(&mut list_of_people);
    for p in &list_of_people {
        println!("{} -> {}", p.name, p.giving_to.as_ref().unwrap());
    }
}

fn get_all_people_input(keyword: &str, lst: &mut Vec<Person>) -> bool {
    loop {
        let mut person: String = String::new();
        io::stdin()
            .read_line(&mut person)
            .expect("Failed to read line.");

        let name = person.trim();
        if name == keyword {
            return true;
        } else if name.is_empty() {
            return false;
        }
        lst.push(convert_str_to_person(name.to_string()));
    }
}

fn convert_str_to_person(person_str: String) -> Person {
    Person {
        name: person_str,
        giving_to: None,
    }
}

fn assign_all_secret_santas(lst: &mut Vec<Person>) {
    let n = lst.len();
    for i in 0..n {
        lst[i].giving_to = Some(lst[(i + 1) % n].name.clone());
    }
}
