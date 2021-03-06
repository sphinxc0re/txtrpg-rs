use item::{Item, ItemType, ItemInfluence};
use std::collections::HashMap;
use inventory::Inventory;
use types::{Health, AttributeValue};

/// The influence the `Attribute::Dexterity` has on the attack_damage of the character
const DEXTERITY_INFLUENCE: f64 = 0.2;

/// The character the player is impersonating
pub struct Character {
    name: String,
    health: Health,
    attributes: HashMap<Attribute, AttributeValue>,
    armor_slot_head: Option<Item>,
    armor_slot_chest: Option<Item>,
    armor_slot_legs: Option<Item>,
    armor_slot_feet: Option<Item>,
    weapon_slot_left: Option<Item>,
    weapon_slot_right: Option<Item>,
    inventory: Inventory,
}

impl Character {
    /// Constructs a new `Character`.
    ///
    /// By default, the character has an attribute set given by `Character::default_attributes()`.
    /// The characters default inventory size is `30`slots.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpg::character::Character;
    /// let character = Character::new("Michael");
    /// ```
    pub fn new(name: &str) -> Character {
        let attribute_map = Self::default_attributes();
        Character {
            name: name.to_owned(),
            health: (&attribute_map)[&Attribute::Constitution] as Health,
            attributes: attribute_map,
            armor_slot_head: None,
            armor_slot_chest: None,
            armor_slot_legs: None,
            armor_slot_feet: None,
            weapon_slot_left: None,
            weapon_slot_right: None,
            inventory: Inventory::new(30),
        }
    }

    /// Updates the given attribute
    pub fn update_attribute(&mut self, attribute: &Attribute, value: AttributeValue) {
        *self.attributes.get_mut(attribute).unwrap() = value;
    }

    /// Calculates and returns the current attack damage of the character based on the attibutes
    pub fn attack_damage(&self) -> AttributeValue {
        let base_dexterity = self.attributes
            .get(&Attribute::Dexterity)
            .expect("Unable to find attribute: Attribute::Dexterity");

        let base_dexterity = ((*base_dexterity as f64) * DEXTERITY_INFLUENCE) as AttributeValue;

        let base_strength = self.attributes
            .get(&Attribute::Strength)
            .expect("Unable to find attribute: Attribute::Strength");

        let mut additional_damage: i64 = 0;
        if let Some(ref inner_item) = self.weapon_slot_left {
            if let Some(ItemInfluence { ref attribute, ref amount }) = inner_item.influence {
                let influence = if attribute == &Attribute::Dexterity {
                    DEXTERITY_INFLUENCE
                } else {
                    1_f64
                };

                additional_damage += ((*amount as f64) * influence) as i64;
            }
        }

        if let Some(ref inner_item) = self.weapon_slot_right {
            if let Some(ItemInfluence { ref attribute, ref amount }) = inner_item.influence {
                let influence = if attribute == &Attribute::Dexterity {
                    DEXTERITY_INFLUENCE
                } else {
                    1_f64
                };

                additional_damage += ((*amount as f64) * influence) as i64;
            }

        }

        base_strength + base_dexterity + additional_damage
    }

    /// Returns the value of the specified attribute
    pub fn get_attribute_value(&self, attribute: &Attribute) -> AttributeValue {
        *self.attributes.get(attribute).unwrap()
    }

    /// A setter method for the head armor slot.
    ///
    /// # Panics
    ///
    /// **Panics** whether the given item is not of type `ItemType::ArmorHead`
    pub fn set_armor_slot_head(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::ArmorHead);
        }

        self.armor_slot_head = item;
    }

    /// A setter method for the chest armor slot.
    ///
    /// # Panics
    ///
    /// **Panics** whether the given item is not of type `ItemType::ArmorChest`
    pub fn set_armor_slot_chest(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::ArmorChest);
        }

        self.armor_slot_chest = item;
    }

    /// A setter method for the legs armor slot.
    ///
    /// # Panics
    ///
    /// **Panics** whether the given item is not
    /// of type `ItemType::ArmorLegs`
    pub fn set_armor_slot_legs(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::ArmorLegs);
        }

        self.armor_slot_legs = item;
    }

    /// A setter method for the feet armor slot.
    ///
    /// # Panics
    ///
    /// **Panics** whether the given item is not
    /// of type `ItemType::ArmorFeet`
    pub fn set_armor_slot_feet(&mut self, item: Option<Item>) {
        if let Some(ref inner_item) = item {
            assert_eq!(inner_item.item_type, ItemType::ArmorFeet);
        }

        self.armor_slot_feet = item;
    }

    /// A setter method for the right weapon slot
    pub fn set_weapon_slot_right(&mut self, item: Option<Item>) {
        self.weapon_slot_right = item;
    }

    /// A setter method for the left weapon slot
    pub fn set_weapon_slot_left(&mut self, item: Option<Item>) {
        self.weapon_slot_left = item;
    }

    /// Returns the default attributes for a character
    pub fn default_attributes() -> HashMap<Attribute, AttributeValue> {
        let mut attribute_map = HashMap::new();

        attribute_map.insert(Attribute::Charisma, 5);
        attribute_map.insert(Attribute::Constitution, 30);
        attribute_map.insert(Attribute::Defense, 15);
        attribute_map.insert(Attribute::Dexterity, 10);
        attribute_map.insert(Attribute::Intelligence, 5);
        attribute_map.insert(Attribute::Luck, 0);
        attribute_map.insert(Attribute::Perception, 10);
        attribute_map.insert(Attribute::Strength, 20);
        attribute_map.insert(Attribute::Willpower, 15);
        attribute_map.insert(Attribute::Wisdom, 5);

        attribute_map
    }
}

