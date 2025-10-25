# Memory Game

A simple memory game built with Rust using the `egui` and `eframe` GUI frameworks. Challenge your memory by recalling sequences of numbers or letters, sometimes in reverse!

## Features

- Interactive GUI powered by `egui` and `eframe`
- Randomly generated sequences using the `rand` crate
- Multiple game phases: Not Started, Showing Sequence, Inputting, Game Over, Success
- Supports both numbers and letters as sequence elements
- Input sequences in forward or reverse order for added challenge

## Gameplay

1. **Start the Game:**
   Click the start button to begin a new round.
2. **Memorize the Sequence:**
   A sequence of numbers or letters will be displayed briefly.
3. **Input the Sequence:**
   After the sequence disappears, input what you remember. Sometimes you'll be asked to enter it in reverse!
4. **Win or Try Again:**
   If you recall the sequence correctly, you win! Otherwise, try again to improve your memory.

## Controls

- Use your keyboard to input the sequence.
- Follow on-screen prompts for the required input direction (FORWARD or REVERSE).

## Building and Running

1. **Install Rust:**
   [Get Rust and Cargo](https://www.rust-lang.org/tools/install)

2. **Clone the Repository:**
   ```sh
   git clone <repo-url>
   cd memory_game
