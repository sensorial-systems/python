use ensta::Client;

fn main() {
    let client = Client::new();
    println!("{}", client.biography_of("notdanilo"));
}
