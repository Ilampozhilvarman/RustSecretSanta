use rand::seq::SliceRandom;
use std::io;

const END_KEYWORD: String = String::from("END");

struct Refrence {
    refrence: Person,
}

struct Person {
    name: String,
    giving_to: Option<Box<Person>>,
}

fn main() {
    let mut list_of_people: Vec<Person> = Vec::new();
    println!("Print a list of people: \n");
    get_all_people_input(&END_KEYWORD, &mut list_of_people);
    list_of_people.shuffle(&mut rand::rng());
}

fn get_all_people_input(keyword: &str, lst: &mut Vec<Person>) {
    let mut over: bool = false;
    while !over {
        let mut person: String = String::new();
        io::stdin()
            .read_line(&mut person)
            .expect("Failed to read line.");

        if person.trim() == keyword {
            over = true;
        }

        lst.push(convert_str_to_person(person));
    }
}

fn convert_str_to_person(person_str: String) -> Person {
    Person {
        name: person_str,
        giving_to: None,
    }
}
