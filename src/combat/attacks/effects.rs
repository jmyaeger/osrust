// Covers any type of effect that gets applied over time
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CombatEffect {
    Poison {
        tick_counter: Option<i32>,
        severity: u32,
    },
    Venom {
        tick_counter: Option<i32>,
        damage: u32,
    },
    Burn {
        tick_counter: Option<i32>,
        stacks: Vec<u32>,
    },
    DelayedAttack {
        tick_delay: Option<i32>,
        damage: u32,
    },
    DelayedHeal {
        tick_delay: i32,
        tick_counter: Option<i32>,
        num_heals: u32,
        heal: u32,
    },
    DamageOverTime {
        tick_counter: Option<i32>,
        tick_interval: i32,
        damage_per_hit: u32,
        total_hits: i32,
        apply_on_hit: bool,
    },
}

impl CombatEffect {
    pub fn apply(&mut self) -> u32 {
        match self {
            Self::Poison {
                ref mut tick_counter,
                ref mut severity,
            } => apply_poison(tick_counter, severity),
            Self::Venom {
                ref mut tick_counter,
                ref mut damage,
            } => apply_venom(tick_counter, damage),
            Self::Burn {
                ref mut tick_counter,
                ref mut stacks,
            } => apply_burn(tick_counter, stacks),
            Self::DelayedAttack { tick_delay, damage } => apply_delayed_attack(tick_delay, damage),
            Self::DelayedHeal {
                tick_delay,
                tick_counter,
                num_heals,
                heal,
            } => apply_delayed_heal(*tick_delay, tick_counter, num_heals, heal),
            Self::DamageOverTime {
                tick_counter,
                tick_interval,
                damage_per_hit,
                total_hits,
                apply_on_hit,
            } => apply_damage_over_time(
                tick_counter,
                tick_interval,
                damage_per_hit,
                total_hits,
                apply_on_hit,
            ),
        }
    }
}

#[derive(Debug)]
pub struct Poison {
    pub tick_counter: Option<i32>,
    pub severity: u32,
}

fn apply_poison(tick_counter: &mut Option<i32>, severity: &mut u32) -> u32 {
    // If the severity is 0, the poison effect has worn off and the tick counter can be reset
    if *severity == 0 {
        *tick_counter = None;
        0
    } else if let Some(mut tick) = tick_counter {
        // Otherwise, increment tick counter, apply poison damage every 30 ticks, and increase severity
        tick += 1;
        if tick == 30 {
            *tick_counter = Some(0);
            *severity -= 1;
            (*severity + 4) / 5
        } else {
            0
        }
    } else {
        // If severity is nonzero and tick counter is None, poison has just been inflicted
        *tick_counter = Some(0);
        *severity -= 1;
        (*severity + 4) / 5
    }
}

fn apply_venom(tick_counter: &mut Option<i32>, damage: &mut u32) -> u32 {
    if let Some(mut tick) = tick_counter {
        // Increment tick counter, apply venom damage every 30 ticks, and increase damage
        tick += 1;
        if tick == 30 {
            *tick_counter = Some(0);
            *damage += 2;
            *damage
        } else {
            0
        }
    } else {
        // If tick counter is None, venom has just been inflicted
        *tick_counter = Some(0);
        *damage = 6;
        *damage
    }
}

fn apply_burn(tick_counter: &mut Option<i32>, stacks: &mut Vec<u32>) -> u32 {
    // Default to a damage of 0
    let mut damage = 0;

    if let Some(tick) = tick_counter {
        // Apply 1 damage for each active stack every 4 ticks
        if *tick % 4 == 0 {
            damage += stacks.len() as u32;

            // Decrease each stack by 1 and remove stacks with 0 values
            *stacks = stacks.iter().map(|&s| s - 1).collect();
            stacks.retain(|s| *s > 0);

            // Burn effect has ended if there are no stacks
            if stacks.is_empty() {
                *tick_counter = None;
                return damage;
            }
        }
        *tick_counter = Some(tick_counter.unwrap_or(0) + 1);
    } else if !stacks.is_empty() {
        // If tick counter is None, burn has just been inflicted
        *tick_counter = Some(0);

        // Apply damage on the first active tick of the effect
        damage += stacks.len() as u32;

        // Decrease each stack by 1 and remove stacks with 0 values
        *stacks = stacks.iter().map(|&s| s.saturating_sub(1)).collect();
        stacks.retain(|s| *s > 0);
    };
    damage
}

fn apply_delayed_attack(tick_delay: &mut Option<i32>, damage: &mut u32) -> u32 {
    if let Some(delay) = tick_delay {
        if *delay == 1 {
            *tick_delay = None;
            *damage
        } else {
            *delay -= 1;
            0
        }
    } else {
        0
    }
}

fn apply_delayed_heal(
    tick_delay: i32,
    tick_counter: &mut Option<i32>,
    num_heals: &mut u32,
    heal: &mut u32,
) -> u32 {
    // Effect is only active when tick counter is Some
    if let Some(counter) = tick_counter {
        if *counter == 1 {
            if *num_heals == 1 {
                // Set tick counter to None to indicate all heals are done
                *tick_counter = None;
            } else {
                // Reset tick counter to max delay and decrement the number of remaining heals
                *tick_counter = Some(tick_delay);
                *num_heals = num_heals.saturating_sub(1);
            }
            *heal
        } else {
            *counter -= 1;
            0
        }
    } else {
        0
    }
}

fn apply_damage_over_time(
    tick_counter: &mut Option<i32>,
    tick_interval: &mut i32,
    damage_per_hit: &mut u32,
    total_hits: &mut i32,
    apply_on_hit: &mut bool,
) -> u32 {
    if let Some(tick) = tick_counter {
        // Increment tick counter
        *tick += 1;

        if *tick == *tick_interval {
            // Reset tick counter
            *tick = 0;

            // Decrement total hits left
            *total_hits -= 1;

            if *total_hits == 0 {
                // If total hits is 0, damage over time effect has ended
                *tick_counter = None;
            }

            *damage_per_hit
        } else {
            0
        }
    } else {
        // If tick counter is None, damage over time has just been inflicted
        *tick_counter = Some(0);

        // Only apply damage on the first tick if specified
        if *apply_on_hit {
            *total_hits -= 1;

            // Shouldn't need this check but just in case
            if *total_hits == 0 {
                *tick_counter = None;
            }

            *damage_per_hit
        } else {
            0
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::monster::Monster;
//     use rand::Rng;

//     #[test]
//     fn test_atlatl_burn() {
//         let mut rng = rand::thread_rng();
//         let n = 1000000000;
//         let mut dummy_monster = Monster::new("Nex", None).unwrap();
//         let mut damage = 0;

//         for i in 0..n {
//             if i % 3 == 0 && rng.gen_range(0..5) == 0 && rng.gen::<f32>() < 0.75 {
//                 dummy_monster.add_burn_stack(10);
//             }

//             for effect in &mut dummy_monster.active_effects {
//                 damage += effect.apply();
//             }

//             dummy_monster.clear_inactive_effects();
//         }

//         let dps = damage as f32 / (n as f32 * 0.6);
//         println!("Burn DPS: {:.4}", dps);

//         assert!(dps > 0.0);
//     }
// }
