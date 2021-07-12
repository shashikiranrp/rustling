fn main() {
    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn remove_vowels(name: String) -> (String, String) {
    // Goal #1: What is needed here to make this compile?
    // >>> Add `mut' modifier to `output' variable
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels
            }
            _ => {
                output.push(c);
            }
        }
    }
    (output, name)
}

fn print_out(name: String) {
    let (devowelized_name, new_name) = remove_vowels(name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // Goal #2: What happens when you uncomment the `println` below?
    // Can you change the code above so that the code below compiles
    // successfully?
    //
     println!("Removing vowels from {:?} yields {:?}",
              new_name, devowelized_name);

    // Extra credit: Can you do it without copying any data?
    // (Using only ownership transfer)
}
