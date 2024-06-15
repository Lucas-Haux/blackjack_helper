use deckofcards::*;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let mut dealer = Hand::new();
    let mut hand1 = Hand::new();
    let mut hand2 = Hand::new();
    let mut hand3 = Hand::new();

    println!(" {}", hand1);

}

