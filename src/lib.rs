use rand::Rng;
use std::collections::HashMap;
use std::io;

static TOTAL_DICE: i8 = 6;

// Define some utility functions.

/// Count dice values.
///
/// Returns a HashMap of die values and their counts.
///
/// #### Examples
/// ```
/// # use std::collections::HashMap;
/// let dice = vec![1, 1, 1, 2, 2, 3];
/// let counts = farkle::count_dice(&dice);
/// let expected = HashMap::<i16, i16>::from([(1, 3), (2, 2), (3, 1)]);
/// assert_eq!(expected, counts);
/// ```
pub fn count_dice(dice: &Vec<i16>) -> HashMap<i16, i16> {
    let mut map: HashMap::<i16, i16> = HashMap::new();
    for die in dice {
        let count = map.entry(*die).or_insert(0);
        *count += 1;
    }
    map
}

/// Returns true if dice contains 3 pairs.
///
/// ### Examples
/// ```
/// let three_pair = vec![1, 1, 2, 2, 3, 3];
/// assert_eq!(true, farkle::is_three_pair(&three_pair));
///
/// let not_three_pair = vec![1, 1, 2, 2, 3, 4];
/// assert_eq!(false, farkle::is_three_pair(&not_three_pair));
///
/// let four_of_a_kind = vec![1, 1, 2, 2, 2, 2];
/// assert_eq!(false, farkle::is_three_pair(&four_of_a_kind));
///
/// let all_ones = vec![1, 1, 1, 1, 1, 1];
/// assert_eq!(false, farkle::is_three_pair(&all_ones));
/// ```
pub fn is_three_pair(dice: &Vec<i16>) -> bool {
    let map = count_dice(dice);
    let mut pairs = 0;
    for (_die, count) in map {
        if count == 2 {
            pairs += 1;
        }
    }
    pairs == 3
}

/// Returns true if dice contains a straight.
///
/// ### Examples
/// ```
/// let dice = vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(true, farkle::is_straight(&dice));
///
/// let dice = vec![1, 2, 3, 4, 5, 5];
/// assert_eq!(false, farkle::is_straight(&dice));
///
/// let dice = vec![1, 4, 2, 6, 5, 3];
/// assert_eq!(true, farkle::is_straight(&dice));
/// ```
pub fn is_straight(dice: &[i16]) -> bool {
    for i in 1..=6 {
        if !dice.contains(&i) {
            return false;
        }
    }
    return true;
}

/// Returns true if dice contains num of a kind.
///
/// ### Examples
/// ```
/// let dice = vec![1, 1, 1, 1, 1, 1];
/// assert_eq!(true, farkle::is_of_a_kind(6, &dice));
///
/// let dice = vec![1, 1, 1, 1, 1, 2];
/// assert_eq!(false, farkle::is_of_a_kind(6, &dice));
///
/// let dice = vec![1, 1, 1, 1, 1, 2];
/// assert_eq!(true, farkle::is_of_a_kind(5, &dice));
///
/// let dice = vec![1, 1, 1, 1, 2, 2];
/// assert_eq!(false, farkle::is_of_a_kind(5, &dice));
///
/// let dice = vec![1, 1, 1, 1, 2, 2];
/// assert_eq!(true, farkle::is_of_a_kind(4, &dice));
///
/// let dice = vec![1, 1, 1, 2, 2, 2];
/// assert_eq!(false, farkle::is_of_a_kind(4, &dice));
///
/// let dice = vec![1, 1, 1, 2, 2, 2];
/// assert_eq!(true, farkle::is_of_a_kind(3, &dice));
/// ```
pub fn is_of_a_kind(num: i16, dice: &Vec<i16>) -> bool {
    let map = count_dice(dice);
    let mut of_a_kind = false;
    for (_die, count) in map {
        if count >= num {
            of_a_kind = true;
        }
    }
    of_a_kind
}

/// Returns true if dice contains 2 triplets.
///
/// ### Examples
/// ```
/// # use std::collections::HashMap;
/// let dice = vec![1, 1, 1, 2, 2, 2];
/// assert_eq!(true, farkle::is_two_triplets(&dice));
///
/// let dice = vec![1, 1, 1, 2, 2, 3];
/// assert_eq!(false, farkle::is_two_triplets(&dice));
///
/// let dice = vec![1, 1, 1, 1, 2, 2];
/// assert_eq!(false, farkle::is_two_triplets(&dice));
/// ```
pub fn is_two_triplets(dice: &Vec<i16>) -> bool {
    let map = count_dice(dice);
    let mut triplets = 0;
    for (_die, count) in map {
        if count >= 3 {
            triplets += 1;
        }
    }
    triplets == 2
}

