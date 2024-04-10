use lazy_static::lazy_static;

use rusqlite::{Connection, Result, Row};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

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

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub enum CombatType {
    None,
    Stab,
    Slash,
    #[default]
    Crush,
    Ranged,
    Magic,
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
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

#[derive(Debug, PartialEq, Default)]
pub struct StrengthBonus {
    pub melee: i32,
    pub ranged: i32,
    pub magic: f32,
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
    pub att_bonuses: StyleBonus,
    pub def_bonuses: StyleBonus,
    pub str_bonuses: StrengthBonus,
    pub prayer: i32,
    pub slot: GearSlot,
}

impl Equipment for Armor {
    fn set_fields_from_row(&mut self, row: &Row) -> Result<()> {
        self.name = row.get("name")?;
        self.att_bonuses.stab = row.get::<_, i32>("astab")?;
        self.att_bonuses.slash = row.get::<_, i32>("aslash")?;
        self.att_bonuses.crush = row.get::<_, i32>("acrush")?;
        self.att_bonuses.ranged = row.get::<_, i32>("arange")?;
        self.att_bonuses.magic = row.get::<_, i32>("amagic")?;
        self.def_bonuses.stab = row.get::<_, i32>("dstab")?;
        self.def_bonuses.slash = row.get::<_, i32>("dslash")?;
        self.def_bonuses.crush = row.get::<_, i32>("dcrush")?;
        self.def_bonuses.ranged = row.get::<_, i32>("drange")?;
        self.def_bonuses.magic = row.get::<_, i32>("dmagic")?;
        self.str_bonuses.melee = row.get::<_, i32>("str")?;
        self.str_bonuses.ranged = row.get::<_, i32>("rstr")?;
        self.str_bonuses.magic = row.get::<_, f32>("mdmg")?;
        self.prayer = row.get::<_, i32>("prayer")?;
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
    pub att_bonuses: StyleBonus,
    pub def_bonuses: StyleBonus,
    pub str_bonuses: StrengthBonus,
    pub prayer: i32,
    pub slot: GearSlot,
    pub speed: i8,
    pub base_speed: i8,
    pub attack_range: i8,
    pub two_handed: bool,
    pub spec_cost: u8,
    pub poison_severity: u8,
    pub combat_styles: HashMap<CombatStyle, CombatOption>,
}

impl Equipment for Weapon {
    fn set_fields_from_row(&mut self, row: &Row) -> Result<()> {
        self.name = row.get("name")?;
        self.att_bonuses.stab = row.get::<_, i32>("astab")?;
        self.att_bonuses.slash = row.get::<_, i32>("aslash")?;
        self.att_bonuses.crush = row.get::<_, i32>("acrush")?;
        self.att_bonuses.ranged = row.get::<_, i32>("arange")?;
        self.att_bonuses.magic = row.get::<_, i32>("amagic")?;
        self.def_bonuses.stab = row.get::<_, i32>("dstab")?;
        self.def_bonuses.slash = row.get::<_, i32>("dslash")?;
        self.def_bonuses.crush = row.get::<_, i32>("dcrush")?;
        self.def_bonuses.ranged = row.get::<_, i32>("drange")?;
        self.def_bonuses.magic = row.get::<_, i32>("dmagic")?;
        self.str_bonuses.melee = row.get::<_, i32>("str")?;
        self.str_bonuses.ranged = row.get::<_, i32>("rstr")?;
        self.str_bonuses.magic = row.get::<_, f32>("mdmg")?;
        self.prayer = row.get::<_, i32>("prayer")?;
        self.slot = GearSlot::Weapon;
        self.speed = row.get::<_, i8>("speed")?;
        self.base_speed = self.speed;
        self.attack_range = row.get::<_, i8>("attackrange")?;
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
            att_bonuses: StyleBonus::default(),
            def_bonuses: StyleBonus::default(),
            str_bonuses: StrengthBonus::default(),
            prayer: 0,
            slot: GearSlot::Weapon,
            speed: 0,
            base_speed: 0,
            attack_range: 0,
            two_handed: false,
            spec_cost: 0,
            poison_severity: 0,
            combat_styles: Weapon::get_styles_from_weapon_type("Unarmed"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_weapon() {
        let weapon = Weapon::default();
        assert_eq!(weapon.name, "");
        assert_eq!(weapon.att_bonuses, StyleBonus::default());
        assert_eq!(weapon.def_bonuses, StyleBonus::default());
        assert_eq!(weapon.str_bonuses, StrengthBonus::default());
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
        assert_eq!(armor.att_bonuses, StyleBonus::default());
        assert_eq!(armor.def_bonuses, StyleBonus::default());
        assert_eq!(armor.str_bonuses, StrengthBonus::default());
        assert_eq!(armor.prayer, 0);
        assert_eq!(armor.slot, GearSlot::None);
    }

    #[test]
    fn test_set_weapon_info() {
        let weapon = Weapon::new("Abyssal whip");
        assert_eq!(weapon.name, "Abyssal whip");
        assert_eq!(weapon.slot, GearSlot::Weapon);
        assert_eq!(weapon.att_bonuses.slash, 82);
        assert_eq!(weapon.str_bonuses.melee, 82);
        let combat_style = weapon.combat_styles.get(&CombatStyle::Flick).unwrap();
        assert_eq!(combat_style.combat_type, CombatType::Slash);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }
}
