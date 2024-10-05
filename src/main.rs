use eframe::egui;
use rand::Rng;

struct GuessingGame {
    secret_number: u32,
    guess: String,
    attempts: u32,
    score: u32,
    high_score: u32,
    message: String,
}

impl Default for GuessingGame {
    fn default() -> Self {
        Self {
            secret_number: rand::thread_rng().gen_range(1..=100),
            guess: String::new(),
            attempts: 0,
            score: 0,
            high_score: 0,
            message: String::from("Guess the number between 1 and 100!"),
        }
    }
}

// Implement the eframe::App trait
impl eframe::App for GuessingGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.message);
            ui.horizontal(|ui| {
                ui.label("Your Guess:");
                ui.text_edit_singleline(&mut self.guess);
            });

            if ui.button("Submit Guess").clicked() {
                if let Ok(num) = self.guess.trim().parse::<u32>() {
                    self.attempts += 1;

                    // Check the guess against the secret number
                    if num < self.secret_number {
                        self.message = "Too small!".to_string();
                    } else if num > self.secret_number {
                        self.message = "Too big!".to_string();
                    } else {
                        self.score = if self.attempts <= 10 {
                            100 - (self.attempts - 1) * 10
                        } else {
                            0
                        };

                        if self.score > self.high_score {
                            self.high_score = self.score;
                            self.message = format!(
                                "You guessed it! The number was {}. New high score: {}.",
                                self.secret_number, self.high_score
                            );
                        } else {
                            self.message = format!(
                                "You guessed it! The number was {}. Your score: {}. High score: {}.",
                                self.secret_number, self.score, self.high_score
                            );
                        }
                        // Reset game
                        self.reset_game();
                    }
                } else {
                    self.message = "Please enter a valid number!".to_string();
                }
            }
        });
    }
}

impl GuessingGame {
    fn reset_game(&mut self) {
        self.secret_number = rand::thread_rng().gen_range(1..=100);
        self.guess.clear();
        self.attempts = 0;
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };

    // Run the native application without using the `?` operator
    eframe::run_native("Guessing Game", options, Box::new(|_cc| Box::<GuessingGame>::default()));

    // Return statement not needed as run_native doesn't return a Result
}
