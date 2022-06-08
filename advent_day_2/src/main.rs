fn main() {
    // initialize array
    let initial_state = [
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 9, 23, 27,
        2, 27, 6, 31, 1, 5, 31, 35, 2, 9, 35, 39, 2, 6, 39, 43, 2, 43, 13, 47, 2, 13, 47, 51, 1,
        10, 51, 55, 1, 9, 55, 59, 1, 6, 59, 63, 2, 63, 9, 67, 1, 67, 6, 71, 1, 71, 13, 75, 1, 6,
        75, 79, 1, 9, 79, 83, 2, 9, 83, 87, 1, 87, 6, 91, 1, 91, 13, 95, 2, 6, 95, 99, 1, 10, 99,
        103, 2, 103, 9, 107, 1, 6, 107, 111, 1, 10, 111, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1,
        123, 13, 127, 1, 127, 5, 131, 1, 6, 131, 135, 2, 135, 13, 139, 1, 139, 2, 143, 1, 143, 10,
        0, 99, 2, 0, 14, 0,
    ];

    for noun in 0..100 {
        for verb in 0..100 {
            let mut state = initial_state;
            state[1] = noun;
            state[2] = verb;
            let result = run_program(state);
            if result == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn run_program(mut state: [u32; 153]) -> u32 {
    let mut i = 0;
    while state[i] != 99 {
        if state[i] == 1 {
            state[state[i + 3] as usize] =
                state[state[i + 1] as usize] + state[state[i + 2] as usize];
        } else if state[i] == 2 {
            state[state[i + 3] as usize] =
                state[state[i + 1] as usize] * state[state[i + 2] as usize];
        } else {
            println!("Error: {}", state[i]);
            return 0;
        }
        i += 4;
    }
    state[0]
}
