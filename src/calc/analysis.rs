use crate::combat::simulation::CumulativeResults;
use crate::constants::SECONDS_PER_TICK;
use core::f64;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone)]
pub struct SimulationStats {
    pub ttk: f64,
    pub ttk_dist: HashMap<i32, f64>,
    pub accuracy: f64,
    pub hit_dist: HashMap<u32, f64>,
    pub success_rate: f64,
    pub avg_food_eaten: f64,
    pub food_eaten_dist: HashMap<u32, f64>,
    pub avg_damage_taken: f64,
    pub avg_leftover_burn: f64,
    pub total_deaths: u32,
}

impl SimulationStats {
    pub fn new(results: &CumulativeResults) -> Self {
        let total_ticks: u64 = results.ttks_ticks.iter().map(|&t| t as u64).sum();
        let total_successful_fights = results.ttks_ticks.len();
        let ttk = (total_ticks as f64 / total_successful_fights as f64) * SECONDS_PER_TICK;

        let ttk_dist = calculate_dist(&results.ttks_ticks);
        let hit_dist = calculate_dist(&results.hit_amounts);
        let food_eaten_dist = calculate_dist(&results.food_eaten);

        let total_hits: u64 = results.hit_counts.iter().map(|&h| h as u64).sum();
        let total_attempts: u64 = results.hit_attempt_counts.iter().map(|&a| a as u64).sum();
        let accuracy = (total_hits as f64 / total_attempts as f64) * 100.0;

        let total_fights = total_successful_fights + results.player_deaths;
        let success_rate = 1.0 - (results.player_deaths as f64 / total_fights as f64);

        let avg_food_eaten = results.food_eaten.iter().sum::<u32>() as f64 / total_fights as f64;
        let avg_damage_taken =
            results.damage_taken.iter().sum::<u32>() as f64 / total_fights as f64;
        let avg_leftover_burn =
            results.leftover_burn.iter().sum::<u32>() as f64 / total_fights as f64;

        Self {
            ttk,
            ttk_dist,
            accuracy,
            hit_dist,
            success_rate,
            avg_food_eaten,
            food_eaten_dist,
            avg_damage_taken,
            avg_leftover_burn,
            total_deaths: results.player_deaths as u32,
        }
    }
}

fn calculate_dist<T: Eq + Hash + Copy>(data: &[T]) -> HashMap<T, f64> {
    if data.is_empty() {
        return HashMap::new();
    }

    let mut map = HashMap::with_capacity(data.len());

    for &item in data {
        *map.entry(item).or_insert(0.0) += 1.0;
    }

    let total = data.len() as f64;
    for count in map.values_mut() {
        *count /= total;
    }

    map
}
