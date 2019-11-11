use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Transition {
    source: String,
    target: String,
}

type Transitions = Vec<Transition>;

fn get_transitions(samples: Vec<&str>) -> Transitions {
    let mut result = vec![];
    samples
        .iter()
        .map(|s| {
            let mut iter = s.split(|c| c == ' ' || c == '.').peekable();
            while let (Some(source), Some(target)) = (iter.next(), iter.peek()) {
                result.push(Transition {
                    source: source.to_string(),
                    target: target.to_string(),
                })
            }
            s
        })
        .for_each(drop);
    result
}

fn generate_sequence(transitions: &Transitions) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();
    let mut tran = transitions.choose(&mut rng).unwrap();
    println!("{:?}", tran);
    result.push_str(&tran.source);
    loop {
        if tran.target == "" {
            break;
        }
        tran = transitions
            .iter()
            .filter(|t| t.source == tran.target)
            .choose(&mut rng)
            .unwrap();
        result.push_str(" ");
        result.push_str(&tran.source);
        println!("{:?}", tran);
    }
    result.push_str(".");
    result
}

fn main() {
    let samples = vec![
        "I am a monster.",
        "I am a rock star.",
        "I want to go to Hawaii.",
        "I want to eat a hamburger.",
        "I have a really big headache.",
        "Haskell is a fun language.",
        "Go eat a big hamburger.",
        "Markov chains are fun to use.",
    ];
    let transitions = get_transitions(samples);
    let sequence = generate_sequence(&transitions);
    println!("{}", sequence);
}
