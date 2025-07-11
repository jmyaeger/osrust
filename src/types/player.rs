use crate::calc::rolls::calc_active_player_rolls;
use crate::combat::attacks::effects::CombatEffect;
use crate::combat::attacks::specs::{SpecialAttackFn, get_spec_attack_function};
use crate::combat::attacks::standard::{AttackFn, get_attack_functions, standard_attack};
use crate::constants::*;
use crate::types::equipment::{
    self, Armor, CombatStance, CombatStyle, CombatType, Equipment, EquipmentBonuses, Gear,
    GearSlot, Weapon,
};
use crate::types::monster::Monster;
use crate::types::potions::{Potion, PotionBoost, PotionBoosts, PotionStat};
use crate::types::prayers::PrayerBoosts;
use crate::types::spells;
use crate::types::stats::{PlayerStats, SpecEnergy, Stat};
use reqwest;
use std::cmp::max;
use std::collections::HashMap;

// Struct for holding sunfire rune min hit value
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct SunfireBoost {
    pub active: bool,
    pub min_hit: u32,
}

// Collection of effects that provide a damage boost in some cases
#[derive(Debug, PartialEq, Clone)]
pub struct StatusBoosts {
    pub on_task: bool,
    pub in_wilderness: bool,
    pub in_multi: bool,
    pub forinthry_surge: bool,
    pub charge_active: bool,
    pub kandarin_diary: bool,
    pub mark_of_darkness: bool,
    pub first_attack: bool,
    pub acb_spec: bool,
    pub zcb_spec: bool,
    pub sunfire: SunfireBoost,
    pub soulreaper_stacks: u32,
    pub current_hp: Option<u32>,
}

impl Default for StatusBoosts {
    fn default() -> Self {
        Self {
            // Assume on task, not in wildy, and Kandarin Hard Diary unlocked
            on_task: true,
            in_wilderness: false,
            in_multi: false,
            forinthry_surge: false,
            charge_active: false,
            kandarin_diary: true,
            mark_of_darkness: false,
            first_attack: true,
            acb_spec: false,
            zcb_spec: false,
            sunfire: SunfireBoost::default(),
            soulreaper_stacks: 0,
            current_hp: None,
        }
    }
}

// Poison and venom effects - will likely rework this in the future
#[derive(Default, Debug, PartialEq, Clone)]
pub struct StatusEffects {
    pub poisoned: bool,
    pub venomed: bool,
    pub immune_poison: bool,
    pub immune_venom: bool,
    pub poison_severity: u8,
}

