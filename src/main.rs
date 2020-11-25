extern crate rand;
extern crate serde;
extern crate yaml_rust;

#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::Abbreviation;

use std::cmp;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io;
use std::ops::Index;
use std::path::Path;

use serde::{Deserialize, Serialize};

use rand::distributions::{Distribution, Uniform};

// https://docs.rs/crate/uom/0.30.0/source/examples/mks.rs

#[macro_use]
mod coin {
    quantity! {
        /// Coin (base unit copper, cp).
        quantity: Coin; "coin";
        /// Coin dimension, cp.
        dimension: Q<Z0>; // amount
        units {
            @copper: 1.0; "cp", "copper", "copper";
            @silver: 10.0; "sp", "silver", "silver";
            @electrum: 50.0; "ep", "electrum", "electrum";
            @gold: 100.0; "gp", "gold", "gold";
            @platinum: 1000.0; "pp", "platinum", "platinum";
        }
    }
}

system! {
    quantities: Q {
        coin: copper, C;
    }

    units: U {
        mod coin::Coin,
    }
}

mod f32 {
    mod mks {
        pub use super::super::*;
    }

    Q!(self::mks, f32);
}

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

#[derive(Serialize, Deserialize, Debug)]
struct Die {
    min: u16,
    max: u16,
}

fn roll_die(die: Die) -> u16 {
    let mut rng = rand::thread_rng();
    let die_range = Uniform::from(die.min..die.max);
    die_range.sample(&mut rng)
}

#[derive(Serialize, Deserialize, Debug)]
struct ClassFeatures {
    hit_dice: Die,
    hit_points_starting: u16,
    hit_points_from_level: Die,
}

#[derive(Serialize, Deserialize, Debug)]
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

trait Item {}

