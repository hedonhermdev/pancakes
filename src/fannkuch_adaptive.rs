use rayon::prelude::*;
use std::{
    cmp,
    mem::{self, replace},
    os::macos::raw::stat,
};

const MAX_N: usize = 16;

struct State {
    count: [usize; MAX_N],
    current_perm: [u8; MAX_N],
    new: bool,
}

impl State {
    pub fn new() -> Self {
        let state = Self {
            count: [0; MAX_N],
            current_perm: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            new: true,
        };
        state
    }
}

pub fn fannkuch_adaptive(n: usize) -> (i32, i32) {
    let factorial_lookup_table = {
        let mut table: [usize; MAX_N] = [0; MAX_N];
        table[0] = 1;
        for i in 1..MAX_N {
            table[i] = i * table[i - 1];
        }
        table
    };

    (0..factorial_lookup_table[n])
        .into_par_iter()
        .adaptive(50)
        .scan(|| State::new(), |state, k| {
            if state.new {
                state.new = false;
                let mut temp_perm: [u8; MAX_N] = [0; MAX_N];
                let mut permutation_index = k;
                for i in (1..n).rev() {
                    let f = factorial_lookup_table[i];
                    let d = permutation_index / f;

                    state.count[i] = d;

                    // Rotate the permutation left by d places. This is faster
                    // than using slice::rotate_left.
                    temp_perm[0..=i - d].copy_from_slice(&state.current_perm[d..=i]);
                    temp_perm[i - d + 1..=i].copy_from_slice(&state.current_perm[..d]);
                    state.current_perm = temp_perm;

                    permutation_index = permutation_index % f;
                }
            }

            let mut flip_count = 1;
            let mut checksum = 0;

            if state.current_perm[0] > 0 {
                // Make a copy of current_permutation[] to work on.
                let mut temp_permutation = state.current_perm;

                // Flip temp_permutation until the element at the
                // first_value index is 1 (0).
                let mut first_value = state.current_perm[0] as usize & 0xF;
                while temp_permutation[first_value] > 0 {
                    // Record the new_first_value and restore the old
                    // first_value at its new flipped position.
                    let new_first_value =
                        replace(&mut temp_permutation[first_value], first_value as u8);

                    // If first_value is greater than 3 (2) then we are
                    // flipping a series of four or more values so we will
                    // also need to flip additional elements in the middle
                    // of the temp_permutation.
                    if first_value > 2 {
                        for (low_index, high_index) in (1..first_value).zip((1..first_value).rev())
                        {
                            temp_permutation.swap(high_index, low_index);

                            if low_index + 3 > high_index {
                                break;
                            }
                        }
                    }

                    // Update first_value to new_first_value that we
                    // recorded earlier.
                    first_value = new_first_value as usize & 0xF;
                    flip_count += 1;
                }
            }

            if k % 2 == 0 {
                checksum += flip_count;
            } else {
                checksum -= flip_count;
            }

            state.current_perm.swap(0, 1);
            let mut first_value = state.current_perm[0];
            for i in 1..MAX_N - 2 {
                state.count[i] += 1;
                if state.count[i] <= i {
                    break;
                }
                state.count[i] = 0;

                let new_first_value = state.current_perm[1];

                for j in 0..i + 1 {
                    state.current_perm[j] = state.current_perm[j + 1];
                }

                state.current_perm[i + 1] = first_value;
                first_value = new_first_value;
            }

            Some((checksum, flip_count))
        })
        .reduce(|| (0, 0), |(cs1, mf1), (cs2, mf2)| (cs1 + cs2, cmp::max(mf1, mf2)))
        
    // let max_flip_count = flip_counts.par_iter().max().unwrap();

    // let checksum = flip_counts.iter().enumerate().fold(0, |chksum, (pos, x)| {
    //     if pos % 2 == 0 {
    //         chksum + x
    //     } else {
    //         chksum - x
    //     }
    // });

    // (checksum, *max_flip_count)
}

mod tests {
    use super::fannkuch_adaptive;

    #[test]
    fn test_adaptive() {
        assert_eq!(fannkuch_adaptive(7), (228, 16));
    }
}
