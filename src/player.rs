use crate::attacks::{standard_attack, AttackFn};
use crate::constants::*;
use crate::equipment::{
    self, Armor, CombatStance, CombatStyle, CombatType, EquipmentBonuses, Weapon,
};
use crate::potions::{Potion, PotionBoost};
use crate::prayers::PrayerBoost;
use crate::spells;
use reqwest::Error;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;

// Base stats of the player - should not be modified
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PlayerStats {
    pub hitpoints: u32,
    pub attack: u32,
    pub strength: u32,
    pub defence: u32,
    pub ranged: u32,
    pub magic: u32,
    pub prayer: u32,
    pub mining: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            // Assume max stats as default case
            hitpoints: 99,
            attack: 99,
            strength: 99,
            defence: 99,
            ranged: 99,
            magic: 99,
            prayer: 99,
            mining: 99,
        }
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        Self::default()
    }
}

// Live stats of the player during combat, including boosts - can be modified
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PlayerLiveStats {
    pub hitpoints: u32,
    pub attack: u32,
    pub strength: u32,
    pub defence: u32,
    pub ranged: u32,
    pub magic: u32,
    pub prayer: u32,
    pub special_attack: u8,
}

impl Default for PlayerLiveStats {
    fn default() -> Self {
        Self {
            hitpoints: 99,
            attack: 99,
            strength: 99,
            defence: 99,
            ranged: 99,
            magic: 99,
            prayer: 99,
            special_attack: 100,
        }
    }
}

impl PlayerLiveStats {
    pub fn new() -> Self {
        Self::default()
    }
}

// Collection of active potion boosts, separated by combat type
#[derive(Debug, Default, PartialEq)]
pub struct PotionBoosts {
    pub attack: Option<PotionBoost>,
    pub strength: Option<PotionBoost>,
    pub defence: Option<PotionBoost>,
    pub ranged: Option<PotionBoost>,
    pub magic: Option<PotionBoost>,
}

// Collection of active prayers and their cumulative boosts
#[derive(Debug, Default, PartialEq)]
pub struct PrayerBoosts {
    pub active_prayers: Option<Vec<PrayerBoost>>,
    pub attack: u32,
    pub strength: u32,
    pub defence: u32,
    pub ranged_att: u32,
    pub ranged_str: u32,
    pub magic_att: u32,
    pub magic_str: u32,
}

impl PrayerBoosts {
    pub fn add(&mut self, prayer: PrayerBoost) {
        match &mut self.active_prayers {
            Some(active_prayers) => {
                // Remove any conflicting prayer boosts first
                active_prayers.retain(|p| !conflicts_with(p, &prayer));
                active_prayers.push(prayer);
            }
            None => {
                self.active_prayers = Some(vec![prayer]);
            }
        }

        // Set boosts to the highest value provided by active prayers
        self.attack = update_prayer_boost(self.active_prayers.as_ref().unwrap(), |p| p.attack);
        self.strength = update_prayer_boost(self.active_prayers.as_ref().unwrap(), |p| p.strength);
        self.defence = update_prayer_boost(self.active_prayers.as_ref().unwrap(), |p| p.defence);
        self.ranged_att =
            update_prayer_boost(self.active_prayers.as_ref().unwrap(), |p| p.ranged_att);
        self.ranged_str =
            update_prayer_boost(self.active_prayers.as_ref().unwrap(), |p| p.ranged_str);
        self.magic_att =
            update_prayer_boost(self.active_prayers.as_ref().unwrap(), |p| p.magic_att);
        self.magic_str =
            update_prayer_boost(self.active_prayers.as_ref().unwrap(), |p| p.magic_str);
    }
}

fn conflicts_with(p1: &PrayerBoost, p2: &PrayerBoost) -> bool {
    // Check if two prayer boosts conflict on any stats
    p1.attack > 0 && p2.attack > 0
        || p1.strength > 0 && p2.strength > 0
        || p1.defence > 0 && p2.defence > 0
        || p1.ranged_att > 0 && p2.ranged_att > 0
        || p1.ranged_str > 0 && p2.ranged_str > 0
        || p1.magic_att > 0 && p2.magic_att > 0
        || p1.magic_str > 0 && p2.magic_str > 0
}

