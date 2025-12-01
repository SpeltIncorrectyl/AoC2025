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
    let mut offset = match (&line[0..1], &line[1..]) {
        ("L", x) => -x.parse::<i32>().unwrap(),
        ("R", x) => x.parse::<i32>().unwrap(),
        _ => unreachable!()
    };

    while offset != 0 {
        if offset.abs() >= 100 {
            offset -= 100 * offset.signum();
            state.score += 1;
            continue;
        }

        offset -= offset.signum();
        state.angle += offset.signum();
        if state.angle == 100 {
            state.angle = 0;
        }
        if state.angle == -1 {
            state.angle = 99;
        }
        if state.angle == 0 {
            state.score += 1;
        }
    }

    return state;
}

pub fn solve(input: &str) -> i32 {
    input.lines().fold(State::default(), process).score
}

fn main() {
    println!("{}", solve(include_str!("input.txt")));
}