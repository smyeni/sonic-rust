fn main() {
    let farewell = String::from("Bye for now, Sifiso!");
    say_bye(farewell);   // ownership moves into the function
    println!("{}", farewell);   // uncomment me: the compiler says it's already gone
}

fn say_bye(msg: String) {
    println!("{msg}");
}   // msg dropped here: memory freed, no garbage collector needed
