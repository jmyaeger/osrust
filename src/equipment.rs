use lazy_static::lazy_static;

use rusqlite::{Connection, Result, Row};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use strum_macros::EnumIter;

lazy_static! {
    static ref EQUIPMENT_DB: PathBuf = {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        current_dir.join("src/databases/equipment.db")
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub enum GearSlot {
    #[default]
    None,
    Head,
    Neck,
    Body,
    Legs,
    Hands,
    Feet,
    Ring,
    Ammo,
    Weapon,
    Shield,
    Cape,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone, EnumIter)]
pub enum CombatType {
    None,
    Stab,
    Slash,
    #[default]
    Crush,
    Ranged,
    Magic,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone)]
pub enum CombatStance {
    None,
    #[default]
    Accurate,
    Aggressive,
    Defensive,
    Controlled,
    Rapid,
    Longrange,
    ShortFuse,
    MediumFuse,
    LongFuse,
    DefensiveAutocast,
    Autocast,
    ManualCast,
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub enum CombatStyle {
    Chop,
    Slash,
    Smash,
    Block,
    Hack,
    Lunge,
    Swipe,
    Pound,
    Pummel,
    Spike,
    Impale,
    Stab,
    Jab,
    Fend,
    Bash,
    Reap,
    #[default]
    Punch,
    Kick,
    Flick,
    Lash,
    Deflect,
    Accurate,
    Rapid,
    Longrange,
    ShortFuse,
    MediumFuse,
    LongFuse,
    DefensiveSpell,
    Spell,
    Scorch,
    Flare,
    Blaze,
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct CombatOption {
    pub combat_type: CombatType,
    pub stance: CombatStance,
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct StyleBonus {
    pub stab: i32,
    pub slash: i32,
    pub crush: i32,
    pub ranged: i32,
    pub magic: i32,
}

impl StyleBonus {
    pub fn add_bonuses(&mut self, other: &StyleBonus) {
        self.stab += other.stab;
        self.slash += other.slash;
        self.crush += other.crush;
        self.ranged += other.ranged;
        self.magic += other.magic;
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct StrengthBonus {
    pub melee: i32,
    pub ranged: i32,
    pub magic: f32,
}

impl StrengthBonus {
    pub fn add_bonuses(&mut self, other: &StrengthBonus) {
        self.melee += other.melee;
        self.ranged += other.ranged;
        self.magic += other.magic;
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct EquipmentBonuses {
    pub attack: StyleBonus,
    pub defence: StyleBonus,
    pub strength: StrengthBonus,
    pub prayer: i32,
}

impl EquipmentBonuses {
    pub fn add_bonuses(&mut self, other: &EquipmentBonuses) {
        self.attack.add_bonuses(&other.attack);
        self.defence.add_bonuses(&other.defence);
        self.strength.add_bonuses(&other.strength);
        self.prayer += other.prayer;
    }
}

pub trait Equipment {
    fn set_info(&mut self, item_name: &str) -> Result<()> {
        let conn = Connection::open(EQUIPMENT_DB.as_path())?;
        let mut stmt = conn.prepare("SELECT * FROM equipment WHERE name = ?")?;
        let mut rows = stmt.query([&item_name])?;
        if let Some(row) = rows.next()? {
            self.set_fields_from_row(row)?;
            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    fn set_fields_from_row(&mut self, row: &Row) -> Result<()>;
}

#[derive(Debug, PartialEq, Default)]
pub struct Armor {
    pub name: String,
    pub bonuses: EquipmentBonuses,
    pub slot: GearSlot,
}

impl Equipment for Armor {
    fn set_fields_from_row(&mut self, row: &Row) -> Result<()> {
        self.name = row.get("name")?;
        self.bonuses.attack.stab = row.get::<_, i32>("astab")?;
        self.bonuses.attack.slash = row.get::<_, i32>("aslash")?;
        self.bonuses.attack.crush = row.get::<_, i32>("acrush")?;
        self.bonuses.attack.ranged = row.get::<_, i32>("arange")?;
        self.bonuses.attack.magic = row.get::<_, i32>("amagic")?;
        self.bonuses.defence.stab = row.get::<_, i32>("dstab")?;
        self.bonuses.defence.slash = row.get::<_, i32>("dslash")?;
        self.bonuses.defence.crush = row.get::<_, i32>("dcrush")?;
        self.bonuses.defence.ranged = row.get::<_, i32>("drange")?;
        self.bonuses.defence.magic = row.get::<_, i32>("dmagic")?;
        self.bonuses.strength.melee = row.get::<_, i32>("str")?;
        self.bonuses.strength.ranged = row.get::<_, i32>("rstr")?;
        self.bonuses.strength.magic = row.get::<_, f32>("mdmg")?;
        self.bonuses.prayer = row.get::<_, i32>("prayer")?;
        self.slot = match row.get::<_, String>("slot")?.as_str() {
            "head" => GearSlot::Head,
            "neck" => GearSlot::Neck,
            "body" => GearSlot::Body,
            "legs" => GearSlot::Legs,
            "hands" => GearSlot::Hands,
            "feet" => GearSlot::Feet,
            "ring" => GearSlot::Ring,
            "ammo" => GearSlot::Ammo,
            "shield" => GearSlot::Shield,
            "cape" => GearSlot::Cape,
            _ => panic!("Invalid slot: {}", row.get::<_, String>("slot")?),
        };
        Ok(())
    }
}

impl Armor {
    pub fn new(name: &str) -> Self {
        let mut armor = Armor::default();
        armor.set_info(name).unwrap();
        armor
    }
}

#[derive(Debug, PartialEq)]
pub struct Weapon {
    pub name: String,
    pub bonuses: EquipmentBonuses,
    pub slot: GearSlot,
    pub speed: i8,
    pub base_speed: i8,
    pub attack_range: i8,
    pub two_handed: bool,
    pub spec_cost: u8,
    pub poison_severity: u8,
    pub combat_styles: HashMap<CombatStyle, CombatOption>,
    pub is_staff: bool,
}

impl Equipment for Weapon {
    fn set_fields_from_row(&mut self, row: &Row) -> Result<()> {
        self.name = row.get("name")?;
        self.bonuses.attack.stab = row.get::<_, i32>("astab")?;
        self.bonuses.attack.slash = row.get::<_, i32>("aslash")?;
        self.bonuses.attack.crush = row.get::<_, i32>("acrush")?;
        self.bonuses.attack.ranged = row.get::<_, i32>("arange")?;
        self.bonuses.attack.magic = row.get::<_, i32>("amagic")?;
        self.bonuses.defence.stab = row.get::<_, i32>("dstab")?;
        self.bonuses.defence.slash = row.get::<_, i32>("dslash")?;
        self.bonuses.defence.crush = row.get::<_, i32>("dcrush")?;
        self.bonuses.defence.ranged = row.get::<_, i32>("drange")?;
        self.bonuses.defence.magic = row.get::<_, i32>("dmagic")?;
        self.bonuses.strength.melee = row.get::<_, i32>("str")?;
        self.bonuses.strength.ranged = row.get::<_, i32>("rstr")?;
        self.bonuses.strength.magic = row.get::<_, f32>("mdmg")?;
        self.bonuses.prayer = row.get::<_, i32>("prayer")?;
        self.slot = GearSlot::Weapon;
        self.speed = row.get::<_, i8>("speed")?;
        self.base_speed = self.speed;
        self.attack_range = match row.get::<_, i8>("attackrange") {
            Ok(range) => range,
            Err(rusqlite::Error::InvalidColumnType(_, _, _)) => {
                match row.get::<_, String>("attackrange")?.as_str() {
                    "staff" => {
                        self.is_staff = true;
                        1
                    }
                    _ => panic!(
                        "Invalid attack range: {}",
                        row.get::<_, String>("attackrange")?
                    ),
                }
            }
            Err(e) => panic!("Error parsing attack range: {}", e),
        };
        self.two_handed = matches!(row.get::<_, String>("slot")?.as_str(), "2h");
        let weapon_type = row.get::<_, String>("combatstyle")?;
        self.combat_styles = Weapon::get_styles_from_weapon_type(weapon_type.as_str());
        Ok(())
    }
}

impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            name: String::new(),
            bonuses: EquipmentBonuses::default(),
            slot: GearSlot::Weapon,
            speed: 0,
            base_speed: 0,
            attack_range: 0,
            two_handed: false,
            spec_cost: 0,
            poison_severity: 0,
            combat_styles: Weapon::get_styles_from_weapon_type("Unarmed"),
            is_staff: false,
        }
    }
}

impl Weapon {
    pub fn new(name: &str) -> Self {
        let mut weapon = Weapon::default();
        weapon.set_info(name).unwrap();
        weapon
    }

    pub fn get_styles_from_weapon_type(weapon_type: &str) -> HashMap<CombatStyle, CombatOption> {
        match weapon_type {
            "2h Sword" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Axe" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Hack,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Banner" => HashMap::from([
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Blunt" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Bludgeon" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
            ]),
            "Bulwark" => HashMap::from([
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::None,
                        stance: CombatStance::None,
                    },
                ),
            ]),
            "Claw" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Partisan" => HashMap::from([
                (
                    CombatStyle::Stab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Pickaxe" => HashMap::from([
                (
                    CombatStyle::Spike,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Impale,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Polearm" => HashMap::from([
                (
                    CombatStyle::Jab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Fend,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Polestaff" => HashMap::from([
                (
                    CombatStyle::Bash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Scythe" => HashMap::from([
                (
                    CombatStyle::Reap,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Jab,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Slash Sword" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Spear" => HashMap::from([
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Spiked" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Spike,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Stab Sword" => HashMap::from([
                (
                    CombatStyle::Stab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Unarmed" => HashMap::from([
                (
                    CombatStyle::Punch,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Kick,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Whip" => HashMap::from([
                (
                    CombatStyle::Flick,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Lash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Deflect,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Bow" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Rapid,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::Rapid,
                    },
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::Longrange,
                    },
                ),
            ]),
            "Chinchompa" => HashMap::from([
                (
                    CombatStyle::ShortFuse,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::ShortFuse,
                    },
                ),
                (
                    CombatStyle::MediumFuse,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::MediumFuse,
                    },
                ),
                (
                    CombatStyle::LongFuse,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::LongFuse,
                    },
                ),
            ]),
            "Bladed Staff" => HashMap::from([
                (
                    CombatStyle::Jab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Fend,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
                (
                    CombatStyle::DefensiveSpell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::DefensiveAutocast,
                    },
                ),
                (
                    CombatStyle::Spell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Autocast,
                    },
                ),
            ]),
            "Powered Staff" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Longrange,
                    },
                ),
            ]),
            "Staff" => HashMap::from([
                (
                    CombatStyle::Bash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Fend,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
                (
                    CombatStyle::DefensiveSpell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::DefensiveAutocast,
                    },
                ),
                (
                    CombatStyle::Spell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Autocast,
                    },
                ),
            ]),
            "Salamander" => HashMap::from([
                (
                    CombatStyle::Scorch,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Flare,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Blaze,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            _ => HashMap::new(),
        }
    }
}

pub fn get_slot_name(item_name: &str) -> Result<String> {
    let conn = Connection::open(EQUIPMENT_DB.as_path()).unwrap();
    let mut stmt = conn.prepare("SELECT slot FROM equipment WHERE name = ?")?;
    let mut rows = stmt.query([&item_name])?;
    if let Some(row) = rows.next()? {
        Ok(row.get(0)?)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_weapon() {
        let weapon = Weapon::default();
        assert_eq!(weapon.name, "");
        assert_eq!(weapon.bonuses, EquipmentBonuses::default());
        assert_eq!(weapon.speed, 0);
        assert_eq!(weapon.base_speed, 0);
        assert_eq!(weapon.attack_range, 0);
        assert!(!weapon.two_handed);
        assert_eq!(weapon.spec_cost, 0);
        assert_eq!(weapon.slot, GearSlot::Weapon);
        let combat_style = weapon.combat_styles.get(&CombatStyle::Punch).unwrap();
        assert_eq!(combat_style.combat_type, CombatType::Crush);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }

    #[test]
    fn test_default_armor() {
        let armor = Armor::default();
        assert_eq!(armor.name, "");
        assert_eq!(armor.bonuses, EquipmentBonuses::default());
        assert_eq!(armor.slot, GearSlot::None);
    }

    #[test]
    fn test_set_weapon_info() {
        let weapon = Weapon::new("Abyssal whip");
        assert_eq!(weapon.name, "Abyssal whip");
        assert_eq!(weapon.slot, GearSlot::Weapon);
        assert_eq!(weapon.bonuses.attack.slash, 82);
        assert_eq!(weapon.bonuses.strength.melee, 82);
        let combat_style = weapon.combat_styles.get(&CombatStyle::Flick).unwrap();
        assert_eq!(combat_style.combat_type, CombatType::Slash);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }
}
