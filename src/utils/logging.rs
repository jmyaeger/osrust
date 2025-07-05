use crate::types::equipment::CombatType;
use crate::types::monster::{AttackType, Monster};
use crate::types::player::Player;
use chrono::Local;
use log::{debug, LevelFilter};
use simplelog::{Config, WriteLogger};
use std::fs::File;

#[derive(Debug, PartialEq, Clone)]
pub struct FightLogger {
    enabled: bool,
}

impl FightLogger {
    pub fn new(enabled: bool, name: &str) -> Self {
        if enabled {
            std::fs::create_dir_all("logs").unwrap_or_else(|e| {
                eprintln!("Failed to create log directory: {e}");
            });

            let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
            let filename = format!("logs/{name}_{timestamp}.log");

            WriteLogger::init(
                LevelFilter::Debug,
                Config::default(),
                File::create(filename).unwrap_or_else(|e| {
                    eprintln!("Failed to create log file: {e}",);
                    panic!("Failed to create log file");
                }),
            )
            .unwrap();
        }
        Self { enabled }
    }

    pub fn log_initial_setup(&mut self, player: &Player, monster: &Monster) {
        if self.enabled {
            debug!("Initial setup:");
            debug!("Player's active combat style: {}", player.combat_type());
            debug!(
                "Player's max attack roll: {}",
                player.att_rolls.get(player.combat_type())
            );
            debug!(
                "Player's max hit: {}",
                player.max_hits.get(player.combat_type())
            );
            debug!(
                "Player's max defence rolls\n: {} (Stab), {} (Slash), {} (Crush), {} (Ranged), {} (Magic)\n", 
                player.def_rolls.get(CombatType::Stab),
                player.def_rolls.get(CombatType::Slash),
                player.def_rolls.get(CombatType::Crush),
                player.def_rolls.get(CombatType::Ranged),
                player.def_rolls.get(CombatType::Magic)
            );
            debug!("Player's stats (with boosts):");
            debug!("Attack: {}", player.stats.attack.current);
            debug!("Strength: {}", player.stats.strength.current);
            debug!("Ranged: {}", player.stats.ranged.current);
            debug!("Defence: {}", player.stats.defence.current);
            debug!("Magic: {}", player.stats.magic.current);
            debug!("Prayer: {}", player.stats.prayer.current);
            debug!("Hitpoints: {}\n", player.stats.hitpoints.current);
            debug!("Active prayers:");
            if let Some(prayers) = &player.prayers.active_prayers {
                for prayer in prayers {
                    debug!("{prayer}");
                }
            } else {
                debug!("None\n");
            }
            debug!("Player's equipment:");
            debug!(
                "Head: {}",
                player
                    .gear
                    .head
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Neck: {}",
                player
                    .gear
                    .neck
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Cape: {}",
                player
                    .gear
                    .cape
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Ammo: {}",
                player
                    .gear
                    .ammo
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!("Weapon: {}", player.gear.weapon.name);
            debug!(
                "Shield: {}",
                player
                    .gear
                    .shield
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Body: {}",
                player
                    .gear
                    .body
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Hands: {}",
                player
                    .gear
                    .hands
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Legs: {}",
                player
                    .gear
                    .legs
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Feet: {}",
                player
                    .gear
                    .feet
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );
            debug!(
                "Ring: {}",
                player
                    .gear
                    .ring
                    .as_ref()
                    .map_or("None".to_string(), |armor| armor.name.clone())
            );

            debug!("\n");
            debug!("Monster's stats:");
            debug!("Attack: {}", monster.stats.attack.current);
            debug!("Strength: {}", monster.stats.strength.current);
            debug!("Ranged: {}", monster.stats.ranged.current);
            debug!("Defence: {}", monster.stats.defence.current);
            debug!("Magic: {}", monster.stats.magic.current);
            debug!("Hitpoints: {}\n", monster.stats.hitpoints.current);

            debug!(
                "Monster's max attack rolls: {} (Stab), {} (Slash), {} (Crush), {} (Ranged), {} (Magic)\n",
                monster.att_rolls.get(CombatType::Stab),
                monster.att_rolls.get(CombatType::Slash),
                monster.att_rolls.get(CombatType::Crush),
                monster.att_rolls.get(CombatType::Ranged),
                monster.att_rolls.get(CombatType::Magic)
            );
            if let Some(max_hits) = &monster.max_hits {
                debug!(
                    "Monster's max hit(s): {}\n",
                    max_hits
                        .iter()
                        .map(|max_hit| format!("{} ({})", max_hit.value, max_hit.style))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            } else {
                debug!("Monster has no stored max hits.\n");
            }
            debug!(
                "Monster's max defence rolls: {} (Stab), {} (Slash), {} (Crush), {} (Light), {} (Standard), {} (Heavy), {} (Magic)\n", 
                monster.def_rolls.get(CombatType::Stab),
                monster.def_rolls.get(CombatType::Slash),
                monster.def_rolls.get(CombatType::Crush),
                monster.def_rolls.get(CombatType::Light),
                monster.def_rolls.get(CombatType::Standard),
                monster.def_rolls.get(CombatType::Heavy),
                monster.def_rolls.get(CombatType::Magic)
            );
        }
    }

    pub fn log_player_attack(&mut self, tick: i32, damage: u32, success: bool, style: CombatType) {
        if self.enabled {
            if success {
                debug!("[Tick {tick}] Player hit with {style} for {damage} damage");
            } else {
                debug!("[Tick {tick}] Player missed with {style}");
            }
        }
    }

    pub fn log_player_spec(&mut self, tick: i32, damage: u32, success: bool, label: String) {
        if self.enabled {
            if success {
                debug!(
                    "[Tick {tick}] Player hit with special attack '{label}' for {damage} damage"
                );
            } else {
                debug!("[Tick {tick}] Player missed with special attack '{label}'");
            }
        }
    }

    pub fn log_player_damage(&mut self, tick: i32, damage: u32, hp: u32) {
        if self.enabled {
            debug!("[Tick {tick}] Player took {damage} damage ({hp} hp remaining)");
        }
    }

    pub fn log_thrall_attack(&mut self, tick: i32, damage: u32) {
        if self.enabled {
            debug!("[Tick {tick}] Thrall hit for {damage} damage");
        }
    }

    pub fn log_monster_attack(
        &mut self,
        monster: &Monster,
        tick: i32,
        damage: u32,
        success: bool,
        style: Option<AttackType>,
    ) {
        let style = if let Some(style) = style {
            style
        } else if monster
            .info
            .attack_styles
            .as_ref()
            .is_some_and(|x| x.len() == 1)
        {
            monster.info.attack_styles.as_ref().unwrap()[0]
        } else {
            AttackType::None
        };
        let name = monster.info.name.as_str();

        if self.enabled {
            if success {
                debug!("[Tick {tick}] {name} hit with {style} for {damage} damage");
            } else {
                debug!("[Tick {tick}] {name} missed with {style}");
            }
        }
    }

    pub fn log_monster_damage(&mut self, tick: i32, damage: u32, hp: u32, name: &str) {
        if self.enabled {
            debug!("[Tick {tick}] {name} took {damage} damage ({hp} hp remaining)");
        }
    }

    pub fn log_gear_switch(&mut self, tick: i32, label: &String) {
        if self.enabled {
            debug!("[Tick {tick}] Player switched to a {label} setup");
        }
    }

    pub fn log_food_eaten(&mut self, tick: i32, heal_amount: u32, hp: u32) {
        if self.enabled {
            debug!("[Tick {tick}] Player ate food for {heal_amount} hp ({hp} hp remaining)");
        }
    }

    pub fn log_hp_regen(&mut self, tick: i32, hp: u32, name: &str) {
        if self.enabled {
            debug!("[Tick {tick}] {name} regenerated 1 hp ({hp} hp remaining)");
        }
    }

    pub fn log_stats_regen(&mut self, tick: i32, name: &str) {
        if self.enabled {
            debug!("[Tick {tick}] {name} regenerated stats by 1");
        }
    }

    pub fn log_monster_death(&mut self, tick: i32, name: &str) {
        if self.enabled {
            debug!("[Tick {tick}] {name} has died.");
        }
    }

    pub fn log_player_death(&mut self, tick: i32) {
        if self.enabled {
            debug!("[Tick {tick}] Player has died, ending the fight");
        }
    }

    pub fn log_monster_effect_damage(&mut self, tick: i32, damage: u32, name: &str) {
        if self.enabled {
            debug!("[Tick {tick}] {name} took {damage} effect damage");
        }
    }

    pub fn log_custom(&mut self, tick: i32, message: &str) {
        if self.enabled {
            debug!("[Tick {tick}] {message}");
        }
    }

    pub fn log_freeze_end(&mut self, tick: i32, name: &str) {
        if self.enabled {
            debug!("[Tick {tick}] {name} is no longer frozen");
        }
    }
}
