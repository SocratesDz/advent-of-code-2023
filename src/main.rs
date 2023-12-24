mod puzzle1;
mod puzzle2;

fn main() {
    let answer1 = puzzle1::answer();
    println!("Answer to puzzle #1 (Part two): {answer1}");

    let (answer2_1, answer2_2) = puzzle2::answer();
    println!("Answer to puzzle #2 (Part one): {answer2_1}");
    println!("Answer to puzzle #2 (Part two): {answer2_2}");
}
