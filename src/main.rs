use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Colours {
    Green,
    Grey,
    Amber,
}

struct Guesser{
    dict: Vec<String>,
    must_contain: HashSet<String>,
    must_not_contain: HashSet<String>,
    exact: Vec<(usize, char)>,
    not_exact: Vec<(usize, char)>
}

impl Guesser {
    fn new(dict: Vec<String>) -> Self {
        Self {
            dict: dict,
            must_contain: HashSet::new(),
            must_not_contain: HashSet::new(),
            exact: Vec::new(),
            not_exact: Vec::new(),
        }
    }

    fn get_most_likley(&self) -> Vec<&str> {
        let frequencies = self.get_frequencies();
        let options = self.get_options();
        let scores = options.iter().fold(HashMap::new(), |mut map, word| {
            map.insert(word, Self::word_score(&word, &frequencies));
            map
        });

        let mut best_score: Vec<(&str, usize)> = scores.iter().map(|v| (**v.0, *v.1)).collect();
        best_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        best_score.iter().map(|(s, _)| *s).take(10).collect()
    }

    fn get_frequencies(&self) -> HashMap<char, usize> {
        self.get_options().iter().fold(HashMap::new(), |mut map, word| {
            for c in word.chars() {
                *map.entry(c).or_insert(0) += 1;
            }
            map
        })
    }

    fn get_options(&self) -> Vec<&str> {
        self.dict.iter()
            .filter(|s| s.len() == 5)
            .map(|s| &s[..])
            .filter(|w| self.must_contain.iter().all(|c| w.contains(c)))
            .filter(|w| !self.must_not_contain.iter().any(|c| w.contains(c)))
            .filter(|w| self.exact.iter().all(|(i, c)| w.chars().nth(*i).unwrap() == *c))
            .filter(|w| !self.not_exact.iter().any(|(i, c)| w.chars().nth(*i).unwrap() == *c))
            .collect()
    }

    fn guess(&mut self, word: String, answers: [Colours; 5]) {
        for (i, c) in word.trim().chars().enumerate() {
            if answers[i] != Colours::Grey {
                self.must_contain.insert(c.to_string());
            }

            match answers[i] {
                Colours::Grey => { self.must_not_contain.insert(c.to_string()); },
                Colours::Green => { self.exact.push((i, c)); },
                Colours::Amber => { self.not_exact.push((i, c)); },
            }
        }
    }

    fn word_score(word: &str, frequencies: &HashMap<char, usize>) -> usize {
        let uniq_chars: HashSet<char> = word.chars().collect();
        uniq_chars
            .iter()
            .fold(0, |sum, c| sum + frequencies.get(c).unwrap())
    }
}

fn main() {
    let file = File::open("/usr/share/dict/words").unwrap();
    let dict: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|s| s.ok())
        .filter(|s| s == &s.to_lowercase())
        .collect();


    let mut guesser = Guesser::new(dict);

    for _ in 0..6 {
        println!("Your most likley options are: {:?}", guesser.get_most_likley());

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

        guesser.guess(word, answers);
    }
}
