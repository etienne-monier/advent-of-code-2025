use combinatorial::Combinations;
use std::{
    collections::HashSet,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct Machine {
    // The desired final light pattern.
    pub light_pattrn: Vec<bool>,
    // The available buttons
    pub buttons: Vec<HashSet<usize>>,
    // The joltage
    pub joltage: Vec<i32>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine {{light_pattrn: {:?}, buttons: {:?}, joltage: {:?}}}",
            self.light_pattrn, self.buttons, self.joltage
        )
    }
}

impl Machine {
    // Return the minimum number of button press to have the right light pattern.
    pub fn min_button_light(&self) -> usize {
        let mut sum = 1000; // Large enough so that the first matching is less that that.

        let buttons_as_vec: Vec<Vec<usize>> = self
            .buttons
            .iter()
            .map(|v| v.iter().cloned().collect())
            .collect();

        // We can use only one button, or two, ... until all -1 (required by the combination function)
        for n in 1..self.buttons.len() {
            // We are using n buttons. We need to have all combinations of n buttons
            let mut combos = Combinations::of_size(&buttons_as_vec, n);

            while let Some(combo) = combos.next() {
                // combo is a vec of button presses.
                // Start fresh state for this combo
                let mut state = vec![false; self.light_pattrn.len()];

                // Press all buttons
                for button in combo.iter() {
                    for &idx in button.iter() {
                        state[idx] = !state[idx];
                    }
                }

                // Compare reached state to goal
                if state == self.light_pattrn {
                    // The button sequence is ok.
                    // Check if this is optimal.
                    if combo.len() < sum {
                        sum = combo.len()
                    }
                }
            }
        }

        sum
    }

    // Return the minimum number of button press to have the right joltage.
    // fn min_button_joltage(&self) -> usize {
    //     let mut sum = 1000; // Large enough so that the first matching is less that that.

    //     // We don't need more than the joltage sum. The worst case would consist in pushing only
    //     // buttons that increment only one digit at a time.
    //     let max_button_presses: i32 = self.joltage.iter().sum();
    //     // println!("max: {}", max_button_presses);

    //     // We can use only one button, or two, ... until all -1 (required by the combination function)
    //     for n in 1..=max_button_presses as usize {
    //         // We are using n buttons. We need to have all combinations of n buttons

    //         for combo in CombinationsWithReplacement::new(&self.buttons.clone(), n) {
    //             // combo is a vec of button presses.
    //             // Start fresh state for this combo
    //             let mut state = vec![0; self.joltage.len()];
    //             // println!("combo: {:?}", combo);

    //             // Press all buttons
    //             for button in combo.iter() {
    //                 for &idx in button.iter() {
    //                     let i = idx as usize;
    //                     state[i] += 1;
    //                 }
    //             }

    //             // println!("state: {:?}, joltage: {:?}", state, self.joltage);
    //             // Compare reached state to goal
    //             if state == self.joltage {
    //                 // The button sequence is ok.
    //                 // Check if this is optimal.
    //                 if combo.len() < sum {
    //                     // println!("{:?}", combo);
    //                     sum = combo.len()
    //                 }
    //             }
    //         }
    //     }

    //     sum
    // }

    // Return the minimum number of button press to have the right joltage.
    pub fn min_button_joltage_bis(&self) -> i32 {
        let mut res = 0;
        let num_dial = self.joltage.len();

        // The vector that is incremented while pushing buttons
        let mut state: Vec<i32> = vec![0; num_dial];

        // The buttons that can be pressed
        let mut remaining_button = self.buttons.clone();

        // REMOVE DUPLICATE BUTTONS e.g. (1), (2) and (1, 2)

        // Get indexes where there is a single button to increment.
        let mut tmp_arr = vec![0; num_dial];
        for button in &remaining_button {
            for &index in button {
                tmp_arr[index] += 1;
            }
        }

        // This is a vec of positions
        let mut single_button_data: Vec<(usize, usize)> = tmp_arr
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 1)
            .map(|(dial_pos, _)| {
                // The joltage dial at index `dial_pos` is incremented by a single button.
                // This button is the following
                let good_button_pos = self
                    .buttons
                    .iter()
                    .position(|button| button.iter().find(|v| **v == dial_pos).is_some())
                    .expect("There should be a button here.");

                // The joltage dial position, the good button position
                (dial_pos, good_button_pos)
            })
            .collect();

        // Sort from last-to-first button
        single_button_data.sort_by(|a, b| b.1.cmp(&a.1));

        // Press on single-status buttons
        for (dial_pos, button_pos) in single_button_data {
            let button = remaining_button.remove(button_pos);
            let number_of_press = self.joltage[dial_pos];
            for &idx in button.iter() {
                let i = idx as usize;
                state[i] += number_of_press;
            }

            res += number_of_press;
        }

        dbg!(state);
        dbg!(remaining_button);
        dbg!(res);

        res
    }
}