// Holds set effect data to avoid iterating through gear many times
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct SetEffects {
    pub full_void: bool,
    pub full_elite_void: bool,
    pub full_justiciar: bool,
    pub full_inquisitor: bool,
    pub full_dharoks: bool,
    pub full_torags: bool,
    pub full_guthans: bool,
    pub full_veracs: bool,
    pub full_karils: bool,
    pub full_ahrims: bool,
    pub full_obsidian: bool,
    pub full_blood_moon: bool,
    pub full_blue_moon: bool,
    pub full_eclipse_moon: bool,
    pub bloodbark_pieces: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SwitchType {
    Melee,
    Ranged,
    Magic,
    Spec(String),
    Custom(String),
}

impl SwitchType {
    pub fn label(&self) -> String {
        match self {
            SwitchType::Melee => "Melee".to_string(),
            SwitchType::Ranged => "Ranged".to_string(),
            SwitchType::Magic => "Magic".to_string(),
            SwitchType::Spec(spec_label) => format!("{spec_label} spec"),
            SwitchType::Custom(custom_label) => custom_label.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GearSwitch {
    pub switch_type: SwitchType,
    pub gear: Gear,
    pub prayers: PrayerBoosts,
    pub spell: Option<spells::Spell>,
    pub active_style: CombatStyle,
    pub set_effects: SetEffects,
    pub attack: AttackFn,
    pub spec: SpecialAttackFn,
    pub att_rolls: PlayerAttRolls,
    pub max_hits: PlayerMaxHits,
    pub def_rolls: PlayerDefRolls,
}

impl GearSwitch {
    pub fn new(switch_type: SwitchType, player: &Player, monster: &Monster) -> Self {
        let mut player_copy = player.clone();
        player_copy.update_bonuses();
        player_copy.update_set_effects();
        calc_active_player_rolls(&mut player_copy, monster);

        let attack = get_attack_functions(&player_copy);
        let spec = get_spec_attack_function(&player_copy);

        Self {
            switch_type,
            gear: player_copy.gear,
            prayers: player_copy.prayers,
            spell: player_copy.attrs.spell,
            active_style: player_copy.attrs.active_style,
            set_effects: player_copy.set_effects,
            attack,
            spec,
            att_rolls: player_copy.att_rolls,
            max_hits: player_copy.max_hits,
            def_rolls: player_copy.def_rolls,
        }
    }
}

impl From<&Player> for GearSwitch {
    fn from(player: &Player) -> Self {
        let switch_type = match player.combat_type() {
            CombatType::Crush | CombatType::Slash | CombatType::Stab => SwitchType::Melee,
            CombatType::Ranged | CombatType::Heavy | CombatType::Light | CombatType::Standard => {
                SwitchType::Ranged
            }
            CombatType::Magic => SwitchType::Magic,
            _ => SwitchType::Custom("Unknown".to_string()),
        };
        let attack = get_attack_functions(player);
        let spec = get_spec_attack_function(player);

        Self {
            switch_type,
            gear: player.gear.clone(),
            prayers: player.prayers.clone(),
            spell: player.attrs.spell,
            active_style: player.attrs.active_style,
            set_effects: player.set_effects,
            attack,
            spec,
            att_rolls: player.att_rolls,
            max_hits: player.max_hits,
            def_rolls: player.def_rolls,
        }
    }
}

// Misc other player info - may restructure if there's a better place for these
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PlayerAttrs {
    pub name: Option<String>,
    pub active_style: CombatStyle,
    pub spell: Option<spells::Spell>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PlayerAttRolls {
    stab: i32,
    slash: i32,
    crush: i32,
    light: i32,
    standard: i32,
    heavy: i32,
    magic: i32,
}

impl PlayerAttRolls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, combat_type: CombatType) -> i32 {
        match combat_type {
            CombatType::Stab => self.stab,
            CombatType::Slash => self.slash,
            CombatType::Crush => self.crush,
            CombatType::Light => self.light,
            CombatType::Standard => self.standard,
            CombatType::Heavy => self.heavy,
            CombatType::Ranged => panic!("Players do not have generic ranged attack rolls"),
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Light => self.light = value,
            CombatType::Standard => self.standard = value,
            CombatType::Heavy => self.heavy = value,
            CombatType::Ranged => panic!("Players do not have generic ranged attack rolls"),
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PlayerDefRolls {
    stab: i32,
    slash: i32,
    crush: i32,
    ranged: i32,
    magic: i32,
}

impl PlayerDefRolls {
    pub fn get(&self, combat_type: CombatType) -> i32 {
        match combat_type {
            CombatType::Stab => self.stab,
            CombatType::Slash => self.slash,
            CombatType::Crush => self.crush,
            CombatType::Ranged => self.ranged,
            CombatType::Light => self.ranged,
            CombatType::Standard => self.ranged,
            CombatType::Heavy => self.ranged,
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Ranged => self.ranged = value,
            CombatType::Light => self.ranged = value,
            CombatType::Standard => self.ranged = value,
            CombatType::Heavy => self.ranged = value,
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PlayerMaxHits {
    stab: u32,
    slash: u32,
    crush: u32,
    light: u32,
    standard: u32,
    heavy: u32,
    magic: u32,
}

impl PlayerMaxHits {
    pub fn get(&self, combat_type: CombatType) -> u32 {
        match combat_type {
            CombatType::Stab => self.stab,
            CombatType::Slash => self.slash,
            CombatType::Crush => self.crush,
            CombatType::Light => self.light,
            CombatType::Standard => self.standard,
            CombatType::Heavy => self.heavy,
            CombatType::Ranged => self.standard, // All ranged max hits are the same
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: u32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Light => self.light = value,
            CombatType::Standard => self.standard = value,
            CombatType::Heavy => self.heavy = value,
            CombatType::Ranged => {
                self.standard = value;
                self.light = value;
                self.heavy = value;
            }
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub stats: PlayerStats,
    pub gear: Gear,
    pub bonuses: EquipmentBonuses,
    pub potions: PotionBoosts,
    pub prayers: PrayerBoosts,
    pub boosts: StatusBoosts,
    pub active_effects: Vec<CombatEffect>,
    pub set_effects: SetEffects,
    pub attrs: PlayerAttrs,
    pub att_rolls: PlayerAttRolls,
    pub max_hits: PlayerMaxHits,
    pub def_rolls: PlayerDefRolls,
    pub attack: AttackFn,
    pub spec: SpecialAttackFn,
    pub switches: Vec<GearSwitch>,
    pub current_switch: Option<SwitchType>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            stats: PlayerStats::default(),
            gear: Gear::default(),
            bonuses: EquipmentBonuses::default(),
            potions: PotionBoosts::default(),
            prayers: PrayerBoosts::default(),
            boosts: StatusBoosts::default(),
            active_effects: Vec::new(),
            set_effects: SetEffects::default(),
            attrs: PlayerAttrs::default(),
            att_rolls: PlayerAttRolls::default(),
            max_hits: PlayerMaxHits::default(),
            def_rolls: PlayerDefRolls::default(),
            attack: standard_attack,
            spec: standard_attack,
            switches: Vec::new(),
            current_switch: None,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn lookup_stats(&mut self, rsn: &str) {
        // Fetch stats from OSRS hiscores and set the corresponding fields
        let stats = fetch_player_data(rsn)
            .await
            .expect("Failed to fetch player data");
        self.stats = parse_player_data(stats);
        self.attrs.name = Some(rsn.to_string());
    }

    pub fn reset_current_stats(&mut self, include_spec: bool) {
        // Restore to base stats, full spec energy, and reapply potion boosts
        if !include_spec {
            let current_spec = self.stats.spec;
            self.stats.reset_all();
            self.stats.spec = current_spec;
        } else {
            self.stats.reset_all();
        }
        if let Some(hp) = self.boosts.current_hp {
            self.stats.hitpoints.current = hp;
        }
        self.apply_potion_boosts();
    }

    pub fn is_wearing(&self, gear_name: &str, version: Option<&str>) -> bool {
        // Check if the player is wearing the specified piece of gear
        let version = version.map(|v| v.to_string());

        self.gear.weapon.name == gear_name && self.gear.weapon.version == version
            || [
                &self.gear.head,
                &self.gear.neck,
                &self.gear.cape,
                &self.gear.ammo,
                &self.gear.shield,
                &self.gear.body,
                &self.gear.legs,
                &self.gear.feet,
                &self.gear.hands,
                &self.gear.ring,
            ]
            .iter()
            .filter_map(|slot| slot.as_ref())
            .any(|armor| armor.name == gear_name && armor.version == version)
    }

    pub fn is_wearing_any_version(&self, gear_name: &str) -> bool {
        // Same as is_wearing() but allows for any version to match
        self.gear.weapon.name == gear_name
            || [
                &self.gear.head,
                &self.gear.neck,
                &self.gear.cape,
                &self.gear.ammo,
                &self.gear.shield,
                &self.gear.body,
                &self.gear.legs,
                &self.gear.feet,
                &self.gear.hands,
                &self.gear.ring,
            ]
            .iter()
            .filter_map(|slot| slot.as_ref())
            .any(|armor| armor.name == gear_name)
    }

    pub fn is_wearing_any<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        // Check if the player is wearing any item in the provided Vec
        gear_names
            .into_iter()
            .any(|gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn is_wearing_all<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        // Check if the player is wearing all items in the provided Vec
        gear_names
            .into_iter()
            .all(|gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn equip(&mut self, item_name: &str, version: Option<&str>) {
        // Equip the specified item in the correct slot
        let slot_name = equipment::get_slot_name(item_name)
            .unwrap_or_else(|_| panic!("Slot not found for item {item_name}"));

        match slot_name.as_str() {
            "head" => self.gear.head = Some(Armor::new(item_name, version)),
            "neck" => self.gear.neck = Some(Armor::new(item_name, version)),
            "cape" => {
                self.gear.cape = Some(Armor::new(item_name, version));

                // For the quiver, apply extra +10 accuracy and +1 strength if applicable
                self.set_quiver_bonuses();
            }
            "ammo" => {
                // If quiver is equipped and the ammo slot is already full with a different ammo type,
                // equip the new ammo in the second_ammo slot
                if self.is_wearing_any_version("Dizana's quiver")
                    && (self.gear.ammo.is_some()
                        && !((self.gear.ammo.as_ref().unwrap().is_bolt()
                            && item_name.contains("bolts"))
                            || (self.gear.ammo.as_ref().unwrap().is_arrow()
                                && item_name.contains("arrow"))))
                {
                    self.gear.second_ammo = Some(Armor::new(item_name, version));

                    self.set_quiver_bonuses();
                } else {
                    self.gear.ammo = Some(Armor::new(item_name, version));

                    self.set_quiver_bonuses();
                }
            }
            "weapon" => {
                self.gear.weapon = Weapon::new(item_name, version);

                // Unequip shield if weapon is two handed
                if self.gear.weapon.is_two_handed {
                    self.gear.shield = None;
                }

                // Modify attack speed if weapon is on rapid
                if self.attrs.active_style == CombatStyle::Rapid
                    && self
                        .gear
                        .weapon
                        .combat_styles
                        .contains_key(&CombatStyle::Rapid)
                {
                    self.gear.weapon.speed = self.gear.weapon.base_speed - 1;
                }

                self.set_quiver_bonuses();
            }
            "shield" => {
                self.gear.shield = Some(Armor::new(item_name, version));

                // Unequip weapon if it is two handed
                if self.gear.weapon.is_two_handed {
                    self.gear.weapon = Weapon::default();
                }
            }
            "body" => self.gear.body = Some(Armor::new(item_name, version)),
            "legs" => self.gear.legs = Some(Armor::new(item_name, version)),
            "hands" => self.gear.hands = Some(Armor::new(item_name, version)),
            "feet" => self.gear.feet = Some(Armor::new(item_name, version)),
            "ring" => self.gear.ring = Some(Armor::new(item_name, version)),
            _ => panic!("Slot not found for item {item_name}"),
        }
        self.update_bonuses();
        self.update_set_effects();
    }

    pub fn unequip_slot(&mut self, slot: &GearSlot) {
        match slot {
            GearSlot::Ammo => self.gear.ammo = None,
            GearSlot::Body => self.gear.body = None,
            GearSlot::Cape => self.gear.cape = None,
            GearSlot::Feet => self.gear.feet = None,
            GearSlot::Hands => self.gear.hands = None,
            GearSlot::Head => self.gear.head = None,
            GearSlot::Legs => self.gear.legs = None,
            GearSlot::Neck => self.gear.neck = None,
            GearSlot::Ring => self.gear.ring = None,
            GearSlot::Shield => self.gear.shield = None,
            GearSlot::Weapon => self.gear.weapon = Weapon::default(),
            GearSlot::None => {}
        }
        self.update_bonuses();
        self.update_set_effects();
    }

    pub fn equip_item(
        &mut self,
        item: Box<dyn Equipment>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let slot = item.slot();
        match slot {
            GearSlot::Weapon => {
                if let Some(weapon) = item.as_any().downcast_ref::<Weapon>() {
                    self.gear.weapon = weapon.clone();

                    // Unequip shield if weapon is two handed
                    if self.gear.weapon.is_two_handed {
                        self.gear.shield = None;
                    }

                    // Modify attack speed if weapon is on rapid
                    if self.attrs.active_style == CombatStyle::Rapid
                        && self
                            .gear
                            .weapon
                            .combat_styles
                            .contains_key(&CombatStyle::Rapid)
                    {
                        self.gear.weapon.speed = self.gear.weapon.base_speed - 1;
                    }

                    self.set_quiver_bonuses();
                } else {
                    return Err("Expected weapon for weapon slot".into());
                }
            }
            GearSlot::Ammo => {
                // If quiver is equipped and the ammo slot is already full with a different ammo type,
                // equip the new ammo in the second_ammo slot
                if self.is_wearing_any_version("Dizana's quiver")
                    && (self.gear.ammo.is_some()
                        && !((self.gear.ammo.as_ref().unwrap().is_bolt()
                            && item.name().contains("bolts"))
                            || (self.gear.ammo.as_ref().unwrap().is_arrow()
                                && item.name().contains("arrow"))))
                {
                    self.gear.second_ammo = item.as_any().downcast_ref::<Armor>().cloned();

                    self.set_quiver_bonuses();
                } else {
                    self.gear.ammo = item.as_any().downcast_ref::<Armor>().cloned();

                    self.set_quiver_bonuses();
                }
            }
            GearSlot::Cape => {
                self.gear.cape = item.as_any().downcast_ref::<Armor>().cloned();
                self.set_quiver_bonuses();
            }
            GearSlot::Shield => {
                self.gear.shield = item.as_any().downcast_ref::<Armor>().cloned();
                if self.gear.weapon.is_two_handed {
                    self.gear.weapon = Weapon::default();
                }
            }
            GearSlot::Body => self.gear.body = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Feet => self.gear.feet = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Hands => self.gear.hands = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Head => self.gear.head = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Legs => self.gear.legs = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Neck => self.gear.neck = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Ring => self.gear.ring = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::None => {
                return Err(format!("{} has the slot type 'none'", item.name()).into());
            }
        }
        self.update_bonuses();
        self.update_set_effects();
        Ok(())
    }

    pub fn get_slot(&self, slot: &GearSlot) -> Option<Box<dyn Equipment>> {
        match slot {
            GearSlot::Weapon => Some(Box::new(self.gear.weapon.clone())),
            GearSlot::Ammo => self
                .gear
                .ammo
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Body => self
                .gear
                .body
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Cape => self
                .gear
                .cape
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Feet => self
                .gear
                .feet
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Hands => self
                .gear
                .hands
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Head => self
                .gear
                .head
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Legs => self
                .gear
                .legs
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Neck => self
                .gear
                .neck
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Ring => self
                .gear
                .ring
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Shield => self
                .gear
                .shield
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::None => None,
        }
    }

    fn set_quiver_bonuses(&mut self) {
        // Apply extra +10 accuracy and +1 strength to quiver if applicable
        if self.is_quiver_bonus_valid() {
            self.gear.cape.as_mut().unwrap().bonuses.attack.ranged = 28;
            self.gear.cape.as_mut().unwrap().bonuses.strength.ranged = 4;
        } else if self.is_wearing_any_version("Dizana's quiver") {
            self.gear.cape.as_mut().unwrap().bonuses.attack.ranged = 18;
            self.gear.cape.as_mut().unwrap().bonuses.strength.ranged = 3;
        }
    }

    pub fn update_bonuses(&mut self) {
        // Update equipment bonuses based on the equipped items
        self.bonuses = EquipmentBonuses::default();

        for item in [
            &self.gear.head,
            &self.gear.neck,
            &self.gear.cape,
            &self.gear.shield,
            &self.gear.body,
            &self.gear.legs,
            &self.gear.hands,
            &self.gear.feet,
            &self.gear.ring,
        ]
        .into_iter()
        .flatten()
        {
            self.bonuses.add_bonuses(&item.bonuses);
        }

        // Don't add ammo bonuses if the weapon uses its own ammo
        if !USES_OWN_AMMO.contains(&(
            self.gear.weapon.name.as_str(),
            self.gear.weapon.version.as_deref(),
        )) {
            for item in [&self.gear.ammo, &self.gear.second_ammo]
                .into_iter()
                .flatten()
            {
                self.bonuses.add_bonuses(&item.bonuses);
            }
        } else if self.gear.ammo.is_some()
            && !(self.gear.ammo.as_ref().unwrap().is_valid_ranged_ammo())
        {
            self.bonuses
                .add_bonuses(&self.gear.ammo.as_ref().unwrap().bonuses);
        }

        // Dinh's bulwark bonus is applied directly to gear strength bonus
        if self.is_wearing("Dinh's bulwark", None) && self.attrs.active_style == CombatStyle::Pummel
        {
            self.bonuses.strength.melee += self.bulwark_bonus();
        }

        self.bonuses.add_bonuses(&self.gear.weapon.bonuses);
    }

    pub fn update_set_effects(&mut self) {
        // Update status of all set effects at once
        self.set_effects.full_ahrims = self.is_wearing_all(FULL_AHRIMS);
        self.set_effects.full_blood_moon = self.is_wearing_all(FULL_BLOOD_MOON);
        self.set_effects.full_blue_moon = self.is_wearing_all(FULL_BLUE_MOON);
        self.set_effects.full_dharoks = self.is_wearing_all(FULL_DHAROKS);
        self.set_effects.full_guthans = self.is_wearing_all(FULL_GUTHANS);
        self.set_effects.full_eclipse_moon = self.is_wearing_all(FULL_ECLIPSE_MOON);
        self.set_effects.full_inquisitor = self.is_wearing_all(FULL_INQUISITOR);
        self.set_effects.full_justiciar = self.is_wearing_all(FULL_JUSTICIAR);
        self.set_effects.full_karils = self.is_wearing_all(FULL_KARILS);
        self.set_effects.full_obsidian = self.is_wearing_all(FULL_OBSIDIAN);
        self.set_effects.full_torags = self.is_wearing_all(FULL_TORAGS);
        self.set_effects.full_void = self.is_wearing_full_void();
        self.set_effects.full_elite_void = self.is_wearing_full_elite_void();
        self.set_effects.bloodbark_pieces = BLOODBARK_ARMOR
            .iter()
            .filter(|armor| self.is_wearing(armor.0, armor.1))
            .count();
    }

    pub fn calc_potion_boosts(&mut self) {
        // Calculate all of the selected potion boosts
        if let Some(potions) = &mut self.potions.attack {
            for potion in potions {
                if potion.potion_type == Potion::Moonlight {
                    potion.calc_moonlight_boost(
                        self.stats.attack,
                        self.stats.herblore,
                        PotionStat::Attack,
                    );
                } else if potion.potion_type == Potion::ZamorakBrew {
                    potion.calc_zamorak_brew_boost(self.stats.attack, PotionStat::Attack);
                } else {
                    potion.calc_boost(self.stats.attack);
                }
            }
        }
        if let Some(potions) = &mut self.potions.strength {
            for potion in potions {
                if potion.potion_type == Potion::Moonlight {
                    potion.calc_moonlight_boost(
                        self.stats.strength,
                        self.stats.herblore,
                        PotionStat::Strength,
                    );
                } else if potion.potion_type == Potion::DragonBattleaxe {
                    potion.calc_dragon_battleaxe_boost(
                        self.stats.attack,
                        self.stats.defence,
                        self.stats.ranged,
                        self.stats.magic,
                    );
                } else if potion.potion_type == Potion::ZamorakBrew {
                    potion.calc_zamorak_brew_boost(self.stats.strength, PotionStat::Strength);
                } else {
                    potion.calc_boost(self.stats.strength);
                }
            }
        }
        if let Some(potions) = &mut self.potions.defence {
            for potion in potions {
                if potion.potion_type == Potion::Moonlight {
                    potion.calc_moonlight_boost(
                        self.stats.defence,
                        self.stats.herblore,
                        PotionStat::Defence,
                    );
                } else {
                    potion.calc_boost(self.stats.defence);
                }
            }
        }
        if let Some(potions) = &mut self.potions.ranged {
            for potion in potions {
                potion.calc_boost(self.stats.ranged);
            }
        }
        if let Some(potions) = &mut self.potions.magic {
            for potion in potions {
                potion.calc_boost(self.stats.magic);
            }
        }
    }

    fn apply_potion_boosts(&mut self) {
        // Apply all of the selected potion boosts to the player's live stats
        if let Some(potions) = &self.potions.attack {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.attack.boost(max_boost);
        }
        if let Some(potions) = &self.potions.strength {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.strength.boost(max_boost);
        }
        if let Some(potions) = &self.potions.defence {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.defence.boost(max_boost);
        }
        if let Some(potions) = &self.potions.ranged {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.ranged.boost(max_boost);
        }
        if let Some(potions) = &self.potions.magic {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.magic.boost(max_boost);
        }
    }

    pub fn combat_stance(&self) -> CombatStance {
        // Get combat stance (accurate, aggressive, etc.)
        self.gear.weapon.combat_styles[&self.attrs.active_style].stance
    }

    pub fn combat_type(&self) -> CombatType {
        // Get combat type (stab, slash, ranged, etc.)
        self.gear.weapon.combat_styles[&self.attrs.active_style].combat_type
    }

    pub fn is_using_melee(&self) -> bool {
        // Check if the player is using any melee style
        let melee_types = [CombatType::Stab, CombatType::Slash, CombatType::Crush];
        melee_types.contains(&self.combat_type())
    }

    pub fn is_using_ranged(&self) -> bool {
        // Check if the player is using any ranged style
        let ranged_types = [CombatType::Light, CombatType::Standard, CombatType::Heavy];
        ranged_types.contains(&self.combat_type())
    }

    pub fn set_active_style(&mut self, style: CombatStyle) {
        // Set the active combat style and make any necessary attack speed adjustments
        self.attrs.active_style = style;

        let stance = self.combat_stance();

        // Reduce attack speed by 1 on rapid
        if stance == CombatStance::Rapid {
            self.gear.weapon.speed = self.gear.weapon.base_speed - 1;
        } else if [
            CombatStance::DefensiveAutocast,
            CombatStance::Autocast,
            CombatStance::ManualCast,
        ]
        .contains(&stance)
        {
            // Prevent staff speed from being set to its melee attack speed if player is casting spells
            self.gear.weapon.speed = if self.is_wearing("Harmonised nightmare staff", None)
                && self.is_using_standard_spell()
            {
                4
            } else if self.is_wearing("Twinflame staff", None) {
                6
            } else {
                5
            }
        } else {
            self.gear.weapon.speed = self.gear.weapon.base_speed;
        }
    }

    pub fn switch(&mut self, switch_type: &SwitchType) {
        for switch in &self.switches {
            if &switch.switch_type == switch_type {
                self.gear = switch.gear.clone();
                self.prayers = switch.prayers.clone();
                self.attrs.spell = switch.spell;
                self.attrs.active_style = switch.active_style;
                self.set_effects = switch.set_effects;
                self.attack = switch.attack;
                self.spec = switch.spec;
                self.att_rolls = switch.att_rolls;
                self.max_hits = switch.max_hits;
                self.def_rolls = switch.def_rolls;
                self.current_switch = Some(switch.switch_type.clone());

                return;
            }
        }
        panic!("Gear switch not found.")
    }

    pub fn is_wearing_black_mask(&self) -> bool {
        // Check if the player is wearing any type of black mask or slayer helmet
        self.is_wearing_any(BLACK_MASKS)
    }

    pub fn is_wearing_imbued_black_mask(&self) -> bool {
        // Check if the player is wearing an imbued black mask or slayer helmet
        self.is_wearing_any(BLACK_MASKS_IMBUED)
    }

    pub fn is_wearing_salve(&self) -> bool {
        // Check if the player is wearing an unenchanted salve amulet
        self.is_wearing_any(SALVE_UNENCHANTED)
    }

    pub fn is_wearing_salve_e(&self) -> bool {
        // Check if the player is wearing an enchanted salve amulet
        self.is_wearing_any(SALVE_ENCHANTED)
    }

    pub fn is_wearing_salve_i(&self) -> bool {
        // Check if the player is wearing an imbued salve amulet
        self.is_wearing_any(SALVE_IMBUED)
    }

    pub fn is_wearing_wildy_mace(&self) -> bool {
        // Check if the player is wearing either type of wilderness mace
        self.is_wearing_any(WILDY_MACES)
    }

    pub fn is_wearing_wildy_bow(&self) -> bool {
        // Check if the player is wearing either type of wilderness bow
        self.is_wearing_any(WILDY_BOWS)
    }

    pub fn is_wearing_wildy_staff(&self) -> bool {
        // Check if the player is wearing any form of wilderness staff
        self.is_wearing_any(WILDY_STAVES)
    }

    pub fn is_wearing_elf_bow(&self) -> bool {
        // Check if the player is wearing a crystal bow or bowfa
        self.is_wearing_any(ELF_BOWS)
    }

    pub fn is_wearing_tzhaar_weapon(&self) -> bool {
        // Check if the player is wearing an obsidan melee weapon
        self.gear.weapon.name.contains("Tzhaar") || self.gear.weapon.name.contains("Toktz")
    }

    pub fn is_wearing_salamander(&self) -> bool {
        // Check if the player is wearing a salamander or swamp lizard
        self.gear.weapon.name.contains("salamander") || self.is_wearing("Swamp lizard", None)
    }

    pub fn is_wearing_smoke_staff(&self) -> bool {
        // Check if the player is wearing either type of smoke staff
        self.is_wearing_any(SMOKE_STAVES)
    }

    pub fn is_wearing_silver_weapon(&self) -> bool {
        // Check if the player is wearing any type of silver weapon
        self.is_wearing_any(SILVER_WEAPONS)
            || (self.combat_type() == CombatType::Ranged && self.is_wearing("Silver bolts", None))
    }

    pub fn is_wearing_ivandis_weapon(&self) -> bool {
        // Check if the player is wearing one of the weapons that can harm T3 vampyres
        self.is_wearing_any(IVANDIS_WEAPONS)
    }

    pub fn is_wearing_keris(&self) -> bool {
        // Check if the player is wearing any type of keris
        self.is_wearing_any(KERIS_WEAPONS)
    }

    pub fn is_wearing_leaf_bladed_weapon(&self) -> bool {
        // Check if the player is wearing any type of leaf-bladed weapon or broad bolts
        (self.is_using_melee() && self.is_wearing_any(LEAF_BLADED_WEAPONS))
            || (self.combat_type() == CombatType::Ranged
                && (self.is_using_crossbow() && self.is_wearing_any(BROAD_BOLTS)))
            || self.is_wearing("Broad arrows", None)
    }

    pub fn is_wearing_full_void(&self) -> bool {
        // Check if the player is wearing a full void set
        FULL_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_full_elite_void(&self) -> bool {
        // Check if the player is wearing a full elite void set
        FULL_ELITE_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_ancient_spectre(&self) -> bool {
        // Check if the player is wearing any type of ancient spectre
        self.is_wearing_any(ANCIENT_SPECTRES)
    }

    pub fn is_wearing_ratbone_weapon(&self) -> bool {
        // Check if the player is wearing any type of ratbone weapon
        self.is_wearing_any(RATBANE_WEAPONS)
    }

    pub fn is_using_spell(&self) -> bool {
        // Check if the player is casting a spell
        self.attrs.spell.is_some()
            && [
                CombatStance::Autocast,
                CombatStance::ManualCast,
                CombatStance::DefensiveAutocast,
            ]
            .contains(&self.combat_stance())
    }

    pub fn is_using_standard_spell(&self) -> bool {
        // Check if the player is casting a spell on the standard spellbook
        self.is_using_spell() && spells::is_standard_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_water_spell(&self) -> bool {
        // Water strike/bolt/blast/wave/surge
        self.is_using_spell() && spells::is_water_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_ancient_spell(&self) -> bool {
        // Check if the player is casting a spell on the ancient spellbook
        self.is_using_spell() && spells::is_ancient_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_smoke_spell(&self) -> bool {
        // Smoke rush/burst/blitz/barrage
        self.is_using_spell() && spells::is_smoke_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_shadow_spell(&self) -> bool {
        // Shadow rush/burst/blitz/barrage
        self.is_using_spell() && spells::is_shadow_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_blood_spell(&self) -> bool {
        // Blood rush/burst/blitz/barrage
        self.is_using_spell() && spells::is_blood_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_ice_spell(&self) -> bool {
        // Ice rush/burst/blitz/barrage
        self.is_using_spell() && spells::is_ice_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_fire_spell(&self) -> bool {
        // Fire strike/bolt/blast/wave/surge
        self.is_using_spell() && spells::is_fire_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_air_spell(&self) -> bool {
        // Air strike/bolt/blast/wave/surge
        self.is_using_spell() && spells::is_air_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_earth_spell(&self) -> bool {
        // Earth strike/bolt/blast/wave/surge
        self.is_using_spell() && spells::is_earth_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_demonbane_spell(&self) -> bool {
        // Inferior/Superior/Dark demonbane
        self.is_using_spell() && spells::is_demonbane_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_bind_spell(&self) -> bool {
        // All bind spells, including grasp spells
        self.is_using_spell() && spells::is_bind_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_grasp_spell(&self) -> bool {
        // Grasp spells on the Arceuus spellbook
        self.is_using_spell() && spells::is_grasp_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_crossbow(&self) -> bool {
        // Check if the player is using any type of crossbow and wielding bolts
        self.gear.weapon.name.contains("rossbow")
            && self.combat_type() == CombatType::Heavy
            && self
                .gear
                .ammo
                .as_ref()
                .unwrap_or(&Armor::default())
                .name
                .contains("bolt")
    }

    pub fn is_using_demonbane(&self) -> bool {
        self.is_using_demonbane_spell() || self.is_wearing_any(DEMONBANE_WEAPONS)
    }

    pub fn is_using_corpbane_weapon(&self) -> bool {
        // Check if the player's weapon does full damage to Corp
        let weapon_name = &self.gear.weapon.name;
        match self.combat_type() {
            CombatType::Magic => true,
            CombatType::Stab => {
                self.is_wearing("Osmumten's fang", None)
                    || weapon_name.contains("halberd")
                    || (weapon_name.contains("spear") && weapon_name.as_str() != "Blue moon spear")
            }
            _ => false,
        }
    }

    pub fn set_spell(&mut self, spell: spells::Spell) {
        if spell.required_level() > self.stats.magic.current {
            panic!("Player does not have enough magic to cast {spell}.");
        }
        self.attrs.spell = Some(spell);
    }

    pub fn add_potion(&mut self, potion: Potion) {
        // Add a potion to the correct slot, calc boosts, and reset live stats
        if potion.boosts_attack() {
            self.potions
                .attack
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_strength() {
            self.potions
                .strength
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_defence() {
            self.potions
                .defence
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_ranged() {
            self.potions
                .ranged
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_magic() {
            self.potions
                .magic
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_all_melee() {
            self.potions
                .attack
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .strength
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .defence
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_all() {
            self.potions
                .attack
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .strength
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .defence
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .ranged
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .magic
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        }

        self.calc_potion_boosts();
        self.reset_current_stats(false);
    }

    pub fn remove_potion(&mut self, potion: Potion) {
        self.potions.remove_potion(potion);
        self.calc_potion_boosts();
        self.reset_current_stats(false);
    }

    pub fn bulwark_bonus(&self) -> i32 {
        // Calculate additional melee strength bonus from bulwark passive
        max(
            0,
            (self.bonuses.defence.stab
                + self.bonuses.defence.slash
                + self.bonuses.defence.crush
                + self.bonuses.defence.ranged
                - 800)
                / 12
                - 38,
        )
    }

    pub fn is_quiver_bonus_valid(&self) -> bool {
        // Check if the player is wearing a quiver and using a weapon with bolts or arrows
        self.gear.cape.as_ref().is_some_and(|cape| {
            cape.name == "Dizana's quiver"
                && cape.matches_version("Charged")
                && self.gear.weapon.uses_bolts_or_arrows()
                && (self
                    .gear
                    .ammo
                    .as_ref()
                    .is_some_and(|ammo| ammo.is_bolt_or_arrow())
                    || self
                        .gear
                        .second_ammo
                        .as_ref()
                        .is_some_and(|ammo| ammo.is_bolt_or_arrow()))
        })
    }

    pub fn heal(&mut self, amount: u32, overheal_hp: Option<u32>) {
        // Heals the player by the specified amount (with optional maximum overheal)
        self.stats.hitpoints.restore(amount, overheal_hp);
    }

    pub fn regen_all_stats(&mut self) {
        if self.stats.hitpoints.current < self.stats.hitpoints.base {
            self.stats.hitpoints.restore(1, None);
        }

        if self.stats.attack.current < self.stats.attack.base {
            self.stats.attack.restore(1, None);
        }

        if self.stats.strength.current < self.stats.strength.base {
            self.stats.strength.restore(1, None);
        }

        if self.stats.defence.current < self.stats.defence.base {
            self.stats.defence.restore(1, None);
        }

        if self.stats.ranged.current < self.stats.ranged.base {
            self.stats.ranged.restore(1, None);
        }

        if self.stats.magic.current < self.stats.magic.base {
            self.stats.magic.restore(1, None);
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        // Takes damage, capping at 0 HP
        self.stats.hitpoints.drain(amount);
    }

    pub fn clear_inactive_effects(&mut self) {
        self.active_effects.retain(|event| match event {
            CombatEffect::Poison { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::Venom { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::Burn { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::DelayedAttack { tick_delay, .. } => tick_delay.is_some(),
            CombatEffect::DelayedHeal { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::DamageOverTime { tick_counter, .. } => tick_counter.is_some(),
        })
    }

    pub fn restore_prayer(&mut self, amount: u32, max_level: Option<u32>) {
        let cap = max_level.unwrap_or(self.stats.prayer.base);
        self.stats.prayer.restore(amount, Some(cap));
    }

    pub fn seercull_spec_max(&self) -> u32 {
        // Calculate the max hit for Seercull, MSB, etc.
        let str_bonus = self
            .gear
            .ammo
            .as_ref()
            .map_or(0, |ammo| ammo.bonuses.strength.ranged);

        (320 + (self.stats.ranged.current + 10) * (str_bonus + 64) as u32) / 640
    }

    pub fn bolt_proc_chance(&self, base_chance: f64) -> f64 {
        if self.boosts.zcb_spec {
            return 1.0;
        }

        let mut proc_chance = base_chance;

        if self.boosts.kandarin_diary {
            proc_chance += 0.1 * base_chance;
        }

        if self.boosts.acb_spec {
            proc_chance += base_chance;
        }

        proc_chance
    }

    pub fn is_wearing_ogre_bow(&self) -> bool {
        self.is_wearing_any(OGRE_BOWS)
    }

    pub fn gets_second_twinflame_hit(&self) -> bool {
        self.is_wearing("Twinflame staff", None) && {
            if let Some(spell) = self.attrs.spell {
                spells::is_blast_spell(&spell)
                    || spells::is_bolt_spell(&spell)
                    || spells::is_wave_spell(&spell)
            } else {
                false
            }
        }
    }
}

pub async fn fetch_player_data(rsn: &str) -> Result<String, reqwest::Error> {
    // Fetches player data from the OSRS hiscores
    let url = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";
    let params = [("player", rsn)];
    let client = reqwest::Client::new();
    let response = client.get(url).query(&params).send().await?; // Use .await
    let data = response.text().await?; // Use .await
    Ok(data)
}

pub fn parse_player_data(data: String) -> PlayerStats {
    // Parses player data and creates a PlayerStats struct from it
    let skills = [
        "attack",
        "defence",
        "strength",
        "hitpoints",
        "ranged",
        "prayer",
        "magic",
    ];
    let data_lines: Vec<&str> = data.lines().collect();
    let mut skill_map = HashMap::new();

    for (i, skill) in skills.iter().enumerate() {
        let line_parts: Vec<&str> = data_lines[i + 1].split(',').collect();
        let level = line_parts[1]
            .parse::<u32>()
            .expect("Level could not be parsed as u8.");
        skill_map.insert(*skill, level);
    }

    let mining_lvl = data_lines[15].split(',').collect::<Vec<&str>>()[1];
    skill_map.insert("mining", mining_lvl.parse::<u32>().unwrap());
    let herblore_lvl = data_lines[16].split(',').collect::<Vec<&str>>()[1];
    skill_map.insert("herblore", herblore_lvl.parse::<u32>().unwrap());

    PlayerStats {
        hitpoints: Stat::new(skill_map["hitpoints"], None),
        attack: Stat::new(skill_map["attack"], None),
        strength: Stat::new(skill_map["strength"], None),
        defence: Stat::new(skill_map["defence"], None),
        ranged: Stat::new(skill_map["ranged"], None),
        magic: Stat::new(skill_map["magic"], None),
        prayer: Stat::new(skill_map["prayer"], None),
        mining: Stat::new(skill_map["mining"], None),
        herblore: Stat::new(skill_map["herblore"], None),
        spec: SpecEnergy::default(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::equipment::{CombatStyle, StrengthBonus, StyleBonus};
    use crate::types::potions::Potion;
    use crate::types::prayers::Prayer;
    use crate::types::spells::{Spell, StandardSpell};

    #[test]
    fn test_default_player() {
        let player = Player::new();

        assert_eq!(player.stats, PlayerStats::default());
        assert_eq!(player.gear, Gear::default());
        assert_eq!(player.bonuses, EquipmentBonuses::default());
        assert_eq!(player.potions, PotionBoosts::default());
        assert_eq!(player.prayers, PrayerBoosts::default());
        assert_eq!(player.boosts, StatusBoosts::default());
        assert_eq!(player.active_effects, Vec::new());
        assert_eq!(player.set_effects, SetEffects::default());
        assert_eq!(player.att_rolls, PlayerAttRolls::default());
        assert_eq!(player.max_hits, PlayerMaxHits::default());
        assert_eq!(player.def_rolls, PlayerDefRolls::default());
        assert!(player.attrs.name.is_none());
        assert_eq!(player.attrs.active_style, CombatStyle::Punch);
        assert!(player.attrs.spell.is_none());
    }

    #[tokio::test]
    async fn test_lookup_stats() {
        let mut player = Player::new();
        player.lookup_stats("Lynx Titan").await;
        assert_eq!(player.stats.attack.base, 99);
        assert_eq!(player.stats.defence.base, 99);
        assert_eq!(player.stats.strength.base, 99);
        assert_eq!(player.stats.hitpoints.base, 99);
        assert_eq!(player.stats.ranged.base, 99);
        assert_eq!(player.stats.magic.base, 99);
        assert_eq!(player.stats.prayer.base, 99);
    }

    #[test]
    fn test_is_wearing() {
        let mut player = Player::new();
        player.gear.head = Some(Armor::default());
        player.gear.head.as_mut().unwrap().name = "Torva full helm".to_string();
        assert!(player.is_wearing("Torva full helm", None));
    }

    #[test]
    fn test_equip_armor() {
        let mut player = Player::new();
        player.equip("Torva full helm", None);
        player.update_bonuses();
        let torva_full_helm = Armor::new("Torva full helm", None);
        assert_eq!(player.gear.head.unwrap(), torva_full_helm);
        assert_eq!(player.bonuses, torva_full_helm.bonuses)
    }

    #[test]
    fn test_equip_weapon() {
        let mut player = Player::new();
        player.equip("Osmumten's fang", None);
        player.update_bonuses();
        let osmumtens_fang = Weapon::new("Osmumten's fang", None);
        assert_eq!(player.gear.weapon, osmumtens_fang);
        assert_eq!(player.bonuses, osmumtens_fang.bonuses)
    }

    #[test]
    fn test_replace_gear() {
        let mut player = Player::new();
        player.equip("Torva full helm", None);
        player.update_bonuses();
        player.equip("Neitiznot faceguard", None);
        player.update_bonuses();
        let neitiznot_faceguard = Armor::new("Neitiznot faceguard", None);
        assert_eq!(player.gear.head.unwrap(), neitiznot_faceguard);
        assert_eq!(player.bonuses, neitiznot_faceguard.bonuses)
    }

    #[test]
    fn test_max_melee_bonuses() {
        let mut player = Player::new();
        let max_melee_gear = Gear {
            head: Some(Armor::new("Torva full helm", None)),
            neck: Some(Armor::new("Amulet of torture", None)),
            cape: Some(Armor::new("Infernal cape", None)),
            ammo: Some(Armor::new("Rada's blessing 4", None)),
            second_ammo: None,
            weapon: Weapon::new("Osmumten's fang", None),
            shield: Some(Armor::new("Avernic defender", None)),
            body: Some(Armor::new("Torva platebody", None)),
            legs: Some(Armor::new("Torva platelegs", None)),
            hands: Some(Armor::new("Ferocious gloves", None)),
            feet: Some(Armor::new("Primordial boots", None)),
            ring: Some(Armor::new("Ultor ring", None)),
        };
        player.gear = max_melee_gear;
        player.update_bonuses();

        let max_melee_bonuses = EquipmentBonuses {
            attack: StyleBonus {
                stab: 172,
                slash: 141,
                crush: 65,
                ranged: -50,
                magic: -71,
            },
            defence: StyleBonus {
                stab: 327,
                slash: 312,
                crush: 320,
                ranged: 309,
                magic: -15,
            },
            strength: StrengthBonus {
                melee: 178,
                ranged: 0,
                magic: 0.0,
            },
            prayer: 9,
        };

        assert_eq!(player.bonuses, max_melee_bonuses);
    }

    #[test]
    fn test_potion_boosts() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_potion(Potion::SuperAttack);
        player.add_potion(Potion::SuperStrength);
        player.add_potion(Potion::SuperDefence);
        player.add_potion(Potion::Ranging);
        player.add_potion(Potion::SaturatedHeart);

        player.calc_potion_boosts();
        player.reset_current_stats(false);

        assert_eq!(player.stats.attack.current, 118);
        assert_eq!(player.stats.strength.current, 118);
        assert_eq!(player.stats.defence.current, 118);
        assert_eq!(player.stats.ranged.current, 112);
        assert_eq!(player.stats.magic.current, 112);
    }

    #[test]
    fn test_dragon_battleaxe_boost() {
        let mut player = Player::new();
        player.add_potion(Potion::ZamorakBrew);
        player.add_potion(Potion::SuperDefence);
        player.add_potion(Potion::Magic);
        player.add_potion(Potion::Ranging);
        player.add_potion(Potion::DragonBattleaxe);
        player.calc_potion_boosts();
        player.reset_current_stats(false);
        player.reset_current_stats(false);

        assert_eq!(player.stats.attack.current, 120);
        assert_eq!(player.stats.strength.current, 120);
        assert_eq!(player.stats.defence.current, 118);
        assert_eq!(player.stats.ranged.current, 112);
        assert_eq!(player.stats.magic.current, 103);
    }

    #[test]
    fn test_prayer_boost() {
        let mut player = Player::new();
        player.prayers.add(Prayer::Chivalry);
        assert_eq!(player.prayers.attack, 15);
        assert_eq!(player.prayers.strength, 18);
        assert_eq!(player.prayers.defence, 20);
        player.prayers.add(Prayer::Piety);
        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.prayers.strength, 23);
        assert_eq!(player.prayers.defence, 25);
    }

    #[test]
    fn test_twinflame_detection() {
        let mut player = Player::new();
        player.equip("Twinflame staff", None);
        player.set_spell(Spell::Standard(StandardSpell::EarthBolt));
        assert!(player.gets_second_twinflame_hit());

        player.set_spell(Spell::Standard(StandardSpell::EarthSurge));
        assert!(!player.gets_second_twinflame_hit());
    }
}
