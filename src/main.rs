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

#[derive(Copy, Clone)]
enum WeaponType {
    Shields,
    SimpleWeapons,
    MartialWeapons,
}
#[derive(Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug)]
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
    ability_scores: CharacterAbilities,
}

impl Character {
    fn get_ability_score(&self, ability: Ability) -> AbilityScore {
        self.ability_scores[ability]
    }
}

struct Trait {
    name: &'static str,
    description: &'static str,
    weapon_proficiency_modifiers: Vec<WeaponProficiencyModifier>,
    armor_proficiency_modifiers: Vec<ArmorProficiencyModifier>,
    ability_proficiency_modifiers: Vec<ArmorProficiencyModifier>,
    ability_modifiers: Vec<AbilityModifier>,
}

trait Modifier<T> {
    fn get_name(&self) -> &'static str;
    fn get_value(&self) -> T;
    fn get_modifier_type(&self) -> ModifierType;
}

struct AbilityModifier {
    name: &'static str,
    ability: Ability,
    value: i8,
}

struct WeaponProficiencyModifier {
    name: &'static str,
    value: WeaponType,
}

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

impl AbilityModifier {
    fn get_ability(&self) -> Ability {
        self.ability
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

fn main() {
    let races = load_races_from_file("data/races.yaml");
    let characters = load_characters_from_file("data/characters.yaml");
    println!("{:?}", races);
    println!("{:?}", characters);
}

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

    println!("{:?}", &races);

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
                ability: Ability::Strength,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Strength,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Strength,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Strength,
                score: 10,
                modifier: 0,
            },
            AbilityScore {
                ability: Ability::Strength,
                score: 10,
                modifier: 0,
            },
        ]),
    };

    println!("{:?}", character.ability_scores[Ability::Strength]);

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

    println!("{:?}", &characters);

    return characters;
}