fn update_prayer_boost(prayers: &[PrayerBoost], stat: fn(&PrayerBoost) -> u32) -> u32 {
    // Search through active prayers and returns the highest boost value for a stat
    prayers.iter().map(stat).max().unwrap_or(0)
}

// Struct for holding sunfire rune min hit value
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct SunfireBoost {
    pub active: bool,
    pub min_hit: u32,
}

// Collection of effects that provide a damage boost in some cases
#[derive(Debug, PartialEq)]
pub struct StatusBoosts {
    pub on_task: bool,
    pub in_wilderness: bool,
    pub in_multi: bool,
    pub forinthry_surge: bool,
    pub charge_active: bool,
    pub kandarin_diary: bool,
    pub mark_of_darkness: bool,
    pub sunfire: SunfireBoost,
    pub soulreaper_stacks: u32,
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
            sunfire: SunfireBoost::default(),
            soulreaper_stacks: 0,
        }
    }
}

// Poison and venom effects - will likely rework this in the future
#[derive(Default, Debug, PartialEq)]
pub struct StatusEffects {
    pub poisoned: bool,
    pub venomed: bool,
    pub immune_poison: bool,
    pub immune_venom: bool,
    pub poison_severity: u8,
}

// Holds set effect data to avoid iterating through gear many times
#[derive(Default, Debug, PartialEq)]
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

#[derive(Default, PartialEq, Debug)]
pub struct Gear {
    pub head: Option<Armor>,
    pub neck: Option<Armor>,
    pub cape: Option<Armor>,
    pub ammo: Option<Armor>,
    pub second_ammo: Option<Armor>,
    pub weapon: Weapon, // Default to unarmed, which is still a weapon
    pub shield: Option<Armor>,
    pub body: Option<Armor>,
    pub legs: Option<Armor>,
    pub hands: Option<Armor>,
    pub feet: Option<Armor>,
    pub ring: Option<Armor>,
}

// Misc other player info - may restructure if there's a better place for these
#[derive(Debug, Default)]
pub struct PlayerAttrs {
    pub name: Option<String>,
    pub active_style: CombatStyle,
    pub spell: Option<spells::Spell>,
}

pub struct Player {
    pub stats: PlayerStats,
    pub live_stats: PlayerLiveStats,
    pub gear: Gear,
    pub bonuses: EquipmentBonuses,
    pub potions: PotionBoosts,
    pub prayers: PrayerBoosts,
    pub boosts: StatusBoosts,
    pub effects: StatusEffects,
    pub set_effects: SetEffects,
    pub attrs: PlayerAttrs,
    pub att_rolls: HashMap<CombatType, i32>,
    pub max_hits: HashMap<CombatType, u32>,
    pub def_rolls: HashMap<CombatType, i32>,
    pub attack: AttackFn,
}