/// A list of all possible attributes
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Attribute {
    /// The charisma of a character
    Charisma,
    /// The constitution of a character
    Constitution,
    /// The defense of a character
    Defense,
    /// The dexterity of a character
    Dexterity,
    /// The intelligence of a character
    Intelligence,
    /// The luck of a character
    Luck,
    /// The perception of a character
    Perception,
    /// The strength of a character
    Strength,
    /// The willpower of a character
    Willpower,
    /// The wisdom of a character
    Wisdom,
}


#[cfg(test)]
mod tests {
    use super::*;

    use item_generator;
    use item::{ItemType, ItemInfluence};

    #[test]
    fn set_armor_slot_head() {
        let mut character = Character::new("TestCharacter");

        assert_eq!(character.armor_slot_head, None);

        let head_piece = item_generator::ItemGenerator::new().item_type(ItemType::ArmorHead).gen();
        let head_piece_clone = head_piece.clone();

        character.set_armor_slot_head(Some(head_piece));

        assert_eq!(character.armor_slot_head, Some(head_piece_clone));
    }

    #[test]
    fn set_armor_slot_chest() {
        let mut character = Character::new("TestCharacter");

        assert_eq!(character.armor_slot_chest, None);

        let chest_piece =
            item_generator::ItemGenerator::new().item_type(ItemType::ArmorChest).gen();
        let chest_piece_clone = chest_piece.clone();

        character.set_armor_slot_chest(Some(chest_piece));

        assert_eq!(character.armor_slot_chest, Some(chest_piece_clone));
    }

    #[test]
    fn set_armor_slot_legs() {
        let mut character = Character::new("TestCharacter");

        assert_eq!(character.armor_slot_legs, None);

        let legs_piece = item_generator::ItemGenerator::new().item_type(ItemType::ArmorLegs).gen();
        let legs_piece_clone = legs_piece.clone();

        character.set_armor_slot_legs(Some(legs_piece));

        assert_eq!(character.armor_slot_legs, Some(legs_piece_clone));
    }

    #[test]
    fn set_armor_slot_feet() {
        let mut character = Character::new("TestCharacter");

        assert_eq!(character.armor_slot_feet, None);

        let shoes_piece = item_generator::ItemGenerator::new().item_type(ItemType::ArmorFeet).gen();
        let shoes_piece_clone = shoes_piece.clone();

        character.set_armor_slot_feet(Some(shoes_piece));

        assert_eq!(character.armor_slot_feet, Some(shoes_piece_clone));
    }

    #[test]
    fn set_weapon_slot_right() {
        let mut character = Character::new("TestCharacter");

        assert_eq!(character.weapon_slot_right, None);

        let weapon = item_generator::ItemGenerator::new().item_type(ItemType::WeaponHammer).gen();
        let weapon_clone = weapon.clone();

        character.set_weapon_slot_right(Some(weapon));

        assert_eq!(character.weapon_slot_right, Some(weapon_clone));
    }

    #[test]
    fn set_weapon_slot_left() {
        let mut character = Character::new("TestCharacter");

        assert_eq!(character.weapon_slot_left, None);

        let weapon = item_generator::ItemGenerator::new().item_type(ItemType::WeaponSword).gen();
        let weapon_clone = weapon.clone();

        character.set_weapon_slot_left(Some(weapon));

        assert_eq!(character.weapon_slot_left, Some(weapon_clone));
    }

    #[test]
    fn attribute_mutation() {
        let mut character = Character::new("Wil Wheaton");

        character.update_attribute(&Attribute::Dexterity, 42);

        assert_eq!(character.get_attribute_value(&Attribute::Dexterity), 42);
    }

    #[test]
    fn basic_attack_damage() {
        let character = Character::new("Wil Wheaton");

        // 22 is the very basic attack damage
        assert_eq!(character.attack_damage(), 22);
    }

    #[test]
    fn attack_damage_with_weapons() {
        let mut character = Character::new("Wil Wheaton");

        let weapon = item_generator::ItemGenerator::new()
            .item_type(ItemType::WeaponSword)
            .influence(Some(ItemInfluence::new(Attribute::Strength, 10)))
            .gen();

        character.set_weapon_slot_left(Some(weapon.clone()));
        character.set_weapon_slot_right(Some(weapon.clone()));

        assert_eq!(character.attack_damage(), 42);
    }
}
