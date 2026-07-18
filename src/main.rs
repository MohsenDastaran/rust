#[derive(Debug)]
enum  CardSuit {
    Heart, Diamond, Spades, Clubs
}
struct Card {
    rank: String,
    suit: CardSuit
}
fn main() {
    let mut first_card = CardSuit::Diamond;

    // first_card = "sdc"            //expected CardSuit, found &'static str
    println!("{:#?}", first_card);

}