/// Returns a vector of dice with count < 3.
///
/// ### Examples
/// ```
/// # use std::collections::HashMap;
/// let dice = vec![1, 1, 1, 2, 2, 2];
/// assert_eq!(Vec::<i16>::new(), farkle::strip_repeats(&dice));
///
/// let dice = vec![1, 1, 1, 2, 2, 3];
/// let mut expected = HashMap::new();
/// expected.insert(2, 2); // 2 appears twice
/// expected.insert(3, 1); // 3 appears once
/// let mut actual = HashMap::new();
/// for die in farkle::strip_repeats(&dice) {
///     let count = actual.entry(die).or_insert(0);
///     *count += 1;
///         }
/// assert_eq!(expected, actual);
///
/// let dice = vec![1, 1, 1, 1, 2, 2];
/// assert_eq!(vec![2, 2], farkle::strip_repeats(&dice));
/// ```
pub fn strip_repeats(dice: &Vec<i16>) -> Vec<i16> {
    let map = count_dice(dice);
    let mut new_dice = Vec::new();
    for (die, count) in map {
        if count < 3  {
            for _i in 0..count {
                new_dice.push(die);
            }
        }
    }
    new_dice
}

/// Returns a vector of dice that are 3 of a kind or more.
///
/// ### Examples
/// ```
/// # use std::collections::HashMap;
/// let dice = vec![1, 1, 1, 2, 2, 2];
/// let mut expected = HashMap::new();
/// expected.insert(2, 3); // 2 appears three times
/// expected.insert(1, 3); // 1 appears three times
/// let mut actual = HashMap::new();
/// for die in farkle::keep_repeats(&dice) {
///     let count = actual.entry(die).or_insert(0);
///     *count += 1;
///         }
/// assert_eq!(expected, actual);
///
/// let dice = vec![1, 1, 1, 2, 2, 3];
/// assert_eq!(vec![1, 1, 1], farkle::keep_repeats(&dice));
///
/// let dice = vec![1, 1, 1, 1, 2, 2];
/// assert_eq!(vec![1, 1, 1, 1], farkle::keep_repeats(&dice));
/// ```
pub fn keep_repeats(dice: &Vec<i16>) -> Vec<i16> {
    let map = count_dice(dice);
    let mut new_dice = Vec::new();
    for (die, count) in map {
        if count >= 3 {
            for _i in 0..count {
                new_dice.push(die);
            }
        }
    }
    new_dice
}

// Game logic
/// Returns the score for a given set of dice.
///
///
/// ### Examples
/// ```
/// let dice = vec![1, 1, 1, 2, 2, 2];
/// assert_eq!(2500, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 2, 2, 3, 3];
/// assert_eq!(1500, farkle::get_score(&dice));
///
/// let dice = vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(1500, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 1, 1, 1, 1];
/// assert_eq!(5000, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 1, 1, 1];
/// assert_eq!(3000, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 1, 1];
/// assert_eq!(2000, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 1];
/// assert_eq!(1000, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 1, 5];
/// assert_eq!(1050, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 1, 5, 5];
/// assert_eq!(1100, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 5, 5, 5];
/// assert_eq!(700, farkle::get_score(&dice));
///
/// let dice = vec![1, 1, 5];
/// assert_eq!(250, farkle::get_score(&dice));
/// ```
pub fn get_score(dice: &Vec<i16>) -> i16 {
    let mut score = 0;
    if is_two_triplets(dice) {
        return  2500;
    }
    else if is_three_pair(dice) {
        return  1500;
    }
    else if is_straight(dice) {
        return  1500;
    }
    else if is_of_a_kind(6, dice) {
        return 5000;
    }
    else if is_of_a_kind(5, dice) {
        score += 3000;
    }
    else if is_of_a_kind(4, dice) {
        score += 2000;
    }
    else if is_of_a_kind(3, dice) {
        let new_dice = keep_repeats(dice);
        if new_dice[0] == 1 {
            score += 1000;
        } else {
            score += new_dice[0] * 100;
            println!("3score: {}", score);
        }
    }
    // Score 1's and 5's.
    let new_dice = strip_repeats(dice);
    for die in new_dice {
        if die == 1 {
            score += 100;
            println!("beep {}", score);
        } else if die == 5 {
            score += 50;
        };
    }
    if score == 0 {
        println!("No scoring dice.\nYour turn is over.\n");
    }
    score
}

