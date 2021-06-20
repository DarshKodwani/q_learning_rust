fn main() {
    use std::collections::HashMap;
    
    let mut location_to_state: HashMap<String, i32> = HashMap::new();
    
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

    for location_state in locations.iter().zip(states.iter()) {
        let (location, state) = location_state;
        location_to_state.insert(
            location.to_string(),
            *state,
        );
    }

}