impl Default for Player {
    fn default() -> Self {
        let att_rolls = vec![
            (CombatType::Stab, 0),
            (CombatType::Slash, 0),
            (CombatType::Crush, 0),
            (CombatType::Light, 0),
            (CombatType::Standard, 0),
            (CombatType::Heavy, 0),
            (CombatType::Magic, 0),
        ]
        .into_iter()
        .collect();

        let max_hits = vec![
            (CombatType::Stab, 0),
            (CombatType::Slash, 0),
            (CombatType::Crush, 0),
            (CombatType::Light, 0),
            (CombatType::Standard, 0),
            (CombatType::Heavy, 0),
            (CombatType::Magic, 0),
        ]
        .into_iter()
        .collect();

        let def_rolls = vec![
            (CombatType::Stab, 0),
            (CombatType::Slash, 0),
            (CombatType::Crush, 0),
            (CombatType::Ranged, 0),
            (CombatType::Magic, 0),
        ]
        .into_iter()
        .collect();

        Self {
            stats: PlayerStats::default(),
            live_stats: PlayerLiveStats::default(),
            gear: Gear::default(),
            bonuses: EquipmentBonuses::default(),
            potions: PotionBoosts::default(),
            prayers: PrayerBoosts::default(),
            boosts: StatusBoosts::default(),
            effects: StatusEffects::default(),
            set_effects: SetEffects::default(),
            attrs: PlayerAttrs::default(),
            att_rolls,
            max_hits,
            def_rolls,
            attack: standard_attack,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup_stats(&mut self, rsn: &str) {
        // Fetch stats from OSRS hiscores and set the corresponding fields
        let stats = fetch_player_data(rsn).expect("Failed to fetch player data");
        self.stats = parse_player_data(stats);
        self.attrs.name = Some(rsn.to_string());
    }

    pub fn reset_live_stats(&mut self) {
        // Restore to base stats, full spec energy, and reapply potion boosts
        self.live_stats.attack = self.stats.attack;
        self.live_stats.strength = self.stats.strength;
        self.live_stats.defence = self.stats.defence;
        self.live_stats.ranged = self.stats.ranged;
        self.live_stats.magic = self.stats.magic;
        self.live_stats.prayer = self.stats.prayer;
        self.live_stats.hitpoints = self.stats.hitpoints;
        self.live_stats.special_attack = 100;

        self.apply_potion_boosts();
    }

    pub fn is_wearing(&self, gear_name: &str, version: Option<&str>) -> bool {
        // Check if the player is wearing the specified piece of gear
        let version = version.map(|v| v.to_string());

        self.gear.head.as_ref().map_or(false, |armor| {
            armor.name == gear_name && armor.version == version
        }) || self.gear.neck.as_ref().map_or(false, |armor| {
            armor.name == gear_name && armor.version == version
        }) || self.gear.cape.as_ref().map_or(false, |armor| {
            armor.name == gear_name && armor.version == version
        }) || self.gear.ammo.as_ref().map_or(false, |armor| {
            armor.name == gear_name && armor.version == version
        }) || self.gear.weapon.name == gear_name && self.gear.weapon.version == version
            || self.gear.shield.as_ref().map_or(false, |armor| {
                armor.name == gear_name && armor.version == version
            })
            || self.gear.body.as_ref().map_or(false, |armor| {
                armor.name == gear_name && armor.version == version
            })
            || self.gear.legs.as_ref().map_or(false, |armor| {
                armor.name == gear_name && armor.version == version
            })
            || self.gear.hands.as_ref().map_or(false, |armor| {
                armor.name == gear_name && armor.version == version
            })
            || self.gear.feet.as_ref().map_or(false, |armor| {
                armor.name == gear_name && armor.version == version
            })
            || self.gear.ring.as_ref().map_or(false, |armor| {
                armor.name == gear_name && armor.version == version
            })
    }

    pub fn is_wearing_any_version(&self, gear_name: &str) -> bool {
        // Same as is_wearing() but allows for any version to match
        self.gear
            .head
            .as_ref()
            .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .neck
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .cape
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .ammo
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self.gear.weapon.name == gear_name
            || self
                .gear
                .shield
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .body
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .legs
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .hands
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .feet
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
            || self
                .gear
                .ring
                .as_ref()
                .map_or(false, |armor| armor.name == gear_name)
    }

    pub fn is_wearing_any(&self, gear_names: Vec<(&str, Option<&str>)>) -> bool {
        // Check if the player is wearing any item in the provided Vec
        gear_names
            .iter()
            .any(|&gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn is_wearing_all(&self, gear_names: Vec<(&str, Option<&str>)>) -> bool {
        // Check if the player is wearing all items in the provided Vec
        gear_names
            .iter()
            .all(|&gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn equip(&mut self, item_name: &str, version: Option<&str>) {
        // Equip the specified item in the correct slot
        let slot_name = equipment::get_slot_name(item_name)
            .unwrap_or_else(|_| panic!("Slot not found for item {}", item_name));

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
                if self.is_wearing_any(vec![
                    ("Dizana's quiver", Some("Charged")),
                    ("Dizana's quiver", Some("Uncharged")),
                ]) && (self.gear.ammo.is_some()
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
            _ => panic!("Slot not found for item {}", item_name),
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
            && !self.gear.ammo.as_ref().unwrap().is_valid_ranged_ammo()
        {
            self.bonuses
                .add_bonuses(&self.gear.ammo.as_ref().unwrap().bonuses);
        }

        self.bonuses.add_bonuses(&self.gear.weapon.bonuses);
    }

    pub fn update_set_effects(&mut self) {
        // Update status of all set effects at once
        self.set_effects.full_ahrims = self.is_wearing_all(Vec::from(FULL_AHRIMS));
        self.set_effects.full_blood_moon = self.is_wearing_all(Vec::from(FULL_BLOOD_MOON));
        self.set_effects.full_blue_moon = self.is_wearing_all(Vec::from(FULL_BLUE_MOON));
        self.set_effects.full_dharoks = self.is_wearing_all(Vec::from(FULL_DHAROKS));
        self.set_effects.full_guthans = self.is_wearing_all(Vec::from(FULL_GUTHANS));
        self.set_effects.full_eclipse_moon = self.is_wearing_all(Vec::from(FULL_ECLIPSE_MOON));
        self.set_effects.full_inquisitor = self.is_wearing_all(Vec::from(FULL_INQUISITOR));
        self.set_effects.full_justiciar = self.is_wearing_all(Vec::from(FULL_JUSTICIAR));
        self.set_effects.full_karils = self.is_wearing_all(Vec::from(FULL_KARILS));
        self.set_effects.full_obsidian = self.is_wearing_all(Vec::from(FULL_OBSIDIAN));
        self.set_effects.full_torags = self.is_wearing_all(Vec::from(FULL_TORAGS));
        self.set_effects.full_void = self.is_wearing_full_void();
        self.set_effects.full_elite_void = self.is_wearing_full_elite_void();
        self.set_effects.bloodbark_pieces = BLOODBARK_ARMOR
            .iter()
            .filter(|armor| self.is_wearing(armor.0, armor.1))
            .count();
    }

    pub fn calc_potion_boosts(&mut self) {
        // Calculate all of the selected potion boosts
        if let Some(potion) = &mut self.potions.attack {
            potion.calc_boost(self.stats.attack)
        }
        if let Some(potion) = &mut self.potions.strength {
            potion.calc_boost(self.stats.strength)
        }
        if let Some(potion) = &mut self.potions.defence {
            potion.calc_boost(self.stats.defence);
        }
        if let Some(potion) = &mut self.potions.ranged {
            potion.calc_boost(self.stats.ranged);
        }
        if let Some(potion) = &mut self.potions.magic {
            potion.calc_boost(self.stats.magic);
        }
    }

    fn apply_potion_boosts(&mut self) {
        // Apply all of the selected potion boosts to the player's live stats
        if let Some(potion) = &self.potions.attack {
            self.live_stats.attack += potion.boost;
        }
        if let Some(potion) = &self.potions.strength {
            self.live_stats.strength += potion.boost;
        }
        if let Some(potion) = &self.potions.defence {
            self.live_stats.defence += potion.boost;
        }
        if let Some(potion) = &self.potions.ranged {
            self.live_stats.ranged += potion.boost;
        }
        if let Some(potion) = &self.potions.magic {
            self.live_stats.magic += potion.boost;
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
            } else {
                5
            }
        } else {
            self.gear.weapon.speed = self.gear.weapon.base_speed;
        }
    }

    pub fn is_wearing_black_mask(&self) -> bool {
        self.is_wearing_any(vec![
            ("Black mask", None),
            ("Black mask (i)", None),
            ("Slayer helmet", None),
            ("Slayer helmet (i)", None),
        ])
    }

    pub fn is_wearing_imbued_black_mask(&self) -> bool {
        self.is_wearing_any(vec![("Black mask (i)", None), ("Slayer helmet (i)", None)])
    }

    pub fn is_wearing_salve(&self) -> bool {
        self.is_wearing_any(vec![("Salve amulet", None), ("Salve amulet(i)", None)])
    }

    pub fn is_wearing_salve_e(&self) -> bool {
        self.is_wearing_any(vec![("Salve amulet (e)", None), ("Salve amulet(ei)", None)])
    }

    pub fn is_wearing_salve_i(&self) -> bool {
        self.is_wearing_any(vec![("Salve amulet(i)", None), ("Salve amulet(ei)", None)])
    }

    pub fn is_wearing_wildy_mace(&self) -> bool {
        self.is_wearing_any(vec![
            ("Viggora's chainmace", Some("Charged")),
            ("Ursine chainmace", Some("Charged")),
        ])
    }

    pub fn is_wearing_wildy_bow(&self) -> bool {
        self.is_wearing_any(vec![
            ("Craw's bow", Some("Charged")),
            ("Webweaver bow", Some("Charged")),
        ])
    }

    pub fn is_wearing_wildy_staff(&self) -> bool {
        self.is_wearing_any(vec![
            ("Thammaron's sceptre", Some("Charged")),
            ("Accursed sceptre", Some("Charged")),
            ("Thammaron's sceptre (a)", Some("Charged")),
            ("Accursed sceptre (a)", Some("Charged")),
        ])
    }

    pub fn is_wearing_crystal_bow(&self) -> bool {
        self.is_wearing_any(vec![
            ("Crystal bow", Some("Active")),
            ("Bow of faerdhinen", Some("Charged")),
            ("Bow of faerdhinen (c)", None),
        ])
    }

    pub fn is_wearing_tzhaar_weapon(&self) -> bool {
        self.gear.weapon.name.contains("Tzhaar") || self.gear.weapon.name.contains("Toktz")
    }

    pub fn is_wearing_salamander(&self) -> bool {
        self.gear.weapon.name.contains("salamander") || self.is_wearing("Swamp lizard", None)
    }

    pub fn is_wearing_smoke_staff(&self) -> bool {
        self.is_wearing_any(vec![
            ("Smoke battlestaff", None),
            ("Mystic smoke staff", None),
        ])
    }

    pub fn is_wearing_silver_weapon(&self) -> bool {
        self.is_wearing_any(vec![
            ("Blessed axe", None),
            ("Silver sickle", None),
            ("Silver sickle (b)", None),
            ("Emerald sickle", None),
            ("Emerald sickle (b)", None),
            ("Enchanted emerald sickle (b)", None),
            ("Ruby sickle (b)", None),
            ("Enchanted ruby sickle (b)", None),
            ("Silverlight", None),
            ("Silverlight", Some("Dyed")),
            ("Darklight", None),
            ("Arclight", None),
            ("Rod of ivandis", None),
            ("Wolfbane", None),
            ("Blisterwood flail", None),
            ("Blisterwood sickle", None),
            ("Ivandis flail", None),
        ]) || (self.combat_type() == CombatType::Ranged && self.is_wearing("Silver bolts", None))
    }

    pub fn is_wearing_ivandis_weapon(&self) -> bool {
        self.is_wearing_any(vec![
            ("Blisterwood flail", None),
            ("Blisterwood sickle", None),
            ("Ivandis flail", None),
        ])
    }

    pub fn is_wearing_keris(&self) -> bool {
        self.is_wearing_any(vec![
            ("Keris", None),
            ("Keris partisan", None),
            ("Keris partisan of the sun", None),
            ("Keris partisan of corruption", None),
            ("Keris partisan of breaching", None),
        ])
    }

    pub fn is_wearing_leaf_bladed_weapon(&self) -> bool {
        (self.is_using_melee()
            && self.is_wearing_any(vec![
                ("Leaf-bladed spear", None),
                ("Leaf-bladed sword", None),
                ("Leaf-bladed battleaxe", None),
            ]))
            || (self.combat_type() == CombatType::Ranged
                && (self.is_using_crossbow()
                    && self.is_wearing_any(vec![
                        ("Broad bolts", None),
                        ("Amethyst broad bolts", None),
                    ])))
            || self.is_wearing("Broad arrows", None)
    }

    pub fn is_wearing_full_void(&self) -> bool {
        FULL_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_full_elite_void(&self) -> bool {
        FULL_ELITE_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_ancient_spectre(&self) -> bool {
        self.is_wearing_any(vec![
            ("Ancient sceptre", None),
            ("Smoke ancient sceptre", None),
            ("Shadow ancient sceptre", None),
            ("Blood ancient sceptre", None),
            ("Ice ancient sceptre", None),
        ])
    }

    pub fn is_wearing_ratbone_weapon(&self) -> bool {
        self.is_wearing_any(vec![
            ("Bone mace", None),
            ("Bone shortbow", None),
            ("Bone staff", None),
        ])
    }

    pub fn is_using_spell(&self) -> bool {
        self.attrs.spell.is_some()
            && [
                CombatStance::Autocast,
                CombatStance::ManualCast,
                CombatStance::DefensiveAutocast,
            ]
            .contains(&self.combat_stance())
    }

    pub fn is_using_standard_spell(&self) -> bool {
        self.is_using_spell() && spells::is_standard_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_water_spell(&self) -> bool {
        self.is_using_spell() && spells::is_water_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_ancient_spell(&self) -> bool {
        self.is_using_spell() && spells::is_ancient_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_smoke_spell(&self) -> bool {
        self.is_using_spell() && spells::is_smoke_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_shadow_spell(&self) -> bool {
        self.is_using_spell() && spells::is_shadow_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_blood_spell(&self) -> bool {
        self.is_using_spell() && spells::is_blood_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_ice_spell(&self) -> bool {
        self.is_using_spell() && spells::is_ice_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_fire_spell(&self) -> bool {
        self.is_using_spell() && spells::is_fire_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_air_spell(&self) -> bool {
        self.is_using_spell() && spells::is_air_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_earth_spell(&self) -> bool {
        self.is_using_spell() && spells::is_earth_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_demonbane_spell(&self) -> bool {
        self.is_using_spell() && spells::is_demonbane_spell(self.attrs.spell.as_ref().unwrap())
    }

    pub fn is_using_crossbow(&self) -> bool {
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

    pub fn is_using_corpbane_weapon(&self) -> bool {
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
        self.attrs.spell = Some(spell);
    }

    pub fn add_potion(&mut self, potion: Potion) {
        match potion {
            Potion::Attack | Potion::SuperAttack | Potion::ZamorakBrewAtt => {
                self.potions.attack = Some(PotionBoost::new(&potion));
            }
            Potion::Strength
            | Potion::SuperStrength
            | Potion::ZamorakBrewStr
            | Potion::DragonBattleaxe => {
                self.potions.strength = Some(PotionBoost::new(&potion));
            }
            Potion::Defence | Potion::SuperDefence | Potion::SaradominBrew => {
                self.potions.defence = Some(PotionBoost::new(&potion));
            }
            Potion::Ranging | Potion::SuperRanging => {
                self.potions.ranged = Some(PotionBoost::new(&potion));
            }
            Potion::Magic
            | Potion::SuperMagic
            | Potion::ImbuedHeart
            | Potion::SaturatedHeart
            | Potion::AncientBrew
            | Potion::ForgottenBrew => {
                self.potions.magic = Some(PotionBoost::new(&potion));
            }
            Potion::SuperCombat => {
                self.potions.attack = Some(PotionBoost::new(&Potion::SuperAttack));
                self.potions.strength = Some(PotionBoost::new(&Potion::SuperStrength));
                self.potions.defence = Some(PotionBoost::new(&Potion::SuperDefence));
            }
            Potion::SmellingSalts
            | Potion::OverloadMinus
            | Potion::Overload
            | Potion::OverloadPlus => {
                self.potions.attack = Some(PotionBoost::new(&potion));
                self.potions.strength = Some(PotionBoost::new(&potion));
                self.potions.defence = Some(PotionBoost::new(&potion));
                self.potions.ranged = Some(PotionBoost::new(&potion));
                self.potions.magic = Some(PotionBoost::new(&potion));
            }
            _ => panic!("Unknown potion type"),
        }
        self.calc_potion_boosts();
        self.reset_live_stats();
    }

    pub fn bulwark_bonus(&self) -> i32 {
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
        self.gear.cape.as_ref().map_or(false, |cape| {
            cape.name == "Dizana's quiver"
                && cape.matches_version("Charged")
                && self.gear.weapon.uses_bolts_or_arrows()
                && (self
                    .gear
                    .ammo
                    .as_ref()
                    .map_or(false, |ammo| ammo.is_bolt_or_arrow())
                    || self
                        .gear
                        .second_ammo
                        .as_ref()
                        .map_or(false, |ammo| ammo.is_bolt_or_arrow()))
        })
    }

    pub fn heal(&mut self, amount: u32, overheal_hp: Option<u32>) {
        let max_hp = match overheal_hp {
            Some(overheal_hp) => self.stats.hitpoints + overheal_hp,
            None => self.stats.hitpoints,
        };
        self.live_stats.hitpoints = min(max_hp, self.live_stats.hitpoints + amount);
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.live_stats.hitpoints = self.live_stats.hitpoints.saturating_sub(amount);
    }
}

fn fetch_player_data(rsn: &str) -> Result<String, Error> {
    let url = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";
    let params = [("player", rsn)];
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).query(&params).send()?;
    let data = response.text()?;
    Ok(data)
}

fn parse_player_data(data: String) -> PlayerStats {
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

    PlayerStats {
        hitpoints: skill_map["hitpoints"],
        attack: skill_map["attack"],
        strength: skill_map["strength"],
        defence: skill_map["defence"],
        ranged: skill_map["ranged"],
        magic: skill_map["magic"],
        prayer: skill_map["prayer"],
        mining: skill_map["mining"],
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::equipment::{CombatStyle, CombatType, StrengthBonus, StyleBonus};
    use crate::potions::Potion;
    use crate::prayers::Prayer;
    use std::collections::HashMap;

    #[test]
    fn test_default_player() {
        let player = Player::new();

        let mut att_rolls = HashMap::new();
        att_rolls.insert(CombatType::Stab, 0);
        att_rolls.insert(CombatType::Slash, 0);
        att_rolls.insert(CombatType::Crush, 0);
        att_rolls.insert(CombatType::Light, 0);
        att_rolls.insert(CombatType::Standard, 0);
        att_rolls.insert(CombatType::Heavy, 0);
        att_rolls.insert(CombatType::Magic, 0);

        let mut max_hits = HashMap::new();
        max_hits.insert(CombatType::Stab, 0);
        max_hits.insert(CombatType::Slash, 0);
        max_hits.insert(CombatType::Crush, 0);
        max_hits.insert(CombatType::Light, 0);
        max_hits.insert(CombatType::Standard, 0);
        max_hits.insert(CombatType::Heavy, 0);
        max_hits.insert(CombatType::Magic, 0);

        let mut def_rolls = HashMap::new();
        def_rolls.insert(CombatType::Stab, 0);
        def_rolls.insert(CombatType::Slash, 0);
        def_rolls.insert(CombatType::Crush, 0);
        def_rolls.insert(CombatType::Ranged, 0);
        def_rolls.insert(CombatType::Magic, 0);

        assert_eq!(player.live_stats, PlayerLiveStats::default());
        assert_eq!(player.stats, PlayerStats::default());
        assert_eq!(player.gear, Gear::default());
        assert_eq!(player.bonuses, EquipmentBonuses::default());
        assert_eq!(player.potions, PotionBoosts::default());
        assert_eq!(player.prayers, PrayerBoosts::default());
        assert_eq!(player.boosts, StatusBoosts::default());
        assert_eq!(player.effects, StatusEffects::default());
        assert_eq!(player.set_effects, SetEffects::default());
        assert_eq!(player.att_rolls, att_rolls);
        assert_eq!(player.max_hits, max_hits);
        assert_eq!(player.def_rolls, def_rolls);
        assert!(player.attrs.name.is_none());
        assert_eq!(player.attrs.active_style, CombatStyle::Punch);
        assert!(player.attrs.spell.is_none());
    }

    #[test]
    fn test_lookup_stats() {
        let mut player = Player::new();
        player.lookup_stats("Lynx Titan");
        assert_eq!(player.stats.attack, 99);
        assert_eq!(player.stats.defence, 99);
        assert_eq!(player.stats.strength, 99);
        assert_eq!(player.stats.hitpoints, 99);
        assert_eq!(player.stats.ranged, 99);
        assert_eq!(player.stats.magic, 99);
        assert_eq!(player.stats.prayer, 99);
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
        player.stats = PlayerStats {
            attack: 99,
            strength: 99,
            defence: 99,
            ranged: 99,
            magic: 99,
            hitpoints: 99,
            prayer: 99,
            mining: 99,
        };
        player.potions.attack = Some(PotionBoost::new(&Potion::SuperAttack));
        player.potions.strength = Some(PotionBoost::new(&Potion::SuperStrength));
        player.potions.defence = Some(PotionBoost::new(&Potion::SuperDefence));
        player.potions.ranged = Some(PotionBoost::new(&Potion::Ranging));
        player.potions.magic = Some(PotionBoost::new(&Potion::SaturatedHeart));

        player.calc_potion_boosts();
        player.reset_live_stats();

        assert_eq!(player.live_stats.attack, 118);
        assert_eq!(player.live_stats.strength, 118);
        assert_eq!(player.live_stats.defence, 118);
        assert_eq!(player.live_stats.ranged, 112);
        assert_eq!(player.live_stats.magic, 112);
    }

    #[test]
    fn test_dragon_battleaxe_boost() {
        let mut player = Player::new();
        player.potions.attack = Some(PotionBoost::new(&Potion::ZamorakBrewAtt));
        player.potions.defence = Some(PotionBoost::new(&Potion::SuperDefence));
        player.potions.magic = Some(PotionBoost::new(&Potion::Magic));
        player.potions.ranged = Some(PotionBoost::new(&Potion::Ranging));
        player.potions.strength = Some(PotionBoost::new(&Potion::DragonBattleaxe));
        player.calc_potion_boosts();
        player.reset_live_stats();
        player
            .potions
            .strength
            .as_mut()
            .unwrap()
            .calc_dragon_battleaxe_boost(
                player.live_stats.attack,
                player.live_stats.defence,
                player.live_stats.ranged,
                player.live_stats.magic,
            );
        player.reset_live_stats();

        assert_eq!(player.live_stats.attack, 120);
        assert_eq!(player.live_stats.strength, 120);
        assert_eq!(player.live_stats.defence, 118);
        assert_eq!(player.live_stats.ranged, 112);
        assert_eq!(player.live_stats.magic, 103);
    }

    #[test]
    fn test_prayer_boost() {
        let mut player = Player::new();
        player.prayers.add(PrayerBoost::new(Prayer::Chivalry));
        assert_eq!(player.prayers.attack, 15);
        assert_eq!(player.prayers.strength, 18);
        assert_eq!(player.prayers.defence, 20);
        player.prayers.add(PrayerBoost::new(Prayer::Piety));
        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.prayers.strength, 23);
        assert_eq!(player.prayers.defence, 25);
    }
}