struct Weapon {
    name: &'static str,
    cost: u32,
    damage: DamageRange,
    weapon_type: WeaponType,
    category: WeaponCategory,
    properties: Vec<WeaponProperty>,
}
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
enum WeaponType {
    Melee,
    Ranged,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
enum WeaponCategory {
    Shields,
    SimpleWeapons,
    MartialWeapons,
}

enum WeaponProperty {
    Ammunition,
    Finesse,
    Heavy,
    Light,
    Loading,
    Range,
    Reach,
    Special,
    Thrown,
    TwoHanded,
    Versatile,
}

#[derive(Serialize, Deserialize, Debug)]
struct DamageRange {
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Armor {
    armor_type: ArmorType,
    category: ArmorCategory,
    cost: f32::Coin,
    base_armor_class: u16,
    weight: u32,
    ability_requirement: Option<AbilityScore>,
    has_stealth_disadvantage: bool,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
enum ArmorCategory {
    LightArmor,
    MediumArmor,
    HeavyArmor,
}

#[derive(Serialize, Deserialize, Debug)]
enum ArmorType {
    Padded,
    Leather,
    StuddedLeather,
    Hide,
    ChainShirt,
    ScaleMail,
    Breastplate,
    HalfPlate,
    RingMail,
    ChainMail,
    Splint,
    Plate,
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
    name: String,
    age: u32,
    class: Vec<Class>,
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

    fn get_current_level(&self) -> u32 {
        calculate_level_from_experience_points(self.experience_points)
    }

    fn get_proficiency_bonus(&self) -> u16 {
        calculate_proficiency_bonus_from_experience_points(self.experience_points)
    }

    fn gain_level(&mut self) {
        let target_level = calculate_level_from_experience_points(self.experience_points);
        match target_level {
            target_level if self.level < target_level => {
                while self.level != target_level {
                    self.roll_hit_points = true; //@TODO: Create map of health rolled at each level
                    self.add_class_features_for_level();
                    self.level = self.level + 1;
                }
            }
            target_level if self.level > target_level => {
                // Handle when someone is overleveled. Return resources?
            }
            target_level if self.level == target_level => {
                // Do nothing if already at level
            }
            _ => {
                // Take no action
            }
        }
    }

    fn add_class_features_for_level(&self) {
        todo!();
    }
}

const EFFECTIVE_SPELL_LEVEL_MIN: u8 = 0;
const EFFECTIVE_SPELL_LEVEL_MAX: u8 = 9;
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

fn get_number_of_spell_slots_for_spell_level(class: Class, level: u32, spell_level: u8) -> u8 {
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
    assert!(spell_level >= EFFECTIVE_SPELL_LEVEL_MIN && spell_level <= EFFECTIVE_SPELL_LEVEL_MAX);

    let indexed_spell_level = spell_level - 1;
    for entry in spell_slots_per_spell_level_table.iter() {
        if level == entry.level {
            return entry.spell_level_count[indexed_spell_level as usize];
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

fn calculate_base_armor_class_for_character(armor: Armor, character: Character) -> u16 {
    match armor.category {
        ArmorCategory::LightArmor => {
            let dexterity = character.ability_scores[Ability::Dexterity];
            let armor_class_bonus = dexterity.modifier as u16;
            return armor.base_armor_class + armor_class_bonus;
        }
        ArmorCategory::MediumArmor => {
            // If you wear medium armor, you add your Dexterity modifier, to a maximum of +2,
            // to the base number from your armor type to determine your Armor Class.
            let dexterity = character.ability_scores[Ability::Dexterity];
            let armor_class_bonus = cmp::max(
                dexterity.modifier as u16,
                cmp::min(dexterity.modifier as u16, 2),
            );
            return armor.base_armor_class + armor_class_bonus;
        }
        ArmorCategory::HeavyArmor => {
            return armor.base_armor_class;
        }
    }
}

fn has_proficiency_for_armor(armor: Armor, character: Character) -> bool {
    for character_trait in character.traits {
        for armor_proficiency_modifiers in character_trait.armor_proficiency_modifiers {
            if armor_proficiency_modifiers.value == armor.category {
                return true;
            }
        }
    }
    return false;
}

#[derive(Serialize, Deserialize, Debug)]
struct Trait {
    name: String,
    description: String,
    weapon_proficiency_modifiers: Vec<WeaponProficiencyModifier>,
    armor_proficiency_modifiers: Vec<ArmorProficiencyModifier>,
}

trait Modifier<T> {
    fn get_name(&self) -> String;
    fn get_value(&self) -> T;
    fn get_modifier_type(&self) -> ModifierType;
}

const INITIAL_ABILITY_SCORE: u8 = 10;
const MIN_ABILITY_MODIFIER_LEVEL: i8 = -5;
const MAX_ABILITY_MODIFIER_LEVEL: i8 = 10;
fn derive_ability_modifier_from_ability_score(score: u8) -> i8 {
    let ability_modifier: f32 = (score as f32 - INITIAL_ABILITY_SCORE as f32) / 2.0;
    let floored_ability_score = ability_modifier.floor() as i8;
    assert!(
        floored_ability_score >= MIN_ABILITY_MODIFIER_LEVEL
            && floored_ability_score <= MAX_ABILITY_MODIFIER_LEVEL
    );
    return floored_ability_score;
}

#[derive(Serialize, Deserialize, Debug)]
struct WeaponProficiencyModifier {
    name: String,
    value: WeaponType,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArmorProficiencyModifier {
    name: String,
    value: ArmorCategory,
}

impl Modifier<WeaponType> for WeaponProficiencyModifier {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> WeaponType {
        self.value
    }

    fn get_modifier_type(&self) -> ModifierType {
        ModifierType::WeaponProficiency
    }
}

impl Modifier<ArmorCategory> for ArmorProficiencyModifier {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> ArmorCategory {
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

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
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

// coin: f32::Coin,
struct Wealth {
    copper: f32::Coin,
    silver: f32::Coin,
    electrum: f32::Coin,
    gold: f32::Coin,
    platinum: f32::Coin,
}

trait WealthManagement {
    fn add_copper(&mut self, amount: f32);
    fn remove_copper(&mut self, amount: f32);
}

impl WealthManagement for Wealth {
    fn add_copper(&mut self, amount: f32) {
        self.copper += f32::Coin::new::<coin::copper>(amount);
    }
    fn remove_copper(&mut self, amount: f32) {
        self.copper -= f32::Coin::new::<coin::copper>(amount);
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn load_races_from_file(file_path: &'static str) -> Vec<RaceInfo> {
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

fn export_characters_to_file(
    characters: Vec<Character>,
    file_path: &'static str,
) -> Result<(), io::Error> {
    let file_path = Path::new(file_path);
    let directory = file_path.parent().unwrap();

    if !directory.exists() {
        std::fs::create_dir(directory).unwrap();
    }

    let characters_output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let result = serde_yaml::to_writer(&characters_output_file, &characters).unwrap();

    return Ok(result);
}

fn load_characters_from_file(file_path: &'static str) -> Result<Vec<Character>, serde_yaml::Error> {
    let characters_import_file = OpenOptions::new().read(true).open(file_path).unwrap();

    let result = serde_yaml::from_reader(&characters_import_file)
        .expect("Can't import the characters data by deserializing.");

    Ok(result)
}

fn export_armor_to_file(armors: Vec<Armor>, file_path: &'static str) -> Result<(), io::Error> {
    let file_path = Path::new(file_path);
    let directory = file_path.parent().unwrap();

    if !directory.exists() {
        std::fs::create_dir(directory).unwrap();
    }

    let armors_export_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let result = serde_yaml::to_writer(&armors_export_file, &armors).unwrap();

    return Ok(result);
}

fn load_armor_from_file(file_path: &'static str) -> Result<Vec<Armor>, serde_yaml::Error> {
    let armors_import_file = OpenOptions::new().read(true).open(file_path).unwrap();

    let result = serde_yaml::from_reader(&armors_import_file)
        .expect("Can't import the armors data by deserializing.");

    Ok(result)
}

fn main() {
    let races = load_races_from_file("./data/races.yaml");
    println!("{:?}", races);
    let characters = load_characters_from_file("./data/characters.yaml").unwrap();

    println!("{:?}", characters);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_ability_score_calculation() {
        let score_modifier_table: [i8; 30] = [
            -5, // 1
            -4, // 2
            -4, // 3
            -3, // 4
            -3, // 4
            -2, // 6
            -2, // 7
            -1, // 8
            -1, // 0
            0,  // 10
            0,  // 11
            1,  // 12
            1,  // 13
            2,  // 14
            2,  // 15
            3,  // 16
            3,  // 17
            4,  // 18
            4,  // 19
            5,  // 20
            5,  // 21
            6,  // 22
            6,  // 23
            7,  // 24
            7,  // 25
            8,  // 26
            8,  // 27
            9,  // 28
            9,  // 29
            10, // 30
        ];
        for (index, &score_modifier) in score_modifier_table.iter().enumerate() {
            assert_eq!(
                derive_ability_modifier_from_ability_score(index as u8 + 1),
                score_modifier
            );
        }
    }

    // DnD OGL
    // Standard Exchange Rates
    // Coin      Abbr    CP   SP   EP    GP      PP
    // Copper    (cp)     1 1/10 1/50 1/100 1/1,000
    // Silver    (sp)    10    1  1/5  1/10   1/100
    // Electrum  (ep)    50    5    1   1/2    1/20
    // Gold      (gp)   100   10    2     1    1/10
    // Platinum  (pp) 1,000  100   20    10       1
    #[test]
    fn verify_standard_exchange_rate_conversions() {
        let copper_amount = 100.0;
        let silver_amount = 100.0;
        let electrum_amount = 100.0;
        let gold_amount = 100.0;
        let platinum_amount = 100.0;

        let wealth = Wealth {
            copper: f32::Coin::new::<coin::copper>(copper_amount),
            silver: f32::Coin::new::<coin::silver>(silver_amount),
            electrum: f32::Coin::new::<coin::electrum>(electrum_amount),
            gold: f32::Coin::new::<coin::gold>(gold_amount),
            platinum: f32::Coin::new::<coin::platinum>(platinum_amount),
        };

        assert_eq!(
            wealth.copper + wealth.silver + wealth.electrum + wealth.gold + wealth.platinum,
            f32::Coin::new::<coin::copper>(
                copper_amount * 1.0
                    + silver_amount * 10.0
                    + electrum_amount * 50.0
                    + gold_amount * 100.0
                    + platinum_amount * 1000.0
            )
        )
    }

    #[test]
    fn export_sample_armors() {
        let mut armors_export: Vec<Armor> = Vec::new();
        let armor = Armor {
            ability_requirement: None,
            armor_type: ArmorType::Leather,
            category: ArmorCategory::LightArmor,
            base_armor_class: 11,
            cost: f32::Coin::new::<coin::gold>(10.0),
            weight: 8,
            has_stealth_disadvantage: false,
        };

        armors_export.push(armor);

        export_armor_to_file(armors_export, "./serialize/armor.yaml").unwrap();
    }
    #[test]
    fn import_sample_armors() {
        let armors_import = load_armor_from_file("./data/armor.yaml").unwrap();

        let mut armors: Vec<Armor> = Vec::new();
        let armor = Armor {
            ability_requirement: None,
            armor_type: ArmorType::Leather,
            category: ArmorCategory::LightArmor,
            base_armor_class: 11,
            cost: f32::Coin::new::<coin::gold>(10.0),
            weight: 8,
            has_stealth_disadvantage: false,
        };

        armors.push(armor);

        // TODO(Kyler): Add PartialEq derive to rest of armor properties
        assert_eq!(
            armors_import[0].ability_requirement,
            armors[0].ability_requirement
        )
    }

    #[test]
    fn export_sample_characters() {
        let mut characters: Vec<Character> = Vec::new();

        let character = Character {
            name: String::from("Tishros"),
            experience_points: 0,
            level: 1,
            race: Race::Dwarf,
            class: vec![Class {
                class_type: ClassType::Barbarian,
                features: ClassFeatures {
                    hit_dice: Die { min: 0, max: 6 },
                    hit_points_starting: 0,
                    hit_points_from_level: Die { min: 0, max: 6 },
                },
            }],
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
                name: String::from("test"),
                description: String::from("Hello"),
                weapon_proficiency_modifiers: vec![],
                armor_proficiency_modifiers: vec![],
            }],
            roll_hit_points: false,
        };

        characters.push(character);

        export_characters_to_file(characters, "./serialize/characters.yaml").unwrap();
    }
    #[test]
    fn import_sample_characters() {
        let characters_import = load_characters_from_file("./data/characters.yaml").unwrap();

        let mut characters: Vec<Character> = Vec::new();

        let character = Character {
            name: String::from("Tishros"),
            experience_points: 0,
            level: 1,
            race: Race::Dwarf,
            class: vec![Class {
                class_type: ClassType::Barbarian,
                features: ClassFeatures {
                    hit_dice: Die { min: 0, max: 6 },
                    hit_points_starting: 0,
                    hit_points_from_level: Die { min: 0, max: 6 },
                },
            }],
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
                name: String::from("test"),
                description: String::from("Hello"),
                weapon_proficiency_modifiers: vec![],
                armor_proficiency_modifiers: vec![],
            }],
            roll_hit_points: false,
        };

        characters.push(character);

        // TODO(Kyler): Add PartialEq derive to rest of character properties
        assert_eq!(characters_import[0].name, characters[0].name)
    }
}
