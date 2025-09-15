mod card;
mod deck;
mod player;

use std::io;
use card::Card;
use deck::Deck;
use player::Player;

fn draw_a_card(deck: &mut Deck, player: &mut Player) {
    let mut cards = deck.deal(1);
    if let Some(card) = cards.pop() {
        player.hand.push(card);
        println!("{} drew a card", player.name);
    }
}

fn get_hand_value(hand: &Vec<Card>) -> u8 {
    hand.iter().map(|card| card.value).sum()
}

fn is_bust(hand: &Vec<Card>) -> bool {
    get_hand_value(hand) > 21
}

fn check_bust(player: &Player) -> bool {
    if is_bust(&player.hand) {
        println!("{} busted!", player.name);
        true
    } else {
        false
    }
}

fn check_21(hand: &Vec<Card>) -> bool {
    get_hand_value(hand) == 21
}

fn show_hand(player: &Player) {
    println!("{}'s hand: {:?}\n", player.name, player.hand);
}

fn dealer_turn(deck: &mut Deck, dealer: &mut Player) -> bool {
    if get_hand_value(&dealer.hand) < 17 {
        draw_a_card(deck, dealer);
    }
    if check_21(&dealer.hand) {
        println!("{} got 21!", dealer.name);
        false
    } else {
        println!("");
        check_bust(dealer)
    }
}

fn player_turn(deck: &mut Deck, player: &mut Player) -> bool {
    draw_a_card(deck, player);
    show_hand(player);
    if check_21(&player.hand) {
        println!("{} got 21!", player.name);
        false
    } else {
        check_bust(player)
    }
}

fn main() {
    println!("Welcom to Blackjack\n");
    let mut deck = Deck::new();
    deck.shuffle();

    let mut dealer = Player::new("Dealer".to_string());
    let mut player = Player::new("Player".to_string());
    draw_a_card(&mut deck, &mut player);
    show_hand(&player);
    draw_a_card(&mut deck, &mut dealer);
    show_hand(&dealer);

    fn check_21_winner(player: &Player, dealer: &Player) -> bool {
        if check_21(&player.hand) && check_21(&dealer.hand) {
            println!("Draw!");
            true
        } else if check_21(&player.hand) {
            println!("Player wins!");
            true
        } else if check_21(&dealer.hand) {
            println!("Dealer wins!");
            true
        } else {
            false
        }
    }

    let mut is_bust = false;
    loop {
        let mut input = String::new();
        println!("Do you want to hit or stand? (h/s)");
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "h" {
            if player_turn(&mut deck, &mut player) { 
                is_bust = true;
                println!("Dealer wins!");
                break; }
            if dealer_turn(&mut deck, &mut dealer) {
                is_bust = true;
                show_hand(&dealer);
                println!("Player wins!");
                break;
            } else if check_21_winner(&player, &dealer) {
                break;
            }
        } else if input == "s" {
            if dealer_turn(&mut deck, &mut dealer) {
                is_bust = true;
                show_hand(&dealer);
                println!("Player wins!");
                break;
            } else if check_21_winner(&player, &dealer) {
                break;
            }
            break;
        }
    }

    if !is_bust {
        while get_hand_value(&dealer.hand) < 17 {
            draw_a_card(&mut deck, &mut dealer);
        }
        if get_hand_value(&dealer.hand) > get_hand_value(&player.hand) {
            println!("Dealer wins!");
        } else if get_hand_value(&dealer.hand) < get_hand_value(&player.hand) {
            println!("Player wins!");
        } else {
            println!("Draw!");
        }
    }
}
