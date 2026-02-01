use crate::calc::rolls::calc_active_player_rolls;
use crate::combat::attacks::effects::CombatEffect;
use crate::combat::attacks::specs::{SpecialAttackFn, get_spec_attack_function};
use crate::combat::attacks::standard::{AttackFn, get_attack_functions, standard_attack};
use crate::constants;
use crate::error::{GearError, PlayerError};
use crate::types::equipment::{
    Armor, CombatStance, CombatStyle, CombatType, Equipment, EquipmentBonuses, Gear, GearSlot,
    Weapon,
};
use crate::types::monster::Monster;
use crate::types::potions::{Potion, PotionBoost, PotionBoosts, PotionStat};
use crate::types::prayers::{Prayer, PrayerBoosts};
use crate::types::spells;
use crate::types::stats::{PlayerStats, SpecEnergy, Stat};
use reqwest;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub acb_spec: bool,
    pub zcb_spec: bool,
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
            acb_spec: false,
            zcb_spec: false,
            sunfire: SunfireBoost::default(),
            soulreaper_stacks: 0,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SwitchType {
    Melee,
    Ranged,
    Magic,
    Spec(
        #[serde(
            serialize_with = "serialize_rc_str",
            deserialize_with = "deserialize_rc_str"
        )]
        Rc<str>,
    ),
    Custom(
        #[serde(
            serialize_with = "serialize_rc_str",
            deserialize_with = "deserialize_rc_str"
        )]
        Rc<str>,
    ),
}

fn serialize_rc_str<S>(rc: &Rc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(rc)
}

fn deserialize_rc_str<'de, D>(deserializer: D) -> Result<Rc<str>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Rc::from(s.as_str()))
}

impl SwitchType {
    pub fn label(&self) -> String {
        match self {
            SwitchType::Melee => "Melee".to_string(),
            SwitchType::Ranged => "Ranged".to_string(),
            SwitchType::Magic => "Magic".to_string(),
            SwitchType::Spec(spec_label) => format!("{} spec", *spec_label),
            SwitchType::Custom(custom_label) => custom_label.to_string(),
        }
    }
}

