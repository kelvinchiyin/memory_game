use eframe::egui;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
enum CharType {
    Number(u8),
    Letter(u8),
}

impl CharType {
    fn as_char(&self) -> char {
        match self {
            CharType::Number(n) => *n as char,
            CharType::Letter(l) => *l as char,
        }
    }

    fn from_char(c: char) -> Self {
        if c.is_ascii_digit() {
            CharType::Number(c as u8)
        } else {
            CharType::Letter(c.to_ascii_uppercase() as u8)
        }
    }
}

#[derive(Debug, Clone)]
enum GamePhase {
    NotStarted,
    ShowingSequence,
    Inputting,
    GameOver,
    Success,
}

struct MemoryGame {
    sequence: Vec<CharType>,
    user_input: Vec<CharType>,
    current_index: usize,
    phase: GamePhase,
    show_duration: std::time::Duration,
    error_message: Option<String>,
    sequence_display_timer: f32,
    current_sequence_index: usize,
    show_sequence: bool,
    normal_mode: bool,
    input_direction: InputDirection, // New field to track input direction
}

#[derive(Debug, Clone)]
enum InputDirection {
    Forward,
    Reverse,
}

impl MemoryGame {
    fn new() -> Self {
        Self {
            sequence: vec![],
            user_input: vec![],
            current_index: 0,
            phase: GamePhase::NotStarted,
            show_duration: std::time::Duration::from_millis(800),
            error_message: None,
            sequence_display_timer: 0.0,
            current_sequence_index: 0,
            show_sequence: false,
            normal_mode: true,
            input_direction: InputDirection::Forward, // Default value
        }
    }

    fn generate_sequence(&mut self, length: usize) {
        let mut rng = rand::thread_rng();
        self.sequence = Vec::with_capacity(length);

        for _ in 0..length {
            if rng.gen_bool(0.5) {
                // Generate a number (0-9)
                let num = rng.gen_range(b'0'..=b'9');
                self.sequence.push(CharType::Number(num));
            } else {
                // Generate a letter (A-Z)
                let letter = rng.gen_range(b'A'..=b'Z');
                self.sequence.push(CharType::Letter(letter));
            }
        }

        // Randomly decide input direction
        self.input_direction = if rng.gen_bool(0.5) {
            InputDirection::Forward
        } else {
            InputDirection::Reverse
        };
    }

    fn start_game(&mut self) {
        self.generate_sequence(5);
        self.user_input.clear();
        self.current_index = self.sequence.len();
        self.phase = GamePhase::ShowingSequence;
        self.error_message = None;
        self.sequence_display_timer = 0.0;
        self.current_sequence_index = 0;
        self.show_sequence = true;
    }

