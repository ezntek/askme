pub struct Question {
    pub permitted_tries: i32,
    pub title: String,
    pub correct_answer: String,
}

impl Question {
    pub fn new(title: String, correct_answer: String, permitted_tries: i32) -> Question {
        Question {
            permitted_tries: permitted_tries,
            title: title.to_string(),
            correct_answer: correct_answer.to_string(),
        }
    }
}