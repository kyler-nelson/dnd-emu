extern crate serde;
extern crate yaml_rust;

use std::fmt::Debug;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::ops::Index;
use std::path::Path;

use serde::{Deserialize, Serialize};
use yaml_rust::YamlLoader;

#[derive(Serialize, Deserialize, Debug)]
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

struct RaceAttributes {
    age: u64,
    alignment: Alignment,
    size: Size,
    speed: u16,
    languages: Vec<Language>,
    traits: Vec<Trait>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Subrace {
    parent: Race,
}
#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
enum Size {
    Small,
    Medium,
    Large,
    Huge,
}
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
enum Background {}
#[derive(Serialize, Deserialize, Debug)]
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

const EFFECTIVE_ABILITY_SCORE_MIN: u8 = 0;
const EFFECTIVE_ABILITY_SCORE_MAX: u8 = 30;
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
enum WeaponType {
    Shields,
    SimpleWeapons,
    MartialWeapons,
}
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
enum ArmorType {
    LightArmor,
    MediumArmor,
    HeavyArmor,
}

#[derive(Serialize, Deserialize, Debug)]
struct CharacterAbilities([AbilityScore; 6]);

impl Index<Ability> for CharacterAbilities {
    type Output = AbilityScore;

    fn index(&self, index: Ability) -> &Self::Output {
        &self
            .0
            .iter()
            .find(|&ability_score| ability_score.ability == index)
            .expect("Ability score not found")
    }
}

const EFFECTIVE_LEVEL_MIN: u32 = 1;
const EFFECTIVE_LEVEL_MAX: u32 = 20;
#[derive(Serialize, Deserialize, Debug)]
struct Character {
    race: Race,
    name: &'static str,
    age: u32,
    alignment: Alignment,
    size: Size,
    speed: i64,
    languages: Vec<Language>,
    experience_points: u64,
    level: u32,
    ability_scores: CharacterAbilities,
    traits: Vec<Trait>,
}

impl Character {
    fn get_ability_score(&self, ability: Ability) -> AbilityScore {
        self.ability_scores[ability]
    }

    fn get_level(&self) -> u32 {
        calculate_level_from_experience_points(self.experience_points)
    }

