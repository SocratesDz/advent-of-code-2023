mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;

fn main() {
    let answer1 = puzzle1::answer();
    println!("Answer to puzzle #1 (Part two): {answer1}");

    let (answer2_1, answer2_2) = puzzle2::answer();
    println!("Answer to puzzle #2 (Part one): {answer2_1}");
    println!("Answer to puzzle #2 (Part two): {answer2_2}");

    let (answer3_1, answer3_2) = puzzle3::answer();
    println!("Answer to puzzle #3 (Part one): {answer3_1}");
    println!("Answer to puzzle #3 (Part two): {answer3_2}");

    let (answer4_1, answer4_2) = puzzle4::answer();
    println!("Answer to puzzle #4 (Part one): {answer4_1}");
    println!("Answer to puzzle #4 (Part two): {answer4_2}");

    let (answer5_1, answer5_2) = puzzle5::answer();
    println!("Answer to puzzle #5 (Part one): {answer5_1}");
    println!("Answer to puzzle #5 (Part two): {answer5_2}");
}
