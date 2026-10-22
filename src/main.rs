//Made by Ilam
//All code is made by me
//This is used for making Secret Santas with your friends
//Have fun!

use rand::seq::SliceRandom;
use std::io::{self, BufRead, Write};

const END_KEYWORD: &str = "END";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Formatted {
    giver: String,
    to: String,
}

fn main() {
    println!();
    println!("!Enter one name per line. Type {END_KEYWORD} when you're done!");
    println!("Enter a list of people:");
    println!("{}", "-".repeat(50));

    let names = match read_names() {
        Ok(names) => names,
        Err(message) => {
            eprintln!("{message}");
            return;
        }
    };

    if names.len() < 2 {
        eprintln!("You need at least 2 people for a Secret Santa.");
        return;
    }

    let assignments = shuffle_assignments(assign_secret_santas(&names));
    reveal_assignments(&assignments);
}

fn read_names() -> Result<Vec<String>, String> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    read_names_from(&mut reader)
}

fn read_names_from<R: BufRead>(reader: &mut R) -> Result<Vec<String>, String> {
    let mut names: Vec<String> = Vec::new();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader
            .read_line(&mut line)
            .map_err(|error| format!("Failed to read line: {error}"))?;

        if bytes_read == 0 {
            break;
        }

        let name = line.trim();
        if name.is_empty() {
            println!("Ignoring empty name.");
            continue;
        }
        if name.eq_ignore_ascii_case(END_KEYWORD) {
            break;
        }
        if names
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(name))
        {
            return Err(format!(
                "The name \"{name}\" was entered more than once. Names must be unique."
            ));
        }

        names.push(name.to_string());
    }

    Ok(names)
}

fn assign_secret_santas(names: &[String]) -> Vec<Formatted> {
    let mut givers: Vec<&String> = names.iter().collect();
    givers.shuffle(&mut rand::rng());

    givers
        .iter()
        .enumerate()
        .map(|(index, giver)| Formatted {
            giver: (*giver).clone(),
            to: givers[(index + 1) % givers.len()].clone(),
        })
        .collect()
}

fn shuffle_assignments(mut assignments: Vec<Formatted>) -> Vec<Formatted> {
    assignments.shuffle(&mut rand::rng());
    assignments
}

fn reveal_assignments(assignments: &[Formatted]) {
    println!();
    println!("Assignments are ready.");
    println!("Pass the computer to each person in turn.");
    println!("Only that person's recipient will be shown.");
    wait_for_enter("Press Enter to start.");

    for assignment in assignments {
        clear_screen();
        println!("Pass the computer to {}.", assignment.giver);
        wait_for_enter("Press Enter when they are ready to see their recipient.");

        clear_screen();
        println!();
        println!("{}, your Secret Santa recipient is:", assignment.giver);
        println!();
        println!("    {}", assignment.to);
        println!();
        wait_for_enter("Press Enter to hide this and pass the computer on.");
    }

    clear_screen();
    println!("All assignments have been revealed. Have fun!");
}

fn wait_for_enter(prompt: &str) {
    print!("{prompt} ");
    io::stdout().flush().expect("Failed to flush stdout.");
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
}

fn clear_screen() {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status();
    }
    #[cfg(not(windows))]
    {
        print!("\x1b[2J\x1b[3J\x1b[H");
        io::stdout().flush().expect("Failed to flush stdout.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn names(count: usize) -> Vec<String> {
        (0..count).map(|index| format!("Person{index}")).collect()
    }

    #[test]
    fn reads_names_until_end_case_insensitively() {
        let input = b"Alice\n\n  Bob  \nend\nCarol\n";
        let names = read_names_from(&mut &input[..]).unwrap();

        assert_eq!(names, vec!["Alice", "Bob"]);
    }

    #[test]
    fn eof_finishes_input() {
        let input = b"Alice\nBob\n";
        let names = read_names_from(&mut &input[..]).unwrap();

        assert_eq!(names, vec!["Alice", "Bob"]);
    }

    #[test]
    fn rejects_duplicate_names_case_insensitively() {
        let input = b"Alice\nalice\nEND\n";
        let error = read_names_from(&mut &input[..]).unwrap_err();

        assert!(error.contains("more than once"));
    }

    #[test]
    fn assigns_one_to_one_without_self_assignment() {
        for count in 2..=10 {
            for _ in 0..200 {
                let people = names(count);
                let assignments = assign_secret_santas(&people);

                assert_eq!(assignments.len(), count);

                let mut recipient_by_giver: HashMap<String, String> = HashMap::new();
                for assignment in &assignments {
                    assert_ne!(assignment.giver, assignment.to);
                    assert!(
                        recipient_by_giver
                            .insert(assignment.giver.clone(), assignment.to.clone())
                            .is_none()
                    );
                }

                let unique_recipients: HashSet<&String> = assignments
                    .iter()
                    .map(|assignment| &assignment.to)
                    .collect();
                assert_eq!(unique_recipients.len(), count);
            }
        }
    }

    #[test]
    fn assignment_is_a_single_cycle() {
        for count in 2..=10 {
            for _ in 0..200 {
                let people = names(count);
                let assignments = assign_secret_santas(&people);
                let recipient_by_giver: HashMap<String, String> = assignments
                    .into_iter()
                    .map(|assignment| (assignment.giver, assignment.to))
                    .collect();

                let start = people[0].clone();
                let mut current = start.clone();
                for _ in 0..count - 1 {
                    current = recipient_by_giver
                        .get(&current)
                        .expect("every giver should have a recipient")
                        .clone();
                }

                assert_eq!(recipient_by_giver.get(&current), Some(&start));
            }
        }
    }

    #[test]
    fn two_people_swap() {
        let people = names(2);
        let assignments: HashMap<String, String> = assign_secret_santas(&people)
            .into_iter()
            .map(|assignment| (assignment.giver, assignment.to))
            .collect();

        assert_eq!(assignments.get("Person0"), Some(&"Person1".to_string()));
        assert_eq!(assignments.get("Person1"), Some(&"Person0".to_string()));
    }

    #[test]
    fn shuffle_preserves_assignments() {
        let people = names(8);
        let assignments = assign_secret_santas(&people);
        let mut expected = assignments.clone();
        expected.sort_by(|left, right| left.giver.cmp(&right.giver));

        let shuffled = shuffle_assignments(assignments);
        let mut actual = shuffled.clone();
        actual.sort_by(|left, right| left.giver.cmp(&right.giver));

        assert_eq!(actual, expected);
        assert_eq!(shuffled.len(), people.len());
    }
}
