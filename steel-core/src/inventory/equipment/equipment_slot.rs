//! Equipment slot definitions for entities.

/// Equipment slot types for categorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlotType {
    /// Hand slots (main hand, off hand).
    Hand,
    /// Humanoid armor slots (head, chest, legs, feet).
    HumanoidArmor,
    /// Animal armor slot (body).
    AnimalArmor,
    /// Saddle slot.
    Saddle,
}

/// Equipment slots for entities.
///
/// Based on Minecraft's `EquipmentSlot` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    /// The main hand slot.
    MainHand,
    /// The off hand slot.
    OffHand,
    /// The feet armor slot (boots).
    Feet,
    /// The legs armor slot (leggings).
    Legs,
    /// The chest armor slot (chestplate).
    Chest,
    /// The head armor slot (helmet).
    Head,
    /// The body armor slot (for animals like horses).
    Body,
    /// The saddle slot (for rideable animals).
    Saddle,
}

impl EquipmentSlot {
    /// All equipment slots in order.
    pub const ALL: [Self; 8] = [
        Self::MainHand,
        Self::OffHand,
        Self::Feet,
        Self::Legs,
        Self::Chest,
        Self::Head,
        Self::Body,
        Self::Saddle,
    ];

    /// Humanoid armor slots (head, chest, legs, feet).
    pub const ARMOR_SLOTS: [Self; 4] = [
        Self::Head,
        Self::Chest,
        Self::Legs,
        Self::Feet,
    ];

    /// Returns the slot type for this equipment slot.
    #[must_use]
    pub const fn slot_type(self) -> EquipmentSlotType {
        match self {
            Self::MainHand | Self::OffHand => EquipmentSlotType::Hand,
            Self::Feet
            | Self::Legs
            | Self::Chest
            | Self::Head => EquipmentSlotType::HumanoidArmor,
            Self::Body => EquipmentSlotType::AnimalArmor,
            Self::Saddle => EquipmentSlotType::Saddle,
        }
    }

    /// Returns the index of this slot for array storage (0-7).
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 1,
            Self::Feet => 2,
            Self::Legs => 3,
            Self::Chest => 4,
            Self::Head => 5,
            Self::Body => 6,
            Self::Saddle => 7,
        }
    }

    /// Returns true if this is an armor slot (humanoid or animal).
    #[must_use]
    pub const fn is_armor(self) -> bool {
        matches!(
            self.slot_type(),
            EquipmentSlotType::HumanoidArmor | EquipmentSlotType::AnimalArmor
        )
    }

    /// Returns the equipment slot with the given name, or None if not found.
    #[must_use]
    pub fn by_name(name: &str) -> Option<Self> {
        match name {
            "mainhand" => Some(Self::MainHand),
            "offhand" => Some(Self::OffHand),
            "feet" => Some(Self::Feet),
            "legs" => Some(Self::Legs),
            "chest" => Some(Self::Chest),
            "head" => Some(Self::Head),
            "body" => Some(Self::Body),
            "saddle" => Some(Self::Saddle),
            _ => None,
        }
    }

    /// Returns the name of this equipment slot.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::MainHand => "mainhand",
            Self::OffHand => "offhand",
            Self::Feet => "feet",
            Self::Legs => "legs",
            Self::Chest => "chest",
            Self::Head => "head",
            Self::Body => "body",
            Self::Saddle => "saddle",
        }
    }
}
