fn main() {
    use std::collections::HashMap;
    use ndarray::arr2;

    let states = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
    ];

    let actions = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
    ];

    let locations = [
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
    ];

    let mut location_to_state: HashMap<String, i32> = HashMap::new();
    for location_state in locations.iter().zip(states.iter()) {
        let (location, state) = location_state;
        location_to_state.insert(
            location.to_string(),
            *state,
        );
    }

    let mut state_to_location: HashMap<i32, String> = HashMap::new();
    for location_state in locations.iter().zip(states.iter()) {
        let (location, state) = location_state;
        state_to_location.insert(
            *state,
            location.to_string(),
        );
    }

    let reward = arr2(&[
        [0,1,0,0,0,0,0,0,0,0,0,0],
        [1,0,1,0,0,1,0,0,0,0,0,0],
        [0,1,0,0,0,0,1,0,0,0,0,0],
        [0,0,0,0,0,0,0,1,0,0,0,0],
        [0,0,0,0,0,0,0,0,1,0,0,0],
        [0,1,0,0,0,0,0,0,0,1,0,0],
        [0,0,1,0,0,0,0,1,0,0,0,0],
        [0,0,0,1,0,0,1,0,0,0,0,1],
        [0,0,0,0,1,0,0,0,0,1,0,0],
        [0,0,0,0,0,1,0,0,1,0,1,0],
        [0,0,0,0,0,0,0,0,0,1,0,1],
        [0,0,0,0,0,0,0,1,0,0,1,0]
        ]);

}

