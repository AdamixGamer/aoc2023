fn main() {
    let input = include_str!("test.txt");
    let result:Vec<&str> = input.split("\n").collect();
    println!("{result:?}");
    findnumber(result[0])
}
fn findnumber(input: &str) -> () {
    let a:Vec<&str> = input.split(char::is_alphabetic).collect();
    println!("{a:?}");
    ()
}