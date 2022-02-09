
fn main() {
    println!("Please enter a calculation");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let parts: Vec<i32> = input.split(['+'])
        .map(|x| x.trim().parse().expect("Only numbers are allowed"))
        .collect();

    if parts.len() != 2 {
        panic!("Currently only + is a supported operator")
    }


    println!("Parts: {:?}", &parts);

    println!("{} + {} = {}", parts[0], parts[1], parts.iter().sum::<i32>())
}