impl std::fmt::Display for SwitchType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Melee => write!(f, "Melee"),
            Self::Ranged => write!(f, "Ranged"),
            Self::Magic => write!(f, "Magic"),
            Self::Spec(s) => write!(f, "Spec ({s})"),
            Self::Custom(c) => write!(f, "Custom ({c})"),
        }
    }
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq)]
pub struct GearSwitch {
    pub switch_type: SwitchType,
    pub gear: Rc<Gear>,
    pub prayers: Rc<PrayerBoosts>,
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
            _ => SwitchType::Custom("Unknown".into()),
        };
        let attack = get_attack_functions(player);
        let spec = get_spec_attack_function(player);

        Self {
            switch_type,
            gear: Rc::clone(&player.gear),
            prayers: Rc::clone(&player.prayers),
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

    pub fn get(&self, combat_type: CombatType) -> Result<i32, PlayerError> {
        match combat_type {
            CombatType::Stab => Ok(self.stab),
            CombatType::Slash => Ok(self.slash),
            CombatType::Crush => Ok(self.crush),
            CombatType::Light => Ok(self.light),
            CombatType::Standard => Ok(self.standard),
            CombatType::Heavy => Ok(self.heavy),
            CombatType::Ranged => Err(PlayerError::NoGenericRangedStyle),
            CombatType::Magic => Ok(self.magic),
            CombatType::None => Ok(0),
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) -> Result<(), PlayerError> {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Light => self.light = value,
            CombatType::Standard => self.standard = value,
            CombatType::Heavy => self.heavy = value,
            CombatType::Ranged => return Err(PlayerError::NoGenericRangedStyle),
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }

        Ok(())
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
            CombatType::Ranged | CombatType::Light | CombatType::Standard | CombatType::Heavy => {
                self.ranged
            }
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Ranged | CombatType::Light | CombatType::Standard | CombatType::Heavy => {
                self.ranged = value;
            }
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
pub struct PlayerState {
    pub first_attack: bool,
    pub last_attack_hit: bool,
    pub current_hp: Option<u32>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            first_attack: true,
            last_attack_hit: false,
            current_hp: None,
        }
    }
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub stats: PlayerStats,
    pub gear: Rc<Gear>,
    pub bonuses: EquipmentBonuses,
    pub potions: PotionBoosts,
    pub prayers: Rc<PrayerBoosts>,
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
    pub state: PlayerState,
    combat_type: CombatType,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            stats: PlayerStats::default(),
            gear: Rc::new(Gear::default()),
            bonuses: EquipmentBonuses::default(),
            potions: PotionBoosts::default(),
            prayers: Rc::new(PrayerBoosts::default()),
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
            state: PlayerState::default(),
            combat_type: CombatType::default(),
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::default()
    }

    pub async fn lookup_stats(&mut self, rsn: &str) -> Result<(), PlayerError> {
        // Fetch stats from OSRS hiscores and set the corresponding fields
        let stats = fetch_player_data(rsn).await?;
        self.stats = parse_player_data(stats)?;
        self.attrs.name = Some(rsn.to_string());

        Ok(())
    }

    pub fn reset_current_stats(&mut self, include_spec: bool) {
        // Restore to base stats, full spec energy, and reapply potion boosts
        if include_spec {
            self.stats.reset_all();
        } else {
            let current_spec = self.stats.spec;
            self.stats.reset_all();
            self.stats.spec = current_spec;
        }
        if let Some(hp) = self.state.current_hp {
            self.stats.hitpoints.current = hp;
        }
        self.apply_potion_boosts();
    }

    pub fn is_wearing(&self, gear_name: &str, version: Option<&str>) -> bool {
        self.gear.is_wearing(gear_name, version)
    }

    pub fn is_wearing_any_version(&self, gear_name: &str) -> bool {
        self.gear.is_wearing_any_version(gear_name)
    }

    pub fn is_wearing_any<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        self.gear.is_wearing_any(gear_names)
    }

    pub fn is_wearing_all<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        self.gear.is_wearing_all(gear_names)
    }

    pub fn equip(&mut self, item_name: &str, version: Option<&str>) -> Result<(), GearError> {
        if let Ok(armor) = Armor::new(item_name, version) {
            self.equip_item(Box::new(armor))?;
        } else if let Ok(weapon) = Weapon::new(item_name, version) {
            self.equip_item(Box::new(weapon))?;
        }

        self.update_bonuses();
        self.update_set_effects();

        Ok(())
    }

    pub fn unequip_slot(&mut self, slot: &GearSlot) {
        let gear = Rc::make_mut(&mut self.gear);
        match slot {
            GearSlot::Ammo => gear.ammo = None,
            GearSlot::Body => gear.body = None,
            GearSlot::Cape => gear.cape = None,
            GearSlot::Feet => gear.feet = None,
            GearSlot::Hands => gear.hands = None,
            GearSlot::Head => gear.head = None,
            GearSlot::Legs => gear.legs = None,
            GearSlot::Neck => gear.neck = None,
            GearSlot::Ring => gear.ring = None,
            GearSlot::Shield => gear.shield = None,
            GearSlot::Weapon => {
                gear.weapon = Weapon::default();
                self.set_active_style(CombatStyle::Kick);
            }
            GearSlot::None => {}
        }
        self.update_bonuses();
        self.update_set_effects();
        self.combat_type = self.gear.weapon.combat_styles[&self.attrs.active_style].combat_type;
    }

    pub fn equip_item(&mut self, item: Box<dyn Equipment>) -> Result<(), GearError> {
        let gear = Rc::make_mut(&mut self.gear);
        let slot = item.slot();
        match slot {
            GearSlot::Weapon => {
                if let Some(weapon) = item.as_any().downcast_ref::<Weapon>() {
                    gear.weapon = weapon.clone();

                    // Unequip shield if weapon is two handed
                    if gear.weapon.is_two_handed {
                        gear.shield = None;
                    }

                    // Modify attack speed if weapon is on rapid
                    if self.attrs.active_style == CombatStyle::Rapid
                        && gear.weapon.combat_styles.contains_key(&CombatStyle::Rapid)
                    {
                        gear.weapon.speed = gear.weapon.base_speed - 1;
                    }

                    self.set_quiver_bonuses();
                } else {
                    return Err(GearError::NotAWeapon {
                        item_name: item.name().to_string(),
                        slot: item.slot().to_string(),
                    });
                }
            }
            GearSlot::Ammo => {
                // If quiver is equipped and the ammo slot is already full with a different ammo type,
                // equip the new ammo in the second_ammo slot
                if let Some(ammo) = &gear.ammo
                    && !((ammo.is_bolt() && item.name().contains("bolts"))
                        || (ammo.is_arrow() && item.name().contains("arrow")))
                    && gear.is_wearing_any_version("Dizana's quiver")
                {
                    gear.second_ammo = item.as_any().downcast_ref::<Armor>().cloned();
                    self.set_quiver_bonuses();
                } else {
                    gear.ammo = item.as_any().downcast_ref::<Armor>().cloned();

                    self.set_quiver_bonuses();
                }
            }
            GearSlot::Cape => {
                gear.cape = item.as_any().downcast_ref::<Armor>().cloned();
                self.set_quiver_bonuses();
            }
            GearSlot::Shield => {
                gear.shield = item.as_any().downcast_ref::<Armor>().cloned();
                if gear.weapon.is_two_handed {
                    gear.weapon = Weapon::default();
                }
            }
            GearSlot::Body => gear.body = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Feet => gear.feet = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Hands => gear.hands = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Head => gear.head = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Legs => gear.legs = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Neck => gear.neck = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Ring => gear.ring = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::None => {
                return Err(GearError::NoneSlot(item.name().to_string()));
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
        let gear = Rc::make_mut(&mut self.gear);
        // Apply extra +10 accuracy and +1 strength to quiver if applicable
        if gear.is_quiver_bonus_valid()
            && let Some(cape) = &mut gear.cape
        {
            cape.bonuses.attack.ranged = 28;
            cape.bonuses.strength.ranged = 4;
        } else if gear.is_wearing_any_version("Dizana's quiver")
            && let Some(cape) = &mut gear.cape
        {
            cape.bonuses.attack.ranged = 18;
            cape.bonuses.strength.ranged = 3;
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
        if !constants::USES_OWN_AMMO.contains(&(
            self.gear.weapon.name.as_str(),
            self.gear.weapon.version.as_deref(),
        )) {
            for item in [&self.gear.ammo, &self.gear.second_ammo]
                .into_iter()
                .flatten()
            {
                self.bonuses.add_bonuses(&item.bonuses);
            }
        } else if let Some(ammo) = &self.gear.ammo
            && !ammo.is_valid_ranged_ammo()
        {
            self.bonuses.add_bonuses(&ammo.bonuses);
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
        self.set_effects.full_ahrims = self.is_wearing_all(constants::FULL_AHRIMS);
        self.set_effects.full_blood_moon = self.is_wearing_all(constants::FULL_BLOOD_MOON);
        self.set_effects.full_blue_moon = self.is_wearing_all(constants::FULL_BLUE_MOON);
        self.set_effects.full_dharoks = self.is_wearing_all(constants::FULL_DHAROKS);
        self.set_effects.full_guthans = self.is_wearing_all(constants::FULL_GUTHANS);
        self.set_effects.full_eclipse_moon = self.is_wearing_all(constants::FULL_ECLIPSE_MOON);
        self.set_effects.full_inquisitor = self.is_wearing_all(constants::FULL_INQUISITOR);
        self.set_effects.full_justiciar = self.is_wearing_all(constants::FULL_JUSTICIAR);
        self.set_effects.full_karils = self.is_wearing_all(constants::FULL_KARILS);
        self.set_effects.full_obsidian = self.is_wearing_all(constants::FULL_OBSIDIAN);
        self.set_effects.full_torags = self.is_wearing_all(constants::FULL_TORAGS);
        self.set_effects.full_void = self.is_wearing_full_void();
        self.set_effects.full_elite_void = self.is_wearing_full_elite_void();
        self.set_effects.bloodbark_pieces = constants::BLOODBARK_ARMOR
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
                        &PotionStat::Attack,
                    );
                } else if potion.potion_type == Potion::ZamorakBrew {
                    potion.calc_zamorak_brew_boost(self.stats.attack, &PotionStat::Attack);
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
                        &PotionStat::Strength,
                    );
                } else if potion.potion_type == Potion::DragonBattleaxe {
                    potion.calc_dragon_battleaxe_boost(
                        self.stats.attack,
                        self.stats.defence,
                        self.stats.ranged,
                        self.stats.magic,
                    );
                } else if potion.potion_type == Potion::ZamorakBrew {
                    potion.calc_zamorak_brew_boost(self.stats.strength, &PotionStat::Strength);
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
                        &PotionStat::Defence,
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
        self.combat_type
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
        let is_using_standard_spell = self.is_using_standard_spell();

        let gear = Rc::make_mut(&mut self.gear);

        // Reduce attack speed by 1 on rapid
        if stance == CombatStance::Rapid {
            gear.weapon.speed = gear.weapon.base_speed - 1;
        } else if [
            CombatStance::DefensiveAutocast,
            CombatStance::Autocast,
            CombatStance::ManualCast,
        ]
        .contains(&stance)
        {
            // Prevent staff speed from being set to its melee attack speed if player is casting spells
            gear.weapon.speed =
                if gear.is_wearing("Harmonised nightmare staff", None) && is_using_standard_spell {
                    4
                } else if gear.is_wearing("Twinflame staff", None) {
                    6
                } else {
                    5
                }
        } else {
            gear.weapon.speed = gear.weapon.base_speed;
        }

        self.combat_type = gear.weapon.combat_styles[&style].combat_type;
    }

    pub fn switch(&mut self, switch_type: &SwitchType) -> Result<(), PlayerError> {
        if let Some(current) = &self.current_switch
            && current == switch_type
        {
            return Ok(());
        }

        for switch in &self.switches {
            if &switch.switch_type == switch_type {
                self.gear = Rc::clone(&switch.gear);
                self.prayers = Rc::clone(&switch.prayers);
                self.attrs.spell = switch.spell;
                self.attrs.active_style = switch.active_style;
                self.set_effects = switch.set_effects;
                self.attack = switch.attack;
                self.spec = switch.spec;
                self.att_rolls = switch.att_rolls;
                self.max_hits = switch.max_hits;
                self.def_rolls = switch.def_rolls;
                self.current_switch = Some(switch.switch_type.clone());
                self.combat_type =
                    self.gear.weapon.combat_styles[&self.attrs.active_style].combat_type;

                return Ok(());
            }
        }
        Err(PlayerError::GearSwitchNotFound(switch_type.clone()))
    }

    pub fn is_wearing_black_mask(&self) -> bool {
        // Check if the player is wearing any type of black mask or slayer helmet
        self.is_wearing_any(constants::BLACK_MASKS)
    }

    pub fn is_wearing_imbued_black_mask(&self) -> bool {
        // Check if the player is wearing an imbued black mask or slayer helmet
        self.is_wearing_any(constants::BLACK_MASKS_IMBUED)
    }

    pub fn is_wearing_salve(&self) -> bool {
        // Check if the player is wearing an unenchanted salve amulet
        self.is_wearing_any(constants::SALVE_UNENCHANTED)
    }

    pub fn is_wearing_salve_e(&self) -> bool {
        // Check if the player is wearing an enchanted salve amulet
        self.is_wearing_any(constants::SALVE_ENCHANTED)
    }

    pub fn is_wearing_salve_i(&self) -> bool {
        // Check if the player is wearing an imbued salve amulet
        self.is_wearing_any(constants::SALVE_IMBUED)
    }

    pub fn is_wearing_wildy_mace(&self) -> bool {
        // Check if the player is wearing either type of wilderness mace
        self.is_wearing_any(constants::WILDY_MACES)
    }

    pub fn is_wearing_wildy_bow(&self) -> bool {
        // Check if the player is wearing either type of wilderness bow
        self.is_wearing_any(constants::WILDY_BOWS)
    }

    pub fn is_wearing_wildy_staff(&self) -> bool {
        // Check if the player is wearing any form of wilderness staff
        self.is_wearing_any(constants::WILDY_STAVES)
    }

    pub fn is_wearing_elf_bow(&self) -> bool {
        // Check if the player is wearing a crystal bow or bowfa
        self.is_wearing_any(constants::ELF_BOWS)
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
        self.is_wearing_any(constants::SMOKE_STAVES)
    }

    pub fn is_wearing_silver_weapon(&self) -> bool {
        // Check if the player is wearing any type of silver weapon
        self.is_wearing_any(constants::SILVER_WEAPONS)
            || (self.combat_type() == CombatType::Ranged && self.is_wearing("Silver bolts", None))
    }

    pub fn is_wearing_ivandis_weapon(&self) -> bool {
        // Check if the player is wearing one of the weapons that can harm T3 vampyres
        self.is_wearing_any(constants::IVANDIS_WEAPONS)
    }

    pub fn is_wearing_keris(&self) -> bool {
        // Check if the player is wearing any type of keris
        self.is_wearing_any(constants::KERIS_WEAPONS)
    }

    pub fn is_wearing_leaf_bladed_weapon(&self) -> bool {
        // Check if the player is wearing any type of leaf-bladed weapon or broad bolts
        (self.is_using_melee() && self.is_wearing_any(constants::LEAF_BLADED_WEAPONS))
            || (self.combat_type() == CombatType::Ranged
                && (self.is_using_crossbow() && self.is_wearing_any(constants::BROAD_BOLTS)))
            || self.is_wearing("Broad arrows", None)
    }

    pub fn is_wearing_full_void(&self) -> bool {
        // Check if the player is wearing a full void set
        constants::FULL_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_full_elite_void(&self) -> bool {
        // Check if the player is wearing a full elite void set
        constants::FULL_ELITE_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_ancient_spectre(&self) -> bool {
        // Check if the player is wearing any type of ancient spectre
        self.is_wearing_any(constants::ANCIENT_SPECTRES)
    }

    pub fn is_wearing_ratbone_weapon(&self) -> bool {
        // Check if the player is wearing any type of ratbone weapon
        self.is_wearing_any(constants::RATBANE_WEAPONS)
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
                .is_some_and(|a| a.name.contains("bolt"))
    }

    pub fn is_using_demonbane(&self) -> bool {
        self.is_using_demonbane_spell() || self.is_wearing_any(constants::DEMONBANE_WEAPONS)
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

    pub fn set_spell(&mut self, spell: spells::Spell) -> Result<(), PlayerError> {
        if spell.required_level() > self.stats.magic.current {
            return Err(PlayerError::MagicLevelTooLow(spell));
        }
        self.attrs.spell = Some(spell);

        Ok(())
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

    pub fn add_prayer(&mut self, prayer: Prayer) {
        Rc::make_mut(&mut self.prayers).add(prayer);
    }

    pub fn remove_prayer(&mut self, prayer: Prayer) {
        Rc::make_mut(&mut self.prayers).remove(prayer);
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
            CombatEffect::Poison { tick_counter, .. }
            | CombatEffect::Venom { tick_counter, .. }
            | CombatEffect::Burn { tick_counter, .. }
            | CombatEffect::DelayedHeal { tick_counter, .. }
            | CombatEffect::DamageOverTime { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::DelayedAttack { tick_delay, .. } => tick_delay.is_some(),
        });
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
        self.is_wearing_any(constants::OGRE_BOWS)
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

    pub fn rolls_accuracy_twice(&self) -> bool {
        !self.state.last_attack_hit
            && self.combat_type() == CombatType::Magic
            && self.is_wearing("Confliction gauntlets", None)
    }
}

/// Builder for constructing `Player` instances.
///
/// # Example
/// ```
/// use osrs::types::player::Player;
/// use osrs::types::potions::Potion;
/// use osrs::types::prayers::Prayer;
/// use osrs::types::equipment::CombatStyle;
///
/// let player = Player::builder()
///     .attack(90)
///     .strength(90)
///     .defence(90)
///     .potion(Potion::SuperCombat)
///     .prayer(Prayer::Piety)
///     .active_style(CombatStyle::Lunge)
///     .build()?;
///
/// Ok(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct PlayerBuilder {
    stats: Option<PlayerStats>,
    gear: Option<Gear>,
    potions: Vec<Potion>,
    prayers: Vec<Prayer>,
    boosts: Option<StatusBoosts>,
    spell: Option<spells::Spell>,
    active_style: Option<CombatStyle>,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set stats using a pre-built `PlayerStats` struct.
    pub fn player_stats(mut self, stats: PlayerStats) -> Self {
        self.stats = Some(stats);
        self
    }

    /// Set the attack level.
    pub fn attack(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).attack = Stat::new(level, None);
        self
    }

    /// Set the strength level.
    pub fn strength(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).strength = Stat::new(level, None);
        self
    }

    /// Set the defence level.
    pub fn defence(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).defence = Stat::new(level, None);
        self
    }

    /// Set the ranged level.
    pub fn ranged(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).ranged = Stat::new(level, None);
        self
    }

    /// Set the magic level.
    pub fn magic(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).magic = Stat::new(level, None);
        self
    }

    /// Set the hitpoints level.
    pub fn hitpoints(mut self, level: u32) -> Self {
        self.stats
            .get_or_insert_with(PlayerStats::default)
            .hitpoints = Stat::new(level, None);
        self
    }

    /// Set the prayer level.
    pub fn prayer_level(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).prayer = Stat::new(level, None);
        self
    }

    /// Set the mining level.
    pub fn mining(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).mining = Stat::new(level, None);
        self
    }

    /// Set the herblore level.
    pub fn herblore(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).herblore = Stat::new(level, None);
        self
    }

    /// Set gear using a pre-built `Gear` struct.
    pub fn gear(mut self, gear: Gear) -> Self {
        self.gear = Some(gear);
        self
    }

    /// Add a potion boost.
    pub fn potion(mut self, potion: Potion) -> Self {
        self.potions.push(potion);
        self
    }

    /// Add a prayer.
    pub fn prayer(mut self, prayer: Prayer) -> Self {
        self.prayers.push(prayer);
        self
    }

    /// Set the spell to autocast.
    pub fn spell(mut self, spell: spells::Spell) -> Self {
        self.spell = Some(spell);
        self
    }

    /// Set the active combat style.
    pub fn active_style(mut self, style: CombatStyle) -> Self {
        self.active_style = Some(style);
        self
    }

    /// Set status boosts using a pre-built `StatusBoosts` struct.
    pub fn status_boosts(mut self, boosts: StatusBoosts) -> Self {
        self.boosts = Some(boosts);
        self
    }

    /// Set whether the player is on a slayer task.
    pub fn on_task(mut self, on_task: bool) -> Self {
        self.boosts
            .get_or_insert_with(StatusBoosts::default)
            .on_task = on_task;
        self
    }

    /// Set whether the player is in the wilderness.
    pub fn in_wilderness(mut self, in_wilderness: bool) -> Self {
        self.boosts
            .get_or_insert_with(StatusBoosts::default)
            .in_wilderness = in_wilderness;
        self
    }

    /// Set whether the player has Kandarin Hard Diary completed (affects bolt procs).
    pub fn kandarin_diary(mut self, completed: bool) -> Self {
        self.boosts
            .get_or_insert_with(StatusBoosts::default)
            .kandarin_diary = completed;
        self
    }

    /// Build the `Player` instance.
    pub fn build(self) -> Result<Player, PlayerError> {
        let mut player = Player {
            stats: self.stats.unwrap_or_default(),
            gear: Rc::new(self.gear.unwrap_or_default()),
            bonuses: EquipmentBonuses::default(),
            potions: PotionBoosts::default(),
            prayers: Rc::new(PrayerBoosts::default()),
            boosts: self.boosts.unwrap_or_default(),
            active_effects: Vec::new(),
            set_effects: SetEffects::default(),
            attrs: PlayerAttrs {
                name: None,
                active_style: self.active_style.unwrap_or(CombatStyle::Punch),
                spell: self.spell,
            },
            att_rolls: PlayerAttRolls::default(),
            max_hits: PlayerMaxHits::default(),
            def_rolls: PlayerDefRolls::default(),
            attack: standard_attack,
            spec: standard_attack,
            switches: Vec::new(),
            current_switch: None,
            state: PlayerState::default(),
            combat_type: CombatType::default(),
        };

        // Apply potions
        for potion in self.potions {
            player.add_potion(potion);
        }

        // Apply prayers
        for prayer in self.prayers {
            player.add_prayer(prayer);
        }

        // Update bonuses and set effects based on gear
        player.update_bonuses();
        player.update_set_effects();

        // Set combat type based on active style
        if player
            .gear
            .weapon
            .combat_styles
            .contains_key(&player.attrs.active_style)
        {
            player.set_active_style(player.attrs.active_style);
        } else if self.active_style.is_some() {
            return Err(PlayerError::CombatStyleMismatch {
                weapon_name: player.gear.weapon.name.clone(),
                style: player.attrs.active_style,
            });
        }

        Ok(player)
    }
}

