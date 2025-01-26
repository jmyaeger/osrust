use crate::types::monster::{AttackType, Monster};
use crate::types::player::Player;
use crate::types::{equipment::CombatType, player::SwitchType};
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
                eprintln!("Failed to create log directory: {}", e);
            });

            let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
            let filename = format!("logs/{}_{}.log", name, timestamp);

            WriteLogger::init(
                LevelFilter::Debug,
                Config::default(),
                File::create(filename).unwrap_or_else(|e| {
                    eprintln!("Failed to create log file: {}", e);
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
                player.att_rolls[&player.combat_type()]
            );
            debug!(
                "Player's max hit: {}",
                player.max_hits[&player.combat_type()]
            );
            debug!(
                "Player's max defence rolls\n: {} (Stab), {} (Slash), {} (Crush), {} (Ranged), {} (Magic)\n", 
                player.def_rolls[&CombatType::Stab],
                player.def_rolls[&CombatType::Slash],
                player.def_rolls[&CombatType::Crush],
                player.def_rolls[&CombatType::Ranged],
                player.def_rolls[&CombatType::Magic]
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
                    debug!("{}", prayer);
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
            debug!("Attack: {}", monster.stats.attack);
            debug!("Strength: {}", monster.stats.strength);
            debug!("Ranged: {}", monster.stats.ranged);
            debug!("Defence: {}", monster.stats.defence);
            debug!("Magic: {}", monster.stats.magic);
            debug!("Hitpoints: {}\n", monster.stats.hitpoints);

            debug!(
                "Monster's max attack rolls: {} (Stab), {} (Slash), {} (Crush), {} (Ranged), {} (Magic)\n",
                monster.att_rolls[&CombatType::Stab],
                monster.att_rolls[&CombatType::Slash],
                monster.att_rolls[&CombatType::Crush],
                monster.att_rolls[&CombatType::Ranged],
                monster.att_rolls[&CombatType::Magic]
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
                monster.def_rolls[&CombatType::Stab],
                monster.def_rolls[&CombatType::Slash],
                monster.def_rolls[&CombatType::Crush],
                monster.def_rolls[&CombatType::Light],
                monster.def_rolls[&CombatType::Standard],
                monster.def_rolls[&CombatType::Heavy],
                monster.def_rolls[&CombatType::Magic]
            );
        }
    }

    pub fn log_player_attack(&mut self, tick: i32, damage: u32, success: bool, style: CombatType) {
        if self.enabled {
            if success {
                debug!(
                    "[Tick {}] Player hit with {} for {} damage",
                    tick, style, damage
                );
            } else {
                debug!("[Tick {}] Player missed with {}", tick, style);
            }
        }
    }

    pub fn log_player_damage(&mut self, tick: i32, damage: u32, hp: u32) {
        if self.enabled {
            debug!(
                "[Tick {}] Player took {} damage ({} hp remaining)",
                tick, damage, hp
            );
        }
    }

    pub fn log_monster_attack(
        &mut self,
        tick: i32,
        damage: u32,
        success: bool,
        style: AttackType,
        name: &str,
    ) {
        if self.enabled {
            if success {
                debug!(
                    "[Tick {}] {} hit with {} for {} damage",
                    name, tick, style, damage
                );
            } else {
                debug!("[Tick {}] {} missed with {}", name, tick, style);
            }
        }
    }

    pub fn log_monster_damage(&mut self, tick: i32, damage: u32, hp: u32, name: &str) {
        if self.enabled {
            debug!(
                "[Tick {}] {} took {} damage ({} hp remaining)",
                name, tick, damage, hp
            );
        }
    }

    pub fn log_gear_switch(&mut self, tick: i32, style: SwitchType) {
        if self.enabled {
            debug!("[Tick {}] Player switched to a {} setup", tick, style);
        }
    }

    pub fn log_food_eaten(&mut self, tick: i32, heal_amount: u32, hp: u32) {
        if self.enabled {
            debug!(
                "[Tick {}] Player ate food for {} hp ({} hp remaining)",
                tick, heal_amount, hp
            );
        }
    }

    pub fn log_hp_regen(&mut self, tick: i32, hp: u32, name: &str) {
        if self.enabled {
            debug!(
                "[Tick {}] {} regenerated 1 hp ({} hp remaining)",
                name, tick, hp
            );
        }
    }

    pub fn log_monster_death(&mut self, tick: i32, name: &str) {
        if self.enabled {
            debug!("[Tick {}] {} has died.", name, tick);
        }
    }

    pub fn log_player_death(&mut self, tick: i32) {
        if self.enabled {
            debug!("[Tick {}] Player has died, ending the fight", tick);
        }
    }

    pub fn log_monster_effect_damage(&mut self, tick: i32, damage: u32, name: &str) {
        if self.enabled {
            debug!("[Tick {}] {} took {} effect damage", name, tick, damage);
        }
    }

    pub fn log_custom(&mut self, tick: i32, message: &str) {
        if self.enabled {
            debug!("[Tick {}] {}", tick, message);
        }
    }
}
