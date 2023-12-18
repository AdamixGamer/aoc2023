fn main() {
    let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    let result: Vec<&str> = input.split("\n").collect();
    //println!("{result:?}");
    println!("{}", correctnumber(result[1]));
}
fn correctnumber(input: &str) -> i32 {
    println!("{input}");
    let mut y: i32 = 0;
    let t: Vec<&str> = input.split(";").collect();
    let length = t.len();
    for i in 0..length {
        let mut u: Vec<&str> = t[i].split(",").collect();
        let lenght2 = u.len();
        for i2 in 0..lenght2 {
            let onecolor: Vec<&str> = u.pop().unwrap().split(" ").collect();
            let mut ballsnumber = 0;
            if onecolor[1] == "Game" {
                y = onecolor[2].trim_matches(':').parse().unwrap();
                println!("{y}");
            } else {
                println!("{}", onecolor[1]);
                ballsnumber = onecolor[1].trim_matches(':').parse::<i32>().unwrap();
            }
            if onecolor[2] == "green" && ballsnumber < 14 {
                continue;
            } else if onecolor[2] == "red" && ballsnumber < 13 {
                continue;
            } else if onecolor[2] == "blue" && ballsnumber < 15 {
                continue;
            } else {
                return 0;
            }
        }
    }
    if y != 0{
        return y;
    }
    return 0;
}
