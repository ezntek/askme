/* Copyright 2023 ezntek (ezntek@xflymusic.com)
 *    
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http: *www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt;
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::Question;

    #[test]
    fn ask_output_correct() {
        let qn = Question::new("TestQuestion".to_string(), "correctanswer".to_string(), "What is the correct answer?".to_string(), 3);
        assert_eq!(qn.ask(), true)
    }
}

pub struct Question {
    pub permitted_tries: i32,
    pub title: String,
    pub prompt: String,
    pub correct_answer: String,
}

impl Question {
    pub fn new(title: String, correct_answer: String, prompt: String, permitted_tries: i32) -> Question {
        Question {
            permitted_tries, title, prompt, correct_answer
        }
    }

    pub fn ask(&self) -> bool {
        let stdin = std::io::stdin();
         
        let mut stdout = std::io::stdout();        
        let mut input_buffer = String::new();
        
        for i in 1..=self.permitted_tries {
            input_buffer.clear();
            stdout.write(std::format!("{} ", self.prompt).as_bytes()).unwrap();
            stdout.flush().unwrap();
            stdin.read_line(&mut input_buffer).expect("Some error happened while reading the string!");

            if input_buffer.trim() == self.correct_answer {
                return true;
            } else if i < self.permitted_tries {
                if i > 1 {
                    println!("Your answer was wrong. You have {} more tries.", self.permitted_tries - i);
                } else {
                    println!("Your answer was wrong. You have 1 more try.");
                }
            } else {
                break;
            }
        }
        false
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Question \"{}\"", self.prompt)
    }
}