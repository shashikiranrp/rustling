
fn main() {
  println!("Hello, World!");
  println!("Hello, {}!", "Earth");
  let mut greet: String = String::from("Mars");
  println!("Hello, {} from {}!", &greet, "Earth");
  greet = say_hello(greet);
  greet = say_hello(greet);
}

fn say_hello(name: String) -> String {
  println!("Saying hello to {}!", name);
  name
}
