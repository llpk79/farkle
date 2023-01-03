//Program to play dice game Farkle from the command line.

use farkle::round;

static WELCOME_MESSAGE: &str = "
Welcome to Farkle! The rules are simple. You roll 6 dice and try to get
scoring combinations.

Scoring combinations are as follows:
1's: 100 points each
5's: 50 points each
3 of a kind: 1000 points for 3 ones, 200 for 3 twos, 300 for 3 threes, etc.
4 of a kind: 2000 points
5 of a kind: 3000 points
6 of a kind: 5000 points
3 pairs: 1500 points
straight: 1500 points
2 triplets: 2500 points

If you'd like to keep the 1st, 3rd, and 5th dice, you would type '135'.
You can roll as many times as you want, but if you
don't get any scoring combinations, you lose all your points for that turn.
You can bank your points at any time by entering 'y' instead of picking dice.
Reach 10,000 points to win!
Good luck!
";

fn main() {
    // Play a game of Farkle.
    println!("{}\n", WELCOME_MESSAGE);
    let mut score = 0;
    while score < 10_000 {
        let round_score = round();
        score += round_score;
        println!("Round score: {}", round_score);
        println!("Total score: {}\n", score);
    }
    println!("You win! Thanks for playing!");
}
