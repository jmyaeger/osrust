use crate::combat::{FightResult, FightVars, Simulation};
use crate::limiters::Limiter;
use crate::monster::{AttackType, Monster};
use crate::player::Player;
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GraardorMethod {
    DoorAltar,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GraardorConfig {
    pub method: GraardorMethod,
    pub eat_hp: u32,
    pub heal_amount: u32,
}

#[derive(Clone)]
pub struct GraardorFight {
    pub player: Player,
    pub graardor: Monster,
    pub melee_minion: Monster,
    pub ranged_minion: Monster,
    pub mage_minion: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
    pub config: GraardorConfig,
}

impl GraardorFight {
    pub fn new(player: Player, config: GraardorConfig) -> GraardorFight {
        let graardor = Monster::new("General Graardor", None).unwrap();
        let melee_minion = Monster::new("Sergeant Strongstack", None).unwrap();
        let ranged_minion = Monster::new("Sergeant Grimspike", None).unwrap();
        let mage_minion = Monster::new("Sergeant Steelwill", None).unwrap();
        let limiter = crate::combat::assign_limiter(&player, &graardor);
        let rng = rand::thread_rng();
        GraardorFight {
            player,
            graardor,
            melee_minion,
            ranged_minion,
            mage_minion,
            limiter,
            rng,
            config,
        }
    }

    fn simulate_door_altar_fight(&mut self) -> FightResult {
        let mut vars = FightVars::new();
        let mut mage_attack_tick = 1;
        let mut melee_attack_tick = 5;
        let player_attack = self.player.attack;

        while self.graardor.live_stats.hitpoints > 0 {
            if vars.tick_counter == vars.attack_tick {
                // Process player attack
                let hit = player_attack(
                    &mut self.player,
                    &mut self.graardor,
                    &mut self.rng,
                    &self.limiter,
                );
                self.player.boosts.first_attack = false;
                self.graardor.take_damage(hit.damage);
                vars.hit_attempts += 1;
                vars.hit_count += if hit.success { 1 } else { 0 };
                vars.hit_amounts.push(hit.damage);
                vars.attack_tick += self.player.gear.weapon.speed;
            }

            // Process effects and apply damage
            let mut effect_damage = 0;
            for effect in &mut self.graardor.active_effects {
                effect_damage += effect.apply();
            }

            self.graardor.take_damage(effect_damage);
            self.graardor.clear_inactive_effects();

            // Process mage minion attack
            if vars.tick_counter == mage_attack_tick {
                let hit = self.mage_minion.attack(
                    &mut self.player,
                    Some(AttackType::Magic),
                    &mut self.rng,
                );
                self.player.take_damage(hit.damage);
                if vars.tick_counter == 6 {
                    mage_attack_tick += 7;
                } else {
                    mage_attack_tick += 5;
                }
            }

            // Process melee minion attack
            if vars.tick_counter == melee_attack_tick {
                let hit = self.melee_minion.attack(
                    &mut self.player,
                    Some(AttackType::Crush),
                    &mut self.rng,
                );
                self.player.take_damage(hit.damage);
                if vars.tick_counter == 5 {
                    melee_attack_tick += 22;
                } else {
                    melee_attack_tick += 12;
                }
            }

            // Eat if below the provided threshold
            // TODO: only allow this on hits where it wouldn't break the cycle
            if self.player.live_stats.hitpoints < self.config.eat_hp {
                self.player.heal(self.config.heal_amount, None);
            }

            // Increment tick counter
            vars.tick_counter += 1;
        }

        let ttk = vars.tick_counter as f64 * 0.6;

        FightResult {
            ttk,
            hit_attempts: vars.hit_attempts,
            hit_count: vars.hit_count,
            hit_amounts: vars.hit_amounts,
        }
    }
}

impl Simulation for GraardorFight {
    fn simulate(&mut self) -> FightResult {
        match self.config.method {
            GraardorMethod::DoorAltar => self.simulate_door_altar_fight(),
        }
    }

    fn is_immune(&self) -> bool {
        self.graardor.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.graardor
    }

    fn set_attack_function(&mut self) {
        self.player.attack = crate::attacks::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_live_stats();
        self.graardor.reset();
        self.melee_minion.reset();
        self.ranged_minion.reset();
        self.mage_minion.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment::CombatStyle;
    use crate::player::Player;
    use crate::potions::Potion;
    use crate::prayers::{Prayer, PrayerBoost};
    use crate::rolls::calc_player_ranged_rolls;

    #[test]
    fn test_simulate_door_altar_fight() {
        let mut player = Player::default();
        player.prayers.add(PrayerBoost::new(Prayer::Rigour));
        player.add_potion(Potion::Ranging);

        player.equip("Bow of faerdhinen (c)", None);
        player.equip("Crystal helm", Some("Active"));
        player.equip("Crystal body", Some("Active"));
        player.equip("Crystal legs", Some("Active"));
        player.equip("Zaryte vambraces", None);
        player.equip("Dizana's quiver", Some("Uncharged"));
        player.equip("Necklace of anguish", None);
        player.equip("Pegasian boots", None);
        player.equip("Ring of suffering (i)", Some("Uncharged"));
        player.equip("Rada's blessing 4", None);
        player.update_bonuses();
        player.set_active_style(CombatStyle::Rapid);

        calc_player_ranged_rolls(
            &mut player,
            &Monster::new("General Graardor", None).unwrap(),
        );

        let fight_config = GraardorConfig {
            method: GraardorMethod::DoorAltar,
            eat_hp: 30,
            heal_amount: 22,
        };

        let mut fight = GraardorFight::new(player, fight_config);

        let result = fight.simulate();

        assert!(result.ttk > 0.0);
    }
}
