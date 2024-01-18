mod puzzle1;
mod puzzle2;
mod puzzle3;

fn main() {
    let answer1 = puzzle1::answer();
    println!("Answer to puzzle #1 (Part two): {answer1}");

    let (answer2_1, answer2_2) = puzzle2::answer();
    println!("Answer to puzzle #2 (Part one): {answer2_1}");
    println!("Answer to puzzle #2 (Part two): {answer2_2}");

    let (answer3_1, answer3_2) = puzzle3::answer();
    println!("Answer to puzzle #3 (Part one): {answer3_1}");
    println!("Answer to puzzle #3 (Part two): {answer3_2}");
}
