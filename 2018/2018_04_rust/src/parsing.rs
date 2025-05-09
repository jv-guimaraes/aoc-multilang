use itertools::Itertools;

#[derive(Debug)]
pub enum Action {
    BeginShift(u32),
    FallSleep,
    WakeUp,
}

impl Action {
    pub fn id(&self) -> u32 {
        match self {
            Action::BeginShift(id) => *id,
            _ => panic!("Was {:?}, not BeginShift", self),
        }
    }
}

#[derive(Debug)]
pub struct Log {
    pub day: String,
    pub time: u8,
    pub action: Action,
}

impl Log {
    pub fn new(text: &str) -> Log {
        let tokens = text.split_ascii_whitespace().collect_vec();
        let mut day = tokens[0][6..].to_string();
        let mut time: u8 = tokens[1].split(':').nth(1).unwrap().strip_suffix(']').unwrap().parse().unwrap();
        let action = match tokens[2] {
            "Guard" => {
                time = 0;
                if tokens[1].starts_with("23") { day = next_day(&day); } 
                Action::BeginShift(tokens[3][1..].parse().unwrap())
            },
            "falls" => Action::FallSleep,
            "wakes" => Action::WakeUp,
            _ => panic!(),
        };
        Log { day, time, action }
    }
}

fn next_day(current_day: &str) -> String {
    let (month, day) = current_day.split('-').collect_tuple().unwrap();
    match (month, day) {
        ("01", "31") => "02-01".to_string(),
        ("02", "28") => "03-01".to_string(),
        ("03", "31") => "04-01".to_string(),
        ("04", "30") => "05-01".to_string(),
        ("05", "31") => "06-01".to_string(),
        ("06", "30") => "07-01".to_string(),
        ("07", "31") => "08-01".to_string(),
        ("08", "31") => "09-01".to_string(),
        ("09", "30") => "10-01".to_string(),
        ("10", "31") => "11-01".to_string(),
        ("11", "30") => "12-01".to_string(),
        _ => format!("{}-{:02}", month, day.parse::<u8>().unwrap() + 1),
    }
}

#[test]
fn test() {
    assert_eq!(next_day("01-30"), "01-31");
    assert_eq!(next_day("01-31"), "02-01");
}