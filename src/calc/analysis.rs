use crate::combat::simulation::CumulativeResults;
use core::f64;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct SimulationStats {
    pub ttk: f64,
    pub ttk_dist: HashMap<i32, f64>,
    pub accuracy: f64,
    pub hit_dist: HashMap<u32, f64>,
    pub success_rate: f64,
    pub avg_food_eaten: f64,
    pub avg_damage_taken: f64,
    pub avg_leftover_burn: f64,
    pub total_deaths: u32,
}

impl SimulationStats {
    pub fn new(results: &CumulativeResults) -> Self {
        // Calculate average ttk and accuracy
        let ttks_seconds: Vec<f64> = results
            .ttks_ticks
            .iter()
            .map(|&ttk| ttk as f64 * 0.6)
            .collect();
        let ttk = ttks_seconds.iter().sum::<f64>() / results.ttks_ticks.len() as f64;
        let ttk_counts: HashMap<i32, u64> =
            results
                .ttks_ticks
                .iter()
                .fold(HashMap::new(), |mut acc, &value| {
                    *acc.entry(value).or_insert(0) += 1;
                    acc
                });
        let ttk_dist = ttk_counts
            .iter()
            .map(|(&key, &value)| (key, value as f64 / results.ttks_ticks.len() as f64))
            .collect();
        let accuracy = results.hit_counts.iter().sum::<u32>() as f64
            / results.hit_attempt_counts.iter().sum::<u32>() as f64
            * 100.0;

        // Convert hit amount Vecs to a HashMap counting the number of times each amount appears
        let hit_counts: HashMap<u32, u32> =
            results
                .hit_amounts
                .iter()
                .fold(HashMap::new(), |mut acc, &value| {
                    *acc.entry(value).or_insert(0) += 1;
                    acc
                });

        // Convert hit counts into a probability distribution
        let hit_dist = hit_counts
            .iter()
            .map(|(&key, &value)| (key, value as f64 / hit_counts.values().sum::<u32>() as f64))
            .collect();

        // Calculate success rate and average food eaten
        let total_fights = results.ttks_ticks.len() + results.player_deaths;
        let success_rate = 1.0 - results.player_deaths as f64 / total_fights as f64;
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
            avg_damage_taken,
            avg_leftover_burn,
            total_deaths: results.player_deaths as u32,
        }
    }
}