    fn get_proficiency_bonus(&self) -> u16 {
        calculate_proficiency_bonus_from_experience_points(self.experience_points)
    }
}

fn calculate_level_from_experience_points(experience_points: u64) -> u32 {
    let mut expected_level = EFFECTIVE_LEVEL_MIN;
    for entry in CHARACTER_ADVANCEMENT_TABLE.iter() {
        if experience_points >= entry.required_experience_points {
            expected_level = entry.level;
        }
    }

    assert!(expected_level >= EFFECTIVE_LEVEL_MIN && expected_level <= EFFECTIVE_LEVEL_MAX);

    expected_level
}

fn calculate_experience_points_required_for_next_level(experience_points: u64) -> u64 {
    let mut required_experience_points = 0;
    for entry in CHARACTER_ADVANCEMENT_TABLE.iter() {
        if experience_points < entry.required_experience_points {
            required_experience_points = entry.required_experience_points - experience_points;
            break;
        }
    }
    required_experience_points
}

fn calculate_proficiency_bonus_from_experience_points(experience_points: u64) -> u16 {
    let mut expected_proficiency_bonus = 1;
    for entry in CHARACTER_ADVANCEMENT_TABLE.iter() {
        if experience_points >= entry.required_experience_points {
            expected_proficiency_bonus = entry.proficiency_bonus;
        }
    }

    return expected_proficiency_bonus;
}

#[derive(Serialize, Deserialize, Debug)]
struct Trait {
    name: &'static str,
    description: &'static str,
    weapon_proficiency_modifiers: Vec<WeaponProficiencyModifier>,
    armor_proficiency_modifiers: Vec<ArmorProficiencyModifier>,
    ability_modifiers: Vec<AbilityModifier>,
}

trait Modifier<T> {
    fn get_name(&self) -> &'static str;
    fn get_value(&self) -> T;
    fn get_modifier_type(&self) -> ModifierType;
}

#[derive(Serialize, Deserialize, Debug)]
struct AbilityModifier {
    name: &'static str,
    ability: Ability,
    value: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeaponProficiencyModifier {
    name: &'static str,
    value: WeaponType,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArmorProficiencyModifier {
    name: &'static str,
    value: ArmorType,
}

impl Modifier<i8> for AbilityModifier {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_value(&self) -> i8 {
        self.value
    }

    fn get_modifier_type(&self) -> ModifierType {
        ModifierType::Ability
    }
}

impl Modifier<WeaponType> for WeaponProficiencyModifier {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_value(&self) -> WeaponType {
        self.value
    }

    fn get_modifier_type(&self) -> ModifierType {
        ModifierType::WeaponProficiency
    }
}

impl Modifier<ArmorType> for ArmorProficiencyModifier {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_value(&self) -> ArmorType {
        self.value
    }

    fn get_modifier_type(&self) -> ModifierType {
        ModifierType::ArmorProficiency
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum ModifierType {
    WeaponProficiency,
    ArmorProficiency,
    Ability,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
struct AbilityScore {
    ability: Ability,
    score: u8,
    modifier: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct RaceInfo {
    race: Race,
    age: u32,
    alignment: Alignment,
    size: Size,
    speed: i64,
    languages: Vec<Language>,
}

struct CharacterAdvancementEntry {
    level: u32,
    required_experience_points: u64,
    proficiency_bonus: u16,
}

#[rustfmt::skip]
const CHARACTER_ADVANCEMENT_TABLE: [CharacterAdvancementEntry; 20] = [
    CharacterAdvancementEntry {required_experience_points: 0,       level: 1,   proficiency_bonus: 2},
    CharacterAdvancementEntry {required_experience_points: 300,     level: 2,   proficiency_bonus: 2},
    CharacterAdvancementEntry {required_experience_points: 900,     level: 3,   proficiency_bonus: 2},
    CharacterAdvancementEntry {required_experience_points: 2700,    level: 4,   proficiency_bonus: 2},
    CharacterAdvancementEntry {required_experience_points: 6500,    level: 5,   proficiency_bonus: 3},
    CharacterAdvancementEntry {required_experience_points: 14000,   level: 6,   proficiency_bonus: 3},
    CharacterAdvancementEntry {required_experience_points: 23000,   level: 7,   proficiency_bonus: 3},
    CharacterAdvancementEntry {required_experience_points: 34000,   level: 8,   proficiency_bonus: 3},
    CharacterAdvancementEntry {required_experience_points: 48000,   level: 9,   proficiency_bonus: 4},
    CharacterAdvancementEntry {required_experience_points: 64000,   level: 10,  proficiency_bonus: 4},
    CharacterAdvancementEntry {required_experience_points: 85000,   level: 11,  proficiency_bonus: 4},
    CharacterAdvancementEntry {required_experience_points: 100000,  level: 12,  proficiency_bonus: 4},
    CharacterAdvancementEntry {required_experience_points: 120000,  level: 13,  proficiency_bonus: 5},
    CharacterAdvancementEntry {required_experience_points: 140000,  level: 14,  proficiency_bonus: 5},
    CharacterAdvancementEntry {required_experience_points: 165000,  level: 15,  proficiency_bonus: 5},
    CharacterAdvancementEntry {required_experience_points: 195000,  level: 16,  proficiency_bonus: 5},
    CharacterAdvancementEntry {required_experience_points: 225000,  level: 17,  proficiency_bonus: 6},
    CharacterAdvancementEntry {required_experience_points: 265000,  level: 18,  proficiency_bonus: 6},
    CharacterAdvancementEntry {required_experience_points: 305000,  level: 19,  proficiency_bonus: 6},
    CharacterAdvancementEntry {required_experience_points: 355000,  level: 20,  proficiency_bonus: 6},
];

fn load_races_from_file(file_path: &'static str) -> Vec<RaceInfo> {
    let mut file = File::open(file_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    YamlLoader::load_from_str(&file_contents).unwrap();

    let mut races: Vec<RaceInfo> = Vec::new();

    races.push(RaceInfo {
        race: Race::Dwarf,
        age: 80,
        alignment: Alignment::ChaoticNeutral,
        size: Size::Medium,
        speed: 25,
        languages: vec![Language::Dwarvish],
    });

    let file_path = Path::new("serialize/races.yaml");
    let directory = file_path.parent().unwrap();

    if !directory.exists() {
        std::fs::create_dir(directory).unwrap();
    }

    let races_output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    serde_yaml::to_writer(&races_output_file, &races).unwrap();

    return races;
}

fn load_characters_from_file(file_path: &'static str) -> Vec<Character> {
    let mut file = File::open(file_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    YamlLoader::load_from_str(&file_contents).unwrap();

    let mut characters: Vec<Character> = Vec::new();

    let character = Character {
        name: "Tishros",
        experience_points: 0,
        level: 1,
        race: Race::Dwarf,
        age: 80,
        alignment: Alignment::ChaoticNeutral,
        size: Size::Medium,
        speed: 25,
        languages: vec![Language::Dwarvish],
        ability_scores: CharacterAbilities([
            AbilityScore {
                ability: Ability::Strength,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Dexterity,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Constitution,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Wisdom,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Intelligence,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Charisma,
                score: 10,
                modifier: 0,
            },
        ]),
        traits: vec![Trait {
            name: "test",
            description: "Hello",
            weapon_proficiency_modifiers: vec![],
            armor_proficiency_modifiers: vec![],
            ability_modifiers: vec![],
        }],
    };

    characters.push(character);

    let file_path = Path::new("serialize/characters.yaml");
    let directory = file_path.parent().unwrap();

    if !directory.exists() {
        std::fs::create_dir(directory).unwrap();
    }

    let characters_output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    serde_yaml::to_writer(&characters_output_file, &characters).unwrap();

    return characters;
}

fn main() {
    let races = load_races_from_file("data/races.yaml");
    let mut characters = load_characters_from_file("data/characters.yaml");
    println!("{:?}", races);
    println!("{:?}", characters);

    characters[0].experience_points = 0;
    println!("{:?}", characters[0].get_level());
    println!("{:?}", characters[0].get_proficiency_bonus());
    characters[0].experience_points = 555;
    println!("{:?}", characters[0].get_level());
    println!("{:?}", characters[0].get_proficiency_bonus());
    println!(
        "{:?}",
        calculate_experience_points_required_for_next_level(characters[0].experience_points)
    );
}
