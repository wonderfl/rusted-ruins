//! Miscellaneous type definitions

use crate::objholder::ItemIdx;
use std::ops::{Index, IndexMut};

/// Elements of damage/attack
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Element {
    None = -1,
    Physical = 0,
    Fire = 1,
    Cold = 2,
    Shock = 3,
    Poison = 4,
    Spirit = 5,
}

pub const ELEMENTS: [Element; Element::Spirit as usize + 1] = [
    Element::Physical,
    Element::Fire,
    Element::Cold,
    Element::Shock,
    Element::Poison,
    Element::Spirit,
];

/// This array has the same size as element types.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct ElementArray<T>(pub [T; Element::Spirit as usize + 1]);

impl<T> Index<Element> for ElementArray<T> {
    type Output = T;
    fn index(&self, e: Element) -> &T {
        assert_ne!(e, Element::None);
        &self.0[e as usize]
    }
}

impl<T> IndexMut<Element> for ElementArray<T> {
    fn index_mut(&mut self, e: Element) -> &mut T {
        assert_ne!(e, Element::None);
        &mut self.0[e as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CreationKind {
    Cooking,
}

/// A recipe for creation
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub product: String,
    pub ingredients: Vec<String>,
    pub difficulty: u32,
    pub required_time: CreationRequiredTime,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreationRequiredTime {
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MedicalEffect {
    None,
    Heal,
    Sleep,
    Poison,
}

impl Default for MedicalEffect {
    fn default() -> MedicalEffect {
        MedicalEffect::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MagicalEffect {
    None,
    Arrow,
}

impl Default for MagicalEffect {
    fn default() -> MagicalEffect {
        MagicalEffect::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolEffect {
    None,
    Build,
}

impl Default for ToolEffect {
    fn default() -> ToolEffect {
        ToolEffect::None
    }
}

/// Reward for quests or events
#[derive(Clone, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct Reward {
    pub money: i64,
    pub item: Vec<ItemIdx>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Harvest {
    pub harvest_type: HarvestType,
    pub target_item: String,
    pub difficulty: u32,
    pub n_yield: u32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarvestType {
    Animal,
    Chop,
    Crop,
    Deconstruct,
    Mine,
}
