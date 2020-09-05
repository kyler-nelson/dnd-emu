extern crate rand;
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

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

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
enum ClassType {
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

struct Die {
    min: u16,
    max: u16,
}

fn roll_die(die: Die) -> u16 {
    let mut rng = rand::thread_rng();
    let die_range = Uniform::from(die.min..die.max);
    die_range.sample(&mut rng)
}

struct ClassFeatures {
    hit_dice: Die,
    hit_points_starting: u16,
    hit_points_from_level: Die,
}

struct Class {
    class_type: ClassType,
    features: ClassFeatures,
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
    roll_hit_points: bool,
}

const MIN_SPELL_LEVEL: u8 = 0;
const MAX_SPELL_LEVEL: u8 = 9;
struct Spell {
    level: u8,
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

    fn gain_level(&mut self) {
        if self.level != calculate_level_from_experience_points(self.experience_points) {
            self.roll_hit_points = true;
            self.add_class_features_for_level();
            self.level = self.level + 1;
        }
    }

    fn add_class_features_for_level(&self) {
        todo!();
    }
}

struct SpellSlotEntry {
    level: u32,
    spell_level_count: [u8; 10],
}

#[rustfmt::skip]
const WIZARD_SPELL_SLOTS_PER_SPELL_LEVEL: [SpellSlotEntry; 20] = [
    SpellSlotEntry {level: 1,  spell_level_count: [3,2,0,0,0,0,0,0,0,0] },
    SpellSlotEntry {level: 2,  spell_level_count: [3,3,0,0,0,0,0,0,0,0] },
    SpellSlotEntry {level: 3,  spell_level_count: [3,4,2,0,0,0,0,0,0,0] },
    SpellSlotEntry {level: 4,  spell_level_count: [4,4,3,0,0,0,0,0,0,0] },
    SpellSlotEntry {level: 5,  spell_level_count: [4,4,3,2,0,0,0,0,0,0] },
    SpellSlotEntry {level: 6,  spell_level_count: [4,4,3,3,0,0,0,0,0,0] },
    SpellSlotEntry {level: 7,  spell_level_count: [4,4,3,3,1,0,0,0,0,0] },
    SpellSlotEntry {level: 8,  spell_level_count: [4,4,3,3,2,0,0,0,0,0] },
    SpellSlotEntry {level: 9,  spell_level_count: [4,4,3,3,3,1,0,0,0,0] },
    SpellSlotEntry {level: 10, spell_level_count: [5,4,3,3,3,2,0,0,0,0] },
    SpellSlotEntry {level: 11, spell_level_count: [5,4,3,3,3,2,1,0,0,0] },
    SpellSlotEntry {level: 12, spell_level_count: [5,4,3,3,3,2,1,0,0,0] },
    SpellSlotEntry {level: 13, spell_level_count: [5,4,3,3,3,2,1,1,0,0] },
    SpellSlotEntry {level: 14, spell_level_count: [5,4,3,3,3,2,1,1,0,0] },
    SpellSlotEntry {level: 15, spell_level_count: [5,4,3,3,3,2,1,1,1,0] },
    SpellSlotEntry {level: 16, spell_level_count: [5,4,3,3,3,2,1,1,1,0] },
    SpellSlotEntry {level: 17, spell_level_count: [5,4,3,3,3,2,1,1,1,1] },
    SpellSlotEntry {level: 18, spell_level_count: [5,4,3,3,3,3,1,1,1,1] },
    SpellSlotEntry {level: 19, spell_level_count: [5,4,3,3,3,3,2,1,1,1] },
    SpellSlotEntry {level: 20, spell_level_count: [5,4,3,3,3,3,2,2,1,1] },
];

fn get_number_of_spell_slots_per_spell_level(class: Class, level: u32, spell_level: u8) -> u8 {
    match class.class_type {
        ClassType::Barbarian => todo!(),
        ClassType::Cleric => todo!(),
        ClassType::Fighter => todo!(),
        ClassType::Monk => todo!(),
        ClassType::Paladin => todo!(),
        ClassType::Ranger => todo!(),
        ClassType::Rogue => todo!(),
        ClassType::Sorceror => todo!(),
        ClassType::Bard => todo!(),
        ClassType::Druid => todo!(),
        ClassType::Warlock => todo!(),
        ClassType::Wizard => {
            return find_spell_splots_for_spell_level(
                WIZARD_SPELL_SLOTS_PER_SPELL_LEVEL,
                level,
                spell_level,
            );
        }
    }
}

fn find_spell_splots_for_spell_level(
    spell_slots_per_spell_level_table: [SpellSlotEntry; 20],
    level: u32,
    spell_level: u8,
) -> u8 {
    for entry in spell_slots_per_spell_level_table.iter() {
        if level == entry.level {
            return entry.spell_level_count[spell_level as usize];
        }
    }

    0
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

// NOTE: This assumes the CHARACTER_ADVANCEMENT_TABLE is ordered by required experience points
// as is shown in the SRD.
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
    let default_proficiency_bonus = 1;
    for entry in CHARACTER_ADVANCEMENT_TABLE.iter() {
        if experience_points >= entry.required_experience_points {
            return entry.proficiency_bonus;
        }
    }

    default_proficiency_bonus
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
        roll_hit_points: false,
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
    dbg!(calculate_experience_points_required_for_next_level(
        characters[0].experience_points
    ));
    dbg!(roll_die(Die { min: 1, max: 6 }));

    let wizard = Class {
        class_type: ClassType::Wizard,
        features: ClassFeatures {
            hit_dice: Die { min: 0, max: 6 },
            hit_points_starting: 0,
            hit_points_from_level: Die { min: 0, max: 6 },
        },
    };
    dbg!(get_number_of_spell_slots_per_spell_level(wizard, 20, 9));
}