/// Ask if player wants to keep round score.
///
/// Returns true if player wants to keep score.
pub fn keep_score() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.contains("y") {
        return true;
    }
    if input.contains("q") {
        println!("Thanks for playing!");
        std::process::exit(0);
    }
    false
}

/// Get dice to keep from the user.
///
/// Returns a string of indices of dice to keep.
pub fn get_dice_to_keep() -> String {
    loop {
        let digits = "123456".to_string();
        let mut dice_to_keep = String::new();
        let mut valid_input = true;
        println!("Enter dice to keep (1-6):");
        // Get input from user.
        io::stdin().read_line(&mut dice_to_keep)
            .expect("Failed to read line");
        // Check if player wants to quit.
        if dice_to_keep.trim().contains("q") {
            println!("Thanks for playing!");
            std::process::exit(0);
        }
        println!("You entered: {}", dice_to_keep);
        // Check for repeated digits.
        let mut counts = HashMap::new();
        for c in dice_to_keep.trim().chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
            if *count > 1 {
                valid_input = false;
            }
        }
        if !valid_input {
            println!("You can't keep the same die twice.");
        }
        // Check for invalid digits.
        for c in dice_to_keep.trim().chars() {
            if !digits.contains(c) {
                println!("Invalid input {}. Try again.", c);
                valid_input = false;
            }
        }
    if !valid_input {
        continue;
    }
    return dice_to_keep;
    }
}

/// Returns a vector of dice to keep.
fn keep_dice(dice: Vec<i16>) -> Vec<i16> {
    // Get dice to keep from user.
    let input = get_dice_to_keep();

    // Create a mask of dice to keep.
    let mut keep_mask: Vec<bool> = Vec::new();
    for c in 1..dice.len() + 1 {
        if input.contains(&c.to_string()) {
            keep_mask.push(true);
        } else {
            keep_mask.push(false);
        };
    }
    // Filter dice, keeping values at indices that are true in keep_mask.
    let kept_dice: Vec<i16> = dice
        .iter()
        // Combine dice and keep_mask.
        .zip(keep_mask.iter())
        // Filter out dice that are not true in keep_mask.
        .filter(|(_dice, mask)| **mask)
        // Keep the values in dice.
        .map(|(dice, _mask)| *dice)
        .collect();
    println!("You kept: {:?}", kept_dice);
    kept_dice
}

/// Returns the score for a turn and the number of dice remaining.
fn turn(num_dice: i8) -> (i16, i8) {
    // num_dice is the number of dice to roll.
    let num_dice:i8 = num_dice;
    let mut dice: Vec<i16> = Vec::new();
    let mut rng = rand::thread_rng();

    // Roll dice.
    for _i in 0..num_dice {
        dice.push(rng.gen_range(1..=6));
    }
    println!("Dice: {:?}", dice);
    let keepers = keep_dice(dice);
    let score = get_score(&keepers);

    // Return score and number of dice to roll.
    (score, keepers.len() as i8)
}

/// Returns the score for a round.
///
/// Take turns in a loop until turn score or number of dice kept is 0.
pub fn round() -> i16 {
    let mut round_score = 0;
    let mut num_dice = TOTAL_DICE;
    loop {
        // Get score and number of dice to roll.
        let (turn_score, num_kept) = turn(num_dice);

        // No keepers or score == end of turn.
        if num_kept == 0 || turn_score == 0 {
            break;
        }
        // Calculate number of dice to roll for next turn.
        num_dice = if num_dice - num_kept <= 0 {
            println!("You got all keepers! Good job!\n");
            TOTAL_DICE
        } else {
            num_dice - num_kept
        };
        round_score += turn_score;
        // Ask if player wants to keep score.
        println!("Your score this round is {}\nWould you like to keep this score?", round_score);
        if keep_score() {
            return round_score;
        }
    }
// End of turn no keepers.
0
}