pub async fn fetch_player_data(rsn: &str) -> Result<String, PlayerError> {
    // Fetches player data from the OSRS hiscores
    let url = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";
    let params = [("player", rsn)];
    let client = reqwest::Client::new();
    let response = client.get(url).query(&params).send().await?; // Use .await
    let data = response.text().await?; // Use .await
    Ok(data)
}

pub fn parse_player_data(data: String) -> Result<PlayerStats, PlayerError> {
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
        let level = line_parts[1].parse::<u32>()?;
        skill_map.insert(*skill, level);
    }

    let mining_lvl = data_lines[15].split(',').collect::<Vec<&str>>()[1];
    skill_map.insert("mining", mining_lvl.parse::<u32>()?);
    let herblore_lvl = data_lines[16].split(',').collect::<Vec<&str>>()[1];
    skill_map.insert("herblore", herblore_lvl.parse::<u32>()?);

    Ok(PlayerStats {
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
    })
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
        assert_eq!(player.gear, Rc::new(Gear::default()));
        assert_eq!(player.bonuses, EquipmentBonuses::default());
        assert_eq!(player.potions, PotionBoosts::default());
        assert_eq!(player.prayers, Rc::new(PrayerBoosts::default()));
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
        player.lookup_stats("Lynx Titan").await.unwrap();
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
        let gear = Rc::make_mut(&mut player.gear);
        gear.head = Some(Armor::default());
        gear.head.as_mut().unwrap().name = "Torva full helm".to_string();
        assert!(player.is_wearing("Torva full helm", None));
    }

    #[test]
    fn test_equip_armor() {
        let mut player = Player::new();
        player.equip("Torva full helm", None).unwrap();
        player.update_bonuses();
        let torva_full_helm =
            Armor::new("Torva full helm", None).expect("Error creating equipment.");
        assert_eq!(player.gear.head.clone().unwrap(), torva_full_helm);
        assert_eq!(player.bonuses, torva_full_helm.bonuses);
    }

    #[test]
    fn test_equip_weapon() {
        let mut player = Player::new();
        player.equip("Osmumten's fang", None).unwrap();
        player.update_bonuses();
        let osmumtens_fang =
            Weapon::new("Osmumten's fang", None).expect("Error creating equipment.");
        assert_eq!(player.gear.weapon, osmumtens_fang);
        assert_eq!(player.bonuses, osmumtens_fang.bonuses);
    }

    #[test]
    fn test_replace_gear() {
        let mut player = Player::new();
        player.equip("Torva full helm", None).unwrap();
        player.update_bonuses();
        player.equip("Neitiznot faceguard", None).unwrap();
        player.update_bonuses();
        let neitiznot_faceguard =
            Armor::new("Neitiznot faceguard", None).expect("Error creating equipment.");
        assert_eq!(player.gear.head.clone().unwrap(), neitiznot_faceguard);
        assert_eq!(player.bonuses, neitiznot_faceguard.bonuses);
    }

    #[test]
    fn test_max_melee_bonuses() {
        let mut player = Player::new();
        let max_melee_gear = Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            neck: Some(Armor::new("Amulet of torture", None).expect("Error creating equipment.")),
            cape: Some(Armor::new("Infernal cape", None).expect("Error creating equipment.")),
            ammo: Some(Armor::new("Rada's blessing 4", None).expect("Error creating equipment.")),
            second_ammo: None,
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating equipment."),
            shield: Some(Armor::new("Avernic defender", None).expect("Error creating equipment.")),
            body: Some(Armor::new("Torva platebody", None).expect("Error creating equipment.")),
            legs: Some(Armor::new("Torva platelegs", None).expect("Error creating equipment.")),
            hands: Some(Armor::new("Ferocious gloves", None).expect("Error creating equipment.")),
            feet: Some(Armor::new("Primordial boots", None).expect("Error creating equipment.")),
            ring: Some(Armor::new("Ultor ring", None).expect("Error creating equipment.")),
        };
        player.gear = Rc::new(max_melee_gear);
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
        let prayers = Rc::make_mut(&mut player.prayers);
        prayers.add(Prayer::Chivalry);
        assert_eq!(prayers.attack, 15);
        assert_eq!(prayers.strength, 18);
        assert_eq!(prayers.defence, 20);
        prayers.add(Prayer::Piety);
        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.prayers.strength, 23);
        assert_eq!(player.prayers.defence, 25);
    }

    #[test]
    fn test_twinflame_detection() {
        let mut player = Player::new();
        player.equip("Twinflame staff", None).unwrap();
        player
            .set_spell(Spell::Standard(StandardSpell::EarthBolt))
            .unwrap();
        assert!(player.gets_second_twinflame_hit());

        player
            .set_spell(Spell::Standard(StandardSpell::EarthSurge))
            .unwrap();
        assert!(!player.gets_second_twinflame_hit());
    }

    #[test]
    fn test_builder_default() {
        let player = Player::builder().build().expect("Error building player.");

        assert_eq!(player.stats, PlayerStats::default());
        assert!(player.attrs.name.is_none());
        assert_eq!(player.attrs.active_style, CombatStyle::Punch);
    }

    #[test]
    fn test_builder_with_individual_stats() {
        let player = Player::builder()
            .attack(75)
            .strength(80)
            .defence(70)
            .ranged(90)
            .magic(85)
            .hitpoints(95)
            .prayer_level(77)
            .mining(70)
            .herblore(71)
            .build()
            .expect("Error building player.");

        assert_eq!(player.stats.attack.base, 75);
        assert_eq!(player.stats.strength.base, 80);
        assert_eq!(player.stats.defence.base, 70);
        assert_eq!(player.stats.ranged.base, 90);
        assert_eq!(player.stats.magic.base, 85);
        assert_eq!(player.stats.hitpoints.base, 95);
        assert_eq!(player.stats.prayer.base, 77);
        assert_eq!(player.stats.mining.base, 70);
        assert_eq!(player.stats.herblore.base, 71);
    }

    #[test]
    fn test_builder_with_potions() {
        let player = Player::builder()
            .potion(Potion::SuperAttack)
            .potion(Potion::SuperStrength)
            .build()
            .expect("Error building player.");

        // Potions should be applied and stats boosted
        assert_eq!(player.stats.attack.current, 118);
        assert_eq!(player.stats.strength.current, 118);
    }

    #[test]
    fn test_builder_with_prayers() {
        let player = Player::builder()
            .prayer(Prayer::Piety)
            .build()
            .expect("Error building player.");

        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.prayers.strength, 23);
        assert_eq!(player.prayers.defence, 25);
    }

    #[test]
    fn test_builder_with_gear() {
        let gear = Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            ..Default::default()
        };

        let player = Player::builder()
            .gear(gear)
            .build()
            .expect("Error building player.");

        assert!(player.is_wearing("Torva full helm", None));
    }

    #[test]
    fn test_builder_with_status_boosts() {
        let player = Player::builder()
            .on_task(false)
            .in_wilderness(true)
            .kandarin_diary(false)
            .build()
            .expect("Error building player.");

        assert!(!player.boosts.on_task);
        assert!(player.boosts.in_wilderness);
        assert!(!player.boosts.kandarin_diary);
    }

    #[test]
    fn test_builder_with_spell() {
        let player = Player::builder()
            .spell(Spell::Standard(StandardSpell::FireSurge))
            .build()
            .expect("Error building player.");

        assert_eq!(
            player.attrs.spell,
            Some(Spell::Standard(StandardSpell::FireSurge))
        );
    }

    #[test]
    fn test_builder_with_active_style() {
        let gear = Gear {
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating weapon."),
            ..Default::default()
        };

        let player = Player::builder()
            .gear(gear)
            .active_style(CombatStyle::Lunge)
            .build()
            .expect("Error building player.");

        assert_eq!(player.attrs.active_style, CombatStyle::Lunge);
        assert_eq!(player.combat_type(), CombatType::Stab);
    }

    #[test]
    fn test_builder_full_setup() {
        let gear = Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            neck: Some(Armor::new("Amulet of torture", None).expect("Error creating equipment.")),
            body: Some(Armor::new("Torva platebody", None).expect("Error creating equipment.")),
            legs: Some(Armor::new("Torva platelegs", None).expect("Error creating equipment.")),
            hands: Some(Armor::new("Ferocious gloves", None).expect("Error creating equipment.")),
            feet: Some(Armor::new("Primordial boots", None).expect("Error creating equipment.")),
            cape: Some(Armor::new("Infernal cape", None).expect("Error creating equipment.")),
            ring: Some(Armor::new("Ultor ring", None).expect("Error creating equipment.")),
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating weapon."),
            shield: Some(Armor::new("Avernic defender", None).expect("Error creating equipment.")),
            ..Default::default()
        };

        let player = Player::builder()
            .gear(gear)
            .potion(Potion::SuperCombat)
            .prayer(Prayer::Piety)
            .active_style(CombatStyle::Lunge)
            .on_task(true)
            .build()
            .expect("Error building player.");

        assert_eq!(player.stats.attack.current, 118);
        assert_eq!(player.stats.strength.current, 118);
        assert_eq!(player.stats.defence.current, 118);
        assert!(player.is_wearing("Torva full helm", None));
        assert!(player.is_wearing("Osmumten's fang", None));
        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.attrs.active_style, CombatStyle::Lunge);
        assert!(player.boosts.on_task);
    }

    #[test]
    fn test_builder_invalid_combat_style() {
        let gear = Gear {
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating weapon."),
            ..Default::default()
        };

        let result = Player::builder()
            .gear(gear)
            .active_style(CombatStyle::Rapid)
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            PlayerError::CombatStyleMismatch { weapon_name, style }
                if weapon_name == "Osmumten's fang" && style == CombatStyle::Rapid
        ));
    }
}
