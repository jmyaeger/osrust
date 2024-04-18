use crate::equipment::{self, Armor, CombatStyle, CombatType, EquipmentBonuses, Weapon};
use crate::potions::PotionBoost;
use crate::prayers::PrayerBoost;
use crate::spells::{Spell, StandardSpell};
use reqwest::Error;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PlayerStats {
    pub hitpoints: u16,
    pub attack: u16,
    pub strength: u16,
    pub defence: u16,
    pub ranged: u16,
    pub magic: u16,
    pub prayer: u16,
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
    pub hitpoints: u16,
    pub attack: u16,
    pub strength: u16,
    pub defence: u16,
    pub ranged: u16,
    pub magic: u16,
    pub prayer: u16,
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
    pub attack: u16,
    pub strength: u16,
    pub defence: u16,
    pub ranged_att: u16,
    pub ranged_str: u16,
    pub magic: u16,
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
        let stats = fetch_player_data(rsn).expect("Failed to fetch player data");
        self.stats = parse_player_data(stats);
    }

    pub fn reset_live_stats(&mut self) {
        self.live_stats.attack = self.stats.attack;
        self.live_stats.strength = self.stats.strength;
        self.live_stats.defence = self.stats.defence;
        self.live_stats.ranged = self.stats.ranged;
        self.live_stats.magic = self.stats.magic;
        self.live_stats.prayer = self.stats.prayer;
        self.live_stats.hitpoints = self.stats.hitpoints;
        self.live_stats.special_attack = 100;
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

    pub fn equip(&mut self, item_name: &str) {
        let slot_name = equipment::get_slot_name(item_name)
            .unwrap_or_else(|_| panic!("Slot not found for item {}", item_name));
        match slot_name.as_str() {
            "head" => self.gear.head = Armor::new(item_name),
            "neck" => self.gear.neck = Armor::new(item_name),
            "cape" => self.gear.cape = Armor::new(item_name),
            "ammo" => self.gear.ammo = Armor::new(item_name),
            "weapon" | "2h" => self.gear.weapon = Weapon::new(item_name),
            "shield" => self.gear.shield = Armor::new(item_name),
            "body" => self.gear.body = Armor::new(item_name),
            "legs" => self.gear.legs = Armor::new(item_name),
            "hands" => self.gear.hands = Armor::new(item_name),
            "feet" => self.gear.feet = Armor::new(item_name),
            "ring" => self.gear.ring = Armor::new(item_name),
            _ => panic!("Slot not found for item {}", item_name),
        }
    }

    pub fn update_bonuses(&mut self) {
        self.bonuses = EquipmentBonuses::default();

        for item in [
            &self.gear.head,
            &self.gear.neck,
            &self.gear.cape,
            &self.gear.ammo,
            &self.gear.shield,
            &self.gear.body,
            &self.gear.legs,
            &self.gear.hands,
            &self.gear.feet,
            &self.gear.ring,
        ] {
            self.bonuses.add_bonuses(&item.bonuses)
        }
        self.bonuses.add_bonuses(&self.gear.weapon.bonuses);
    }

    pub fn calc_potion_boosts(&mut self) {
        self.potions.attack.calc_boost(self.stats.attack);
        self.potions.strength.calc_boost(self.stats.strength);
        self.potions.defence.calc_boost(self.stats.defence);
        self.potions.ranged.calc_boost(self.stats.ranged);
        self.potions.magic.calc_boost(self.stats.magic);
    }

    pub fn apply_potion_boosts(&mut self) {
        self.reset_live_stats();
        self.live_stats.attack += self.potions.attack.boost;
        self.live_stats.strength += self.potions.strength.boost;
        self.live_stats.defence += self.potions.defence.boost;
        self.live_stats.ranged += self.potions.ranged.boost;
        self.live_stats.magic += self.potions.magic.boost;
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
            .parse::<u16>()
            .expect("Level could not be parsed as u8.");
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
    use crate::equipment::{CombatStyle, CombatType, StrengthBonus, StyleBonus};
    use crate::potions::Potion;
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

    #[test]
    fn test_is_wearing() {
        let mut player = Player::new();
        player.gear.head.name = "Torva full helm".to_string();
        assert!(player.is_wearing("Torva full helm"));
    }

    #[test]
    fn test_equip_armor() {
        let mut player = Player::new();
        player.equip("Torva full helm");
        player.update_bonuses();
        let torva_full_helm = Armor::new("Torva full helm");
        assert_eq!(player.gear.head, torva_full_helm);
        assert_eq!(player.bonuses, torva_full_helm.bonuses)
    }

    #[test]
    fn test_equip_weapon() {
        let mut player = Player::new();
        player.equip("Osmumten's fang");
        player.update_bonuses();
        let osmumtens_fang = Weapon::new("Osmumten's fang");
        assert_eq!(player.gear.weapon, osmumtens_fang);
        assert_eq!(player.bonuses, osmumtens_fang.bonuses)
    }

    #[test]
    fn test_replace_gear() {
        let mut player = Player::new();
        player.equip("Torva full helm");
        player.update_bonuses();
        player.equip("Neitiznot faceguard");
        player.update_bonuses();
        let neitiznot_faceguard = Armor::new("Neitiznot faceguard");
        assert_eq!(player.gear.head, neitiznot_faceguard);
        assert_eq!(player.bonuses, neitiznot_faceguard.bonuses)
    }

    #[test]
    fn test_max_melee_bonuses() {
        let mut player = Player::new();
        let max_melee_gear = Gear {
            head: Armor::new("Torva full helm"),
            neck: Armor::new("Amulet of torture"),
            cape: Armor::new("Infernal cape"),
            ammo: Armor::new("Rada's blessing 4"),
            weapon: Weapon::new("Osmumten's fang"),
            shield: Armor::new("Avernic defender"),
            body: Armor::new("Torva platebody"),
            legs: Armor::new("Torva platelegs"),
            hands: Armor::new("Ferocious gloves"),
            feet: Armor::new("Primordial boots"),
            ring: Armor::new("Ultor ring"),
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
        };
        player.potions.attack = PotionBoost::new(Potion::SuperAttack);
        player.potions.strength = PotionBoost::new(Potion::SuperStrength);
        player.potions.defence = PotionBoost::new(Potion::SuperDefence);
        player.potions.ranged = PotionBoost::new(Potion::Ranging);
        player.potions.magic = PotionBoost::new(Potion::SaturatedHeart);

        player.calc_potion_boosts();
        player.apply_potion_boosts();

        assert_eq!(player.live_stats.attack, 118);
        assert_eq!(player.live_stats.strength, 118);
        assert_eq!(player.live_stats.defence, 118);
        assert_eq!(player.live_stats.ranged, 112);
        assert_eq!(player.live_stats.magic, 112);
    }

    #[test]
    fn test_dragon_battleaxe_boost() {
        let mut player = Player::new();
        player.stats = PlayerStats {
            attack: 99,
            strength: 99,
            defence: 99,
            ranged: 99,
            magic: 99,
            hitpoints: 99,
            prayer: 99,
        };
        player.potions.attack = PotionBoost::new(Potion::ZamorakBrewAtt);
        player.potions.defence = PotionBoost::new(Potion::SuperDefence);
        player.potions.magic = PotionBoost::new(Potion::Magic);
        player.potions.ranged = PotionBoost::new(Potion::Ranging);
        player.potions.strength = PotionBoost::new(Potion::DragonBattleaxe);
        player.calc_potion_boosts();
        player.apply_potion_boosts();
        player.potions.strength.calc_dragon_battleaxe_boost(
            player.live_stats.attack,
            player.live_stats.defence,
            player.live_stats.ranged,
            player.live_stats.magic,
        );
        player.apply_potion_boosts();

        assert_eq!(player.live_stats.attack, 120);
        assert_eq!(player.live_stats.strength, 120);
        assert_eq!(player.live_stats.defence, 118);
        assert_eq!(player.live_stats.ranged, 112);
        assert_eq!(player.live_stats.magic, 103);
    }
}
