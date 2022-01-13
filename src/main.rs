use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Colours {
    Green,
    Grey,
    Amber,
}

fn main() {
    let file = File::open("/usr/share/dict/words").unwrap();
    let dict: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|s| s.ok())
        .filter(|s| s == &s.to_lowercase())
        .collect();
    let mut wordlist: Vec<&str> = dict
        .iter()
        .filter(|s| s.len() == 5)
        .map(|s| &s[..])
        .collect();

    make_guess(&make_guess(&make_guess(&make_guess(&make_guess(&wordlist)))));
}

fn make_guess<'a>(wordlist: &'a Vec<&str>) -> Vec<&'a str> {
    let frequencies = wordlist.iter().fold(HashMap::new(), |mut map, word| {
        for c in word.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    });

    let likley = get_most_likley(&wordlist, &frequencies);

    println!("Your next best words are: {:?}", likley.iter().take(10).map(|w| *w).collect::<Vec<&str>>());

    println!("What word did you enter?");
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();

    println!("What colours did you see \n G = Green\nX = Grey\nA = Amber\ne.g XGXXA:");
    let mut answer_input = String::new();
    io::stdin().read_line(&mut answer_input).unwrap();

    if answer_input.trim().len() != 5 {
        panic!(
            "you need to enter 5 characters {:?} {}",
            answer_input.trim(),
            answer_input.trim().len()
        );
    }

    let answers: [Colours; 5] = answer_input
        .trim()
        .chars()
        .map(|c| match c {
            'G' => Colours::Green,
            'X' => Colours::Grey,
            'A' => Colours::Amber,
            _ => panic!("Invalid char: {}", c),
        })
        .collect::<Vec<Colours>>()
        .try_into()
        .unwrap();

    let wl = refine_wordlist(&word.trim()[..], answers, wordlist);
    println!("{:?}", wl);
    wl
}

fn refine_wordlist<'a, 'b>(
    word: &'b str,
    answers: [Colours; 5],
    wordlist: &'a Vec<&str>,
) -> Vec<&'a str> {
    let must_contain: HashSet<String> = word
        .chars()
        .enumerate()
        .filter(|(i, _)| answers[*i] != Colours::Grey)
        .map(|(_, c)| c.to_string())
        .collect();
    let must_not_contain: HashSet<String> = word
        .chars()
        .enumerate()
        .filter(|(i, _)| answers[*i] == Colours::Grey)
        .map(|(_, c)| c.to_string())
        .collect();
    let exact: Vec<(usize, char)> = word
        .chars()
        .enumerate()
        .filter(|(i, _)| answers[*i] == Colours::Green)
        .map(|(i, c)| (i, c))
        .collect();
    let not_exact: Vec<(usize, char)> = word
        .chars()
        .enumerate()
        .filter(|(i, _)| answers[*i] == Colours::Amber)
        .map(|(i, c)| (i, c))
        .collect();
    wordlist
        .iter()
        .filter(|w| must_contain.iter().all(|c| w.contains(c)))
        .filter(|w| !must_not_contain.iter().any(|c| w.contains(c)))
        .filter(|w| exact.iter().all(|(i, c)| w.chars().nth(*i).unwrap() == *c))
        .filter(|w| !not_exact.iter().any(|(i, c)| w.chars().nth(*i).unwrap() == *c))
        .map(|v| *v)
        .collect()
}

fn get_most_likley<'a>(wordlist: &'a Vec<&str>, frequencies: &HashMap<char, u32>) -> Vec<&'a str>  {
    let scores = wordlist.iter().fold(HashMap::new(), |mut map, word| {
        map.insert(word, word_cost(&word, frequencies));
        map
    });

    let mut best_score: Vec<(&str, u32)> = scores.iter().map(|v| (**v.0, *v.1)).collect();
    best_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    best_score.iter().map(|(s, _)| *s).collect()
}

fn word_cost(word: &str, frequencies: &HashMap<char, u32>) -> u32 {
    let uniq_chars: HashSet<char> = word.chars().collect();
    uniq_chars
        .iter()
        .fold(0, |sum, c| sum + frequencies.get(c).unwrap())
}
