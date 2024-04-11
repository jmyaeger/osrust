use crate::equipment::{Armor, CombatStyle, CombatType, StyleBonus, Weapon};
use crate::potions::PotionBoost;
use crate::prayers::PrayerBoost;
use crate::spells::{Spell, StandardSpell};
use reqwest::Error;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PlayerStats {
    pub hitpoints: u8,
    pub attack: u8,
    pub strength: u8,
    pub defence: u8,
    pub ranged: u8,
    pub magic: u8,
    pub prayer: u8,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            hitpoints: 10,
            attack: 1,
            strength: 1,
            defence: 1,
            ranged: 1,
            magic: 1,
            prayer: 1,
        }
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PlayerLiveStats {
    pub hitpoints: u8,
    pub attack: u8,
    pub strength: u8,
    pub defence: u8,
    pub ranged: u8,
    pub magic: u8,
    pub prayer: u8,
    pub special_attack: u8,
}

impl Default for PlayerLiveStats {
    fn default() -> Self {
        Self {
            hitpoints: 10,
            attack: 1,
            strength: 1,
            defence: 1,
            ranged: 1,
            magic: 1,
            prayer: 1,
            special_attack: 100,
        }
    }
}

impl PlayerLiveStats {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct EquipmentBonuses {
    pub attack: StyleBonus,
    pub defence: StyleBonus,
    pub strength: StyleBonus,
    pub prayer: i32,
}

#[derive(Debug, Default, PartialEq)]
pub struct PotionBoosts {
    pub attack: PotionBoost,
    pub strength: PotionBoost,
    pub defence: PotionBoost,
    pub ranged: PotionBoost,
    pub magic: PotionBoost,
}

#[derive(Debug, Default, PartialEq)]
pub struct PrayerBoosts {
    pub active_prayers: Vec<PrayerBoost>,
    pub attack: u8,
    pub strength: u8,
    pub defence: u8,
    pub ranged_att: u8,
    pub ranged_str: u8,
    pub magic: u8,
}

impl PrayerBoosts {
    pub fn add(&mut self, prayer: PrayerBoost) {
        self.active_prayers.retain(|p| {
            !(p.attack > 0 && prayer.attack > 0
                || (p.strength > 0 && prayer.strength > 0)
                || (p.defence > 0 && prayer.defence > 0)
                || (p.ranged_att > 0 && prayer.ranged_att > 0)
                || (p.ranged_str > 0 && prayer.ranged_str > 0)
                || (p.magic > 0 && prayer.magic > 0))
        });

        self.active_prayers.push(prayer);

        self.attack = self
            .active_prayers
            .iter()
            .map(|p| p.attack)
            .max()
            .unwrap_or(0);
        self.strength = self
            .active_prayers
            .iter()
            .map(|p| p.strength)
            .max()
            .unwrap_or(0);
        self.defence = self
            .active_prayers
            .iter()
            .map(|p| p.defence)
            .max()
            .unwrap_or(0);
        self.ranged_att = self
            .active_prayers
            .iter()
            .map(|p| p.ranged_att)
            .max()
            .unwrap_or(0);
        self.ranged_str = self
            .active_prayers
            .iter()
            .map(|p| p.ranged_str)
            .max()
            .unwrap_or(0);
        self.magic = self
            .active_prayers
            .iter()
            .map(|p| p.magic)
            .max()
            .unwrap_or(0);
    }
}

#[derive(Debug, PartialEq)]
pub struct StatusBoosts {
    pub on_task: bool,
    pub in_wilderness: bool,
    pub in_multi: bool,
    pub forinthry_surge: bool,
    pub charge_active: bool,
    pub kandarin_diary: bool,
    pub mark_of_darkness: bool,
    pub sunfire_runes: bool,
}

impl Default for StatusBoosts {
    fn default() -> Self {
        Self {
            on_task: true,
            in_wilderness: false,
            in_multi: false,
            forinthry_surge: false,
            charge_active: false,
            kandarin_diary: true,
            mark_of_darkness: false,
            sunfire_runes: false,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct StatusEffects {
    pub poisoned: bool,
    pub venomed: bool,
    pub immune_poison: bool,
    pub immune_venom: bool,
    pub poison_severity: u8,
}

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
}

#[derive(Default, PartialEq, Debug)]
pub struct Gear {
    pub head: Armor,
    pub neck: Armor,
    pub cape: Armor,
    pub ammo: Armor,
    pub weapon: Weapon,
    pub shield: Armor,
    pub body: Armor,
    pub legs: Armor,
    pub hands: Armor,
    pub feet: Armor,
    pub ring: Armor,
}

#[derive(Debug)]
pub struct PlayerAttrs {
    pub name: String,
    pub active_style: CombatStyle,
    pub spell: Box<dyn Spell>,
}

impl Default for PlayerAttrs {
    fn default() -> Self {
        Self {
            name: String::new(),
            active_style: CombatStyle::default(),
            spell: Box::<StandardSpell>::default(),
        }
    }
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
    pub att_rolls: HashMap<CombatType, u32>,
    pub max_hits: HashMap<CombatType, u8>,
    pub def_rolls: HashMap<CombatType, i32>,
}

impl Default for Player {
    fn default() -> Self {
        let mut att_rolls = HashMap::new();
        att_rolls.insert(CombatType::Stab, 0);
        att_rolls.insert(CombatType::Slash, 0);
        att_rolls.insert(CombatType::Crush, 0);
        att_rolls.insert(CombatType::Ranged, 0);
        att_rolls.insert(CombatType::Magic, 0);

        let mut max_hits = HashMap::new();
        max_hits.insert(CombatType::Stab, 0);
        max_hits.insert(CombatType::Slash, 0);
        max_hits.insert(CombatType::Crush, 0);
        max_hits.insert(CombatType::Ranged, 0);
        max_hits.insert(CombatType::Magic, 0);

        let mut def_rolls = HashMap::new();
        def_rolls.insert(CombatType::Stab, 0);
        def_rolls.insert(CombatType::Slash, 0);
        def_rolls.insert(CombatType::Crush, 0);
        def_rolls.insert(CombatType::Ranged, 0);
        def_rolls.insert(CombatType::Magic, 0);

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
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup_stats(&mut self, rsn: &str) {
        let stats = fetch_player_data(rsn).unwrap();
        self.stats = parse_player_data(stats);
    }

    pub fn is_wearing(&self, gear_name: &str) -> bool {
        self.gear.head.name == gear_name
            || self.gear.neck.name == gear_name
            || self.gear.cape.name == gear_name
            || self.gear.ammo.name == gear_name
            || self.gear.weapon.name == gear_name
            || self.gear.shield.name == gear_name
            || self.gear.body.name == gear_name
            || self.gear.legs.name == gear_name
            || self.gear.hands.name == gear_name
            || self.gear.feet.name == gear_name
            || self.gear.ring.name == gear_name
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
        let level = line_parts[1].parse::<u8>().unwrap();
        skill_map.insert(*skill, level);
    }

    PlayerStats {
        hitpoints: skill_map["hitpoints"],
        attack: skill_map["attack"],
        strength: skill_map["strength"],
        defence: skill_map["defence"],
        ranged: skill_map["ranged"],
        magic: skill_map["magic"],
        prayer: skill_map["prayer"],
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::equipment::{CombatStyle, CombatType};
    use crate::spells::StandardSpell;
    use std::collections::HashMap;

    #[test]
    fn test_default_player() {
        let player = Player::new();

        let mut att_rolls = HashMap::new();
        att_rolls.insert(CombatType::Stab, 0);
        att_rolls.insert(CombatType::Slash, 0);
        att_rolls.insert(CombatType::Crush, 0);
        att_rolls.insert(CombatType::Ranged, 0);
        att_rolls.insert(CombatType::Magic, 0);

        let mut max_hits = HashMap::new();
        max_hits.insert(CombatType::Stab, 0);
        max_hits.insert(CombatType::Slash, 0);
        max_hits.insert(CombatType::Crush, 0);
        max_hits.insert(CombatType::Ranged, 0);
        max_hits.insert(CombatType::Magic, 0);

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
        assert_eq!(player.attrs.name, String::new());
        assert_eq!(player.attrs.active_style, CombatStyle::Punch);
        assert!(
            player.attrs.spell.as_any().downcast_ref::<StandardSpell>()
                == Some(&StandardSpell::None)
        );
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
}
