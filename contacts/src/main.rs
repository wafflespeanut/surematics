use std::io::{self, Write};
use std::str;

fn get_input(s: &str) -> Result<String, ()> {
    print!("{}", s);
    let _ = io::stdout().flush();
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    if line.is_empty() {
        Err(())
    } else {
        Ok(line)
    }
}

fn get_contacts() -> Result<(), ()> {
    let fields = get_input("Enter space-separated list of fields...\n(for e.g., name, phone, etc.)\n\nFields: ")?;
    let fields = fields.split(" ").map(str::to_lowercase).collect::<Vec<_>>();
    let mut contacts = vec![];
    loop {      // loop until valid choice is made (or until empty input, which breaks the loop)
        let choice = get_input("\n\t1. Add new contact\n\t2. List contacts\n\nChoice: ")?;
        let choice = match choice.parse::<u8>() {
            Ok(v) if v > 0 && v <= 2 => v,
            _ => {
                println!("Please enter a valid choice.");
                continue
            },
        };

        if choice == 1 {
            let mut contact = vec![];
            for field in &fields {
                let value = get_input(&format!("Enter value for {}: ", field))?;
                contact.push(value);
            }

            contacts.push(contact);
        } else {
            let mut idx = 0;
            loop {      // loop until valid field name is given (or value is skipped for default)
                let prompt = format!("Enter field name for sorting (default: {}): ", &fields[0]);
                if let Ok(f) = get_input(&prompt) {
                    let lower = f.to_lowercase();
                    idx = match fields.iter().position(|v| v == &*lower) {
                        Some(i) => i,
                        None => {
                            println!("Field does not exist!");
                            continue
                        },
                    };
                }

                break
            }

            println!("");
            // sorting every time doesn't have any impact on the output
            contacts.sort_by(|v_1, v_2| v_1[idx].cmp(&v_2[idx]));
            for (i, contact) in contacts.iter().enumerate() {
                println!("{}. {}", i + 1, contact.join("\t"));
            }
        }
    }
}

fn main() {
    let _ = get_contacts();
}