    fn handle_key_input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            for event in &i.raw.events {
                if let egui::Event::Key { key, pressed, modifiers: _, physical_key: _, repeat: _ } = event {
                    if *pressed {
                        match key {
                            egui::Key::A => self.process_input(CharType::Letter(b'A')),
                            egui::Key::B => self.process_input(CharType::Letter(b'B')),
                            egui::Key::C => self.process_input(CharType::Letter(b'C')),
                            egui::Key::D => self.process_input(CharType::Letter(b'D')),
                            egui::Key::E => self.process_input(CharType::Letter(b'E')),
                            egui::Key::F => self.process_input(CharType::Letter(b'F')),
                            egui::Key::G => self.process_input(CharType::Letter(b'G')),
                            egui::Key::H => self.process_input(CharType::Letter(b'H')),
                            egui::Key::I => self.process_input(CharType::Letter(b'I')),
                            egui::Key::J => self.process_input(CharType::Letter(b'J')),
                            egui::Key::K => self.process_input(CharType::Letter(b'K')),
                            egui::Key::L => self.process_input(CharType::Letter(b'L')),
                            egui::Key::M => self.process_input(CharType::Letter(b'M')),
                            egui::Key::N => self.process_input(CharType::Letter(b'N')),
                            egui::Key::O => self.process_input(CharType::Letter(b'O')),
                            egui::Key::P => self.process_input(CharType::Letter(b'P')),
                            egui::Key::Q => self.process_input(CharType::Letter(b'Q')),
                            egui::Key::R => self.process_input(CharType::Letter(b'R')),
                            egui::Key::S => self.process_input(CharType::Letter(b'S')),
                            egui::Key::T => self.process_input(CharType::Letter(b'T')),
                            egui::Key::U => self.process_input(CharType::Letter(b'U')),
                            egui::Key::V => self.process_input(CharType::Letter(b'V')),
                            egui::Key::W => self.process_input(CharType::Letter(b'W')),
                            egui::Key::X => self.process_input(CharType::Letter(b'X')),
                            egui::Key::Y => self.process_input(CharType::Letter(b'Y')),
                            egui::Key::Z => self.process_input(CharType::Letter(b'Z')),
                            egui::Key::Num0 => self.process_input(CharType::Number(b'0')),
                            egui::Key::Num1 => self.process_input(CharType::Number(b'1')),
                            egui::Key::Num2 => self.process_input(CharType::Number(b'2')),
                            egui::Key::Num3 => self.process_input(CharType::Number(b'3')),
                            egui::Key::Num4 => self.process_input(CharType::Number(b'4')),
                            egui::Key::Num5 => self.process_input(CharType::Number(b'5')),
                            egui::Key::Num6 => self.process_input(CharType::Number(b'6')),
                            egui::Key::Num7 => self.process_input(CharType::Number(b'7')),
                            egui::Key::Num8 => self.process_input(CharType::Number(b'8')),
                            egui::Key::Num9 => self.process_input(CharType::Number(b'9')),
                            egui::Key::Backspace | egui::Key::Delete => {
                                self.delete_last();
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    fn process_input(&mut self, input: CharType) {
        // Only process input when in Inputting phase
        if !matches!(self.phase, GamePhase::Inputting) {
            return;
        }

        self.user_input.push(input.clone());

        // Check if the input matches the expected character based on direction
        let expected_index = match self.input_direction {
            InputDirection::Forward => self.user_input.len() - 1, // 0-indexed for forward
            InputDirection::Reverse => self.sequence.len() - self.user_input.len(), // Reverse indexing
        };

        if expected_index < self.sequence.len() &&
           self.sequence[expected_index] != input {
            if self.normal_mode {
                // In Normal mode, just record the error but continue
                // Don't show the error message immediately - only show when sequence is complete
            } else {
                // In Strict mode, end the game immediately
                self.error_message = Some(format!(
                    "Wrong! Expected '{}' but got '{}'",
                    self.sequence[expected_index].as_char(),
                    input.as_char()
                ));
                self.phase = GamePhase::GameOver;
                return;
            }
        }

        // Check if the sequence is complete
        if self.user_input.len() == self.sequence.len() {
            // Check if all characters were correct
            let mut all_correct = true;

            for (i, user_char) in self.user_input.iter().enumerate() {
                let seq_index = match self.input_direction {
                    InputDirection::Forward => i,
                    InputDirection::Reverse => self.sequence.len() - i - 1, // Reverse order
                };

                if self.sequence[seq_index] != *user_char {
                    all_correct = false;
                    break;
                }
            }

            if all_correct {
                self.phase = GamePhase::Success;
            } else {
                // Show the error message only when the sequence is complete
                if !self.normal_mode {
                    // In strict mode, we already have an error message
                } else {
                    // In normal mode, show an error message
                    self.error_message = Some("Sequence is incorrect!".to_string());
                }
                self.phase = GamePhase::GameOver;
            }
        }
    }

    fn delete_last(&mut self) {
        // Only allow deletion when in Inputting phase
        if !matches!(self.phase, GamePhase::Inputting) {
            return;
        }

        if !self.user_input.is_empty() {
            self.user_input.pop();
            // Don't clear error message when deleting in Normal mode
            // Only clear error if we're in Strict mode and there are no more errors
            if !self.normal_mode {
                // In Strict mode, if we've deleted the incorrect character, clear error
                if self.user_input.len() > 0 {
                    let expected_index = match self.input_direction {
                        InputDirection::Forward => self.user_input.len() - 1,
                        InputDirection::Reverse => self.sequence.len() - self.user_input.len(),
                    };

                    if expected_index < self.sequence.len() {
                        let last_char = self.user_input.last().unwrap();
                        if self.sequence[expected_index] == *last_char {
                            self.error_message = None;
                        }
                    }
                } else {
                    self.error_message = None;
                }
            }
        }
    }
}

impl eframe::App for MemoryGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        self.handle_key_input(ctx);

        // Update the sequence display timer (only during showing sequence phase)
        if matches!(self.phase, GamePhase::ShowingSequence) {
            self.sequence_display_timer += ctx.input(|i| i.stable_dt).min(0.1);

            // Show each character for 0.8 seconds
            let chars_per_second = 1.0 / 0.8;
            self.current_sequence_index = (self.sequence_display_timer * chars_per_second) as usize;

            if self.current_sequence_index >= self.sequence.len() {
                // Finished showing sequence, move to input phase
                self.phase = GamePhase::Inputting;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.heading("Memory Game");

                // Mode selection - Normal mode first
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.normal_mode, true, "Normal");
                    ui.radio_value(&mut self.normal_mode, false, "Strict");
                });

                ui.separator();

                match &self.phase {
                    GamePhase::NotStarted => {
                        ui.heading("Click 'Start New Game' to begin!");

                        ui.label("Instructions:");
                        ui.label("1. Remember the sequence of numbers and letters shown on screen");
                        ui.label("2. Enter them in the required order when prompted");
                        ui.label("3. Use Backspace/Delete to remove the last entry if you make a mistake");
                    }

                    GamePhase::ShowingSequence => {
                        ui.heading("Remember the sequence:");

                        // Show only up to current_sequence_index
                        let mut seq_text = String::new();
                        for i in 0..=self.current_sequence_index.min(self.sequence.len()) {
                            if i < self.sequence.len() {
                                seq_text.push_str(&format!("{} ", self.sequence[i].as_char()));
                            }
                        }

                        ui.heading(&seq_text);

                        // Progress indicator
                        ui.label(format!(
                            "Showing sequence... {}/{}",
                            (self.current_sequence_index + 1).min(self.sequence.len()),
                            self.sequence.len()
                        ));
                    }

                    GamePhase::Inputting => {
                        let direction_text = match self.input_direction {
                            InputDirection::Forward => "forward order",
                            InputDirection::Reverse => "reverse order",
                        };

                        ui.heading(format!("Enter the sequence in {}:", direction_text));

                        // Show what the user has entered so far
                        let entered: String = self.user_input
                            .iter()
                            .map(|c| c.as_char().to_string())
                            .collect::<Vec<String>>()
                            .join(" ");

                        ui.heading(format!("Your input: [{}]", entered));

                        // Only show error in Strict mode during input
                        if !self.normal_mode {
                            if let Some(error) = &self.error_message {
                                ui.colored_label(egui::Color32::RED, error);
                            }
                        }

                        ui.label("Press the corresponding keys on your keyboard");
                        ui.label("Press Backspace/Delete to remove the last entry");

                        ui.separator();

                        // Show the remaining count
                        ui.label(format!(
                            "Characters remaining: {}",
                            self.sequence.len() - self.user_input.len()
                        ));
                    }

                    GamePhase::GameOver => {
                        ui.heading("Game Over!");

                        if let Some(error) = &self.error_message {
                            ui.colored_label(egui::Color32::RED, error);
                        }

                        // Show both sequences for comparison
                        ui.label("Your input:");
                        let user_seq: String = self.user_input
                            .iter()
                            .map(|c| c.as_char().to_string())
                            .collect::<Vec<String>>()
                            .join(", ");
                        ui.heading(&user_seq);

                        // Show the correct sequence based on input direction
                        let correct_seq: String = match self.input_direction {
                            InputDirection::Forward => {
                                self.sequence
                                    .iter()
                                    .map(|c| c.as_char().to_string())
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            }
                            InputDirection::Reverse => {
                                self.sequence
                                    .iter()
                                    .rev()
                                    .map(|c| c.as_char().to_string())
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            }
                        };

                        let direction_text = match self.input_direction {
                            InputDirection::Forward => "forward order",
                            InputDirection::Reverse => "reverse order",
                        };

                        ui.label(format!("Correct sequence (in {}):", direction_text));
                        ui.heading(&correct_seq);

                        if self.normal_mode {
                            ui.label("In Normal mode, the sequence was checked after you entered all characters.");
                        }
                    }

                    GamePhase::Success => {
                        let direction_text = match self.input_direction {
                            InputDirection::Forward => "forward",
                            InputDirection::Reverse => "reverse",
                        };

                        ui.heading("Congratulations!");
                        ui.colored_label(egui::Color32::GREEN, format!("You remembered the sequence correctly in {} order!", direction_text));

                        // Show the sequence for confirmation
                        ui.label("The sequence was:");
                        let correct_seq: String = self.sequence
                            .iter()
                            .map(|c| c.as_char().to_string())
                            .collect::<Vec<String>>()
                            .join(", ");
                        ui.heading(&correct_seq);
                    }
                }

                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    if ui.button("Start New Game").clicked() {
                        self.start_game();
                    }

                    // if matches!(self.phase, GamePhase::Inputting) {
                    //     if ui.button("Delete Last").clicked() {
                    //         self.delete_last();
                    //     }
                    // }
                });

                ui.add_space(20.0);

                ui.collapsing("Game Instructions", |ui| {
                    ui.label("1. Remember the sequence of numbers and letters shown on screen");
                    ui.label("2. Enter them in the required order when prompted");
                    ui.label("3. Use Backspace or Delete key to remove the last entry if you make a mistake");
                    ui.label("4. Be quick - you need to remember and enter the sequence correctly!");
                    ui.separator();
                    ui.label("Modes:");
                    ui.label("- Normal: Enter the entire sequence, errors are checked at the end");
                    ui.label("- Strict: Game ends immediately if you make a mistake");
                });
            });
        });

        // Request repaint for animation
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([300.0, 200.0])
            .with_title("Memory Game"),
        ..Default::default()
    };

    eframe::run_native(
        "Memory Game",
        options,
        Box::new(|_cc| Ok(Box::new(MemoryGame::new()))),
    )
}
