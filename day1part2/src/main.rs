pub struct State {
    pub score: i32,
    pub angle: i32
}

impl Default for State {
    fn default() -> Self {
        State {score: 0, angle: 50}
    }
}

pub fn process(mut state: State, line: &str) -> State {
    let offset = match (&line[0..1], &line[1..]) {
        ("L", x) => -x.parse::<i32>().unwrap(),
        ("R", x) => x.parse::<i32>().unwrap(),
        _ => unreachable!()
    };

    // if we start at 0 and turn left the program will count that as a click
    // but this would mean be double count a click from last time
    // so if we are at 0 and turning left reduce score by 1 to compensate
    if state.angle == 0 && offset < 0 {
        state.score -= 1;
    }

    state.angle += offset;
    
    while state.angle >= 100 {
        state.angle -= 100;
        state.score += 1;
    }

    while state.angle < 0 {
        state.angle += 100;
        state.score += 1;
    }

    // if state.score < 100 {
    //     println!("{}: angle: {} score: {}", line, state.angle, state.score);
    // }

    return state;
}

pub fn solve(input: &str) -> i32 {
    input.lines().fold(State::default(), process).score
}

fn main() {
    println!("{}", solve(include_str!("input.txt")));
}