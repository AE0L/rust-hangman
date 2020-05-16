extern crate rand;

use std::io;
use std::process;
use rand::*;

pub struct Game {
    pub lives: u8,
    pub word: String,
    pub guess: Vec<String>,
    pub left: Vec<Letter>,
}

#[derive(Debug)]
pub struct Letter(char, usize);

impl Letter {
    fn new(&self) -> Self {
        Letter(self.0, self.1)
    }
}

impl Game {
    pub fn instance() -> Game {
        let lives = 6;
        let word = Game::random_word();

        let guess = word.chars()
            .map(|_| String::from("_"))
            .collect::<Vec<String>>();

        let left: Vec<Letter> = word.chars()
            .enumerate()
            .map(|(i, c)| {
                Letter(c, i)
            }).collect();

        Game {
            lives,
            word,
            guess,
            left
        }
    }

    pub fn random_word() -> String {
        let words = include_str!("words.txt");
        let words: Vec<&str> = words.split("\n").collect();
        let rand_index = rand::thread_rng().gen_range(1, words.len());
        let rand_word = words[rand_index];

        rand_word.trim().to_string()
    }

    pub fn run(&mut self) {
        while self.lives > 0 && self.left.len() > 0 {
            self.tick()
        }

        self.draw();

        println!("Word: {}", self.word);
    }

    pub fn tick(&mut self) {
        self.draw();

        let mut letter = self.ask_letter();

        if letter.len() > 1 {
            println!("\nToo many letters!");
            println!("Press <Enter> key to continue...");
            io::stdin().read_line(&mut letter).unwrap();
            return;
        }

        if self.is_correct_letter(&letter) {
            self.remove_from_left(&letter);
        } else {
            self.decrease_live();
        }
    }

    pub fn draw(&self) {
        clr_scrn();
        self.print_lives();
        self.print_guess();
        println!();
    }

    pub fn is_correct_letter(&self, letter: &str) -> bool {
        self.word.contains(letter)
    }

    pub fn remove_from_left(&mut self, letter: &str) {
        let mut removed = Vec::new();

        self.left = self.left.iter()
            .filter(|l| {
                let character = letter.chars().next().unwrap();

                if l.0 == character {
                    removed.push(l.new());
                    return false
                }

                true
            })
            .map(|l| l.new())
            .collect();

            for letter in removed.iter() {
                self.update_guess(letter);
            }
    }

    pub fn update_guess(&mut self, letter: &Letter) {
        let chr = letter.0;
        let pos = letter.1;

        self.guess = self.guess.iter()
            .enumerate()
            .map(|(i, c)| {
                if i == pos {
                    chr.to_string()
                } else {
                    c.to_string()
                }
            }).collect();
    }

    pub fn ask_letter(&self) -> String {
        let mut letter = String::new();

        println!("Enter letter:");

        match io::stdin()
            .read_line(&mut letter) {
                Err(_) => {
                    println!("Error reading input!");
                    process::exit(1);
                },
                Ok(_) => ()
            }

        letter.trim().to_string()
    }

    pub fn print_guess(&self) {
        println!("{}", self.guess.join(" "));
    }

    pub fn decrease_live(&mut self) {
        self.lives -= 1;
    }

    pub fn print_lives(&self) {
        if self.lives == 6 {
            println!("
  ┌─────┐
  │     │
        │
        │
        │
        │
        │
        │
   ─────┴─────
")
        } else if self.lives == 5 {
            println!("
  ┌─────┐
  │     │
 ┌┴┐    │
 └┬┘    │
        │
        │
        │
        │
   ─────┴─────
")
        } else if self.lives == 4 {
            println!("
  ┌─────┐
  │     │
 ┌┴┐    │
 └┬┘    │
  ┼     │
  │     │
        │
        │
   ─────┴─────
")
        } else if self.lives == 3 {
            println!("
  ┌─────┐
  │     │
 ┌┴┐    │
 └┬┘    │
──┼     │
  │     │
        │
        │
   ─────┴─────
")
        } else if self.lives == 2 {
            println!("
  ┌─────┐
  │     │
 ┌┴┐    │
 └┬┘    │
──┼──   │
  │     │
        │
        │
   ─────┴─────
")
        } else if self.lives == 1 {
            println!("
  ┌─────┐
  │     │
 ┌┴┐    │
 └┬┘    │
──┼──   │
  │     │
 /      │
        │
   ─────┴─────
")
        } else if self.lives == 0 {
            println!("
  ┌─────┐
  │     │
 ┌┴┐    │
 └┬┘    │
──┼──   │
  │     │
 / \\    │
        │
   ─────┴─────
")
        }
    }
}

pub fn clr_scrn() {
    print!("{}[2J", 27 as char);
}

