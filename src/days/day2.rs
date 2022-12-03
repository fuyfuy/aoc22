use phf::phf_map;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
pub fn run() {
    let input_dir = format!("{}{}", env::current_dir().unwrap().to_str().unwrap(), r"\inputs\inp2.txt");
    let file = File::open(input_dir).unwrap();
    let reader = BufReader::new(file);
    let mut total_score = 0;
    let mut total_score_part2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let input = line.split_once(' ').unwrap_or_default();
        total_score += calculate_score_part1(input);
        total_score_part2 += calculate_score_part2(input);
    }
    println!("part 1 score is {}", total_score);
    println!("part 2 score is {}", total_score_part2);
}

fn calculate_score_part1(round_items: (&str, &str)) -> i32 {
    let opponent_item = OPP_LETTERS_TO_ITEMS.get(round_items.0).unwrap().clone();
    let hero_item = HERO_LETTERS_TO_ITEMS.get(round_items.1).unwrap().clone();
    let round = GameRound {
        hero_item: &hero_item, opponent_item: &opponent_item
    };
    fight(round) as i32 + hero_item as i32
}

fn calculate_score_part2(input_p2: (&str, &str)) -> i32 {
    let desired_result = LETTERS_TO_RESULT.get(input_p2.1).unwrap().clone();
    let opp_item = OPP_LETTERS_TO_ITEMS.get(input_p2.0).unwrap().clone();
    let hero_item = find_item_by_result(opp_item, &desired_result);
    desired_result as i32 + hero_item as i32
}

fn fight(round: GameRound) -> Result {
    if round.hero_item == round.opponent_item { return Result::Draw; };
    match round {
        GameRound { hero_item: GameItem::Rock, opponent_item: GameItem::Paper, } => Result::Lose,
        GameRound { hero_item: GameItem::Rock, opponent_item: GameItem::Scissors, } => Result::Win,
        GameRound { hero_item: GameItem::Paper, opponent_item: GameItem::Rock, } => Result::Win,
        GameRound { hero_item: GameItem::Paper, opponent_item: GameItem::Scissors, } => Result::Lose,
        GameRound { hero_item: GameItem::Scissors, opponent_item: GameItem::Paper, } => Result::Win,
        GameRound { hero_item: GameItem::Scissors, opponent_item: GameItem::Rock, } => Result::Lose,
        _ => panic!(), }
}

fn find_item_by_result(opp_item: GameItem, desired_result: &Result) -> GameItem {
    for item in GameItem::iter() {
        let candidate_round = GameRound {hero_item: &item, opponent_item: &opp_item };
        if &fight(candidate_round) == desired_result {
            return item;
        }
    }
    panic!()
}

struct GameRound<'a> {
    hero_item: &'a GameItem,
    opponent_item: &'a GameItem,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Result { Lose = 0, Draw = 3, Win = 6, }

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
enum GameItem { Rock = 1, Paper = 2, Scissors = 3, }

static HERO_LETTERS_TO_ITEMS: phf::Map<&str, GameItem> =
    phf_map! { "X" => GameItem::Rock, "Y" => GameItem::Paper, "Z" => GameItem::Scissors};
static OPP_LETTERS_TO_ITEMS: phf::Map<&str, GameItem> =
    phf_map! { "A" => GameItem::Rock, "B" => GameItem::Paper, "C" => GameItem::Scissors};
static LETTERS_TO_RESULT: phf::Map<&str, Result> =
    phf_map! { "X" => Result::Lose, "Y" => Result::Draw, "Z" => Result::Win};
