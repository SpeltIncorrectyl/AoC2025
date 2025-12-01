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
    state.angle += match (&line[0..1], &line[1..]) {
        ("L", x) => -x.parse::<i32>().unwrap(),
        ("R", x) => x.parse::<i32>().unwrap(),
        _ => unreachable!()
    };
    
    while state.angle > 99 {
        state.angle -= 100;
    }

    while state.angle < 0 {
        state.angle += 100;
    }

    if state.angle == 0 {
        state.score += 1;
    }

    return state;
}

pub fn solve(input: &str) -> i32 {
    input.lines().fold(State::default(), process).score
}

fn main() {
    println!("{}", solve(include_str!("input.txt")));
}
