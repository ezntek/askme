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
use std::fs;
use std::path::Path;
use std::io::Write;
use serde_derive::{Deserialize,Serialize};

#[cfg(test)]
mod tests {
    use super::{Question,Questions};
/*
    #[test]
    fn get_qn_string() {
        static STR: &str = "
        permitted_tries = 3
        title = 'hello'
        prompt = 'hello?'
        correct_answer = 'hi'
        ";
        let qn = Question::from_string(STR.to_string());
        assert_eq!(qn, Question::new("hello".to_string(), "hi".to_string(), "hello?".to_string(), 3));
    }

    #[test]
    fn into_qn_string() {
        static STR: &str = "permitted_tries = 3
title = \"hello\"
prompt = \"hello?\"
correct_answer = \"hi\"
"; // funny syntax bc newlines suck
        let qn = Question::new("hello".to_string(), "hi".to_string(), "hello?".to_string(), 3);
        assert_eq!(STR, qn.into_string());
    }
*/
    #[test]
    fn test_file_out() {
        let qns = Questions::new(vec![Question::new("hello".to_string(), "hi".to_string(), "hello?".to_string(), 3), Question::new("hello".to_string(), "hi".to_string(), "hello?".to_string(), 3)]);
        qns.into_file("weewoo.toml".to_string());
    }

    #[test]
    fn test_file_in() {
        let qns = Questions::new(vec![Question::new("hello".to_string(), "hi".to_string(), "hello?".to_string(), 3), Question::new("hello".to_string(), "hi".to_string(), "hello?".to_string(), 3)]);
        assert_eq!(Questions::from_file("weewoo.toml".to_string()), qns);
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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

    pub fn from_string(s: String) -> Question {
        toml::from_str(&s).unwrap()
    }

    pub fn into_string(&self) -> String {
        toml::to_string(&self).expect("failed to serialize the struct!")
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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Questions {
    pub question: Vec<Question>
}

impl Questions {
    pub fn new(question: Vec<Question>) -> Questions {
        Questions {
            question
        }
    }

    pub fn from_file(file_name: String) -> Questions {
        toml::from_str(&fs::read_to_string(file_name).expect("There was an error reading the file! perhaps you mistyped the file name?")).expect("")
    }

    pub fn into_file(&self, file_name: String) {
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file = match std::fs::File::create(&path) {
            Err(reason) => panic!("Failed to create file {}: {}", display, reason),
            Ok(f) => f,
        };

        match file.write_all((toml::to_string(&self).expect("Failed to serialize the struct!")).as_bytes()) {
            Err(reason) => panic!("Failed to write file {}: {}", display, reason),
            Ok(_) => println!("Wrote file {} successfuly.", display),
        }
    }
}