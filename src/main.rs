extern crate yaml_rust;

use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use yaml_rust::YamlLoader;

#[derive(Debug)]
enum Race {
    Dwarf,
    HillDwarf,
    Elf,
    HighElf,
    Human,
    Gnome,
    Halfling,
    Dragonborn,
    HalfElf,
    HalfOrc,
    Tiefling,
}

struct Subrace {
    parent: Race,
}

enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    Neutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
    Unaligned,
}

enum Size {
    Small,
    Medium,
    Large,
    Huge,
}

enum Language {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommmon,
}

enum Background {}

enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorceror,
    Warlock,
    Wizard,
}

struct Character {
    race: Race,
    name: &'static str,
    age: u32,
    alignment: Alignment,
    size: Size,
    speed: i64,
    languages: Vec<Language>,
    experience_points: u32,
    level: u32,
}

fn main() {
    let races = load_races_from_file("./data/races.yaml");
    println!("{:?}", races);
}

fn load_races_from_file(file_path: &'static str) -> Vec<Race> {
    let mut file = File::open(file_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    YamlLoader::load_from_str(&file_contents).unwrap();

    let races: Vec<Race> = Vec::new();

    return races;
}
