use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        let mut upper_bound = max;
        let mut lower_bound = min;
        let mut midpoint = (upper_bound + lower_bound)/2;
        for i in min..max {
            let mut guess = player.ask_to_compare(midpoint);
            if guess == 0 {
                return midpoint;
            } else if guess == -1 {
                upper_bound = midpoint + 1;
                midpoint = upper_bound / 2;
            } else if guess == 1 {
                lower_bound = midpoint - 1;
                midpoint = (lower_bound + upper_bound) / 2;
            }
        }
        return 0;
    }
}
