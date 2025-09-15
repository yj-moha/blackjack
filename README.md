# 🃏 Blackjack Game

A terminal-based Blackjack game written in Rust, featuring interactive gameplay, proper scoring, and dealer AI.

## ✨ Features

- **Interactive Gameplay**: Hit or stand with simple keyboard commands
- **Proper Blackjack Rules**: Standard casino rules implementation
- **Dealer AI**: Dealer follows house rules (hits on < 17, stands on ≥ 17)
- **Deck Management**: Full 52-card deck with proper shuffling
- **Win Detection**: Automatic bust detection and score comparison
- **Clean Architecture**: Modular design with separate components

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (2021 edition or later)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yj-moha/blackjack.git
cd blackjack
```

2. Build the project:
```bash
cargo build --release
```

3. Run the game:
```bash
cargo run -q
```

## 🎯 Game Rules

- Cards 1-13 represent their face values
- Player wins by getting closer to 21 than the dealer without busting
- Dealer must hit on hands < 17 and stand on hands ≥ 17
- Bust occurs when hand total exceeds 21
- Ties result in a draw

## 🎮 How to Play

1. **Start**: The game deals one card each to you and the dealer
2. **Your Turn**: Choose to hit (`h`) or stand (`s`)
   - **Hit**: Draw another card
   - **Stand**: Keep your current hand
3. **Dealer's Turn**: Dealer automatically plays according to house rules
4. **Scoring**: 
   - Get as close to 21 as possible without going over
   - Bust if your hand exceeds 21
   - Beat the dealer's hand to win!

### Game Flow Example
```
Welcome to Blackjack

Player's hand: [Card { value: 8, suit: Heart }]

Dealer's hand: [Card { value: 10, suit: Spade }]

Do you want to hit or stand? (h/s)
h
Player drew a card
Player's hand: [Card { value: 8, suit: Heart }, Card { value: 5, suit: Club }]

Do you want to hit or stand? (h/s)
s
Dealer wins!
```

## 📁 Project Structure

```
src/
├── main.rs     # Game loop and core logic
├── card.rs     # Card struct and Suit enum
├── deck.rs     # Deck management and shuffling
└── player.rs   # Player struct and hand management
```

## 🛠️ Dependencies

- [`rand`](https://crates.io/crates/rand) `0.9.2` - For deck shuffling


