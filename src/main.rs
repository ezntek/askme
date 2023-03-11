mod question;

fn main() {
    let qn = question::Question::new("damn".to_string(), "damn".to_string(), 3);
    println!("{}", qn.title);
}
