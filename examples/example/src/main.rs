use ensta::Client;

fn main() {
    let client = Client::new();
    println!("{}", client.profile_picture_of("leomessi"));
}
