use crate::combat::simulation::CumulativeResults;
use crate::constants::SECONDS_PER_TICK;
use core::f64;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulationStats {
    pub ttk: f64,
    pub ttk_dist: Vec<f64>,
    pub accuracy: f64,
    pub hit_dist: Vec<f64>,
    pub success_rate: f64,
    pub avg_food_eaten: f64,
    pub food_eaten_dist: Vec<f64>,
    pub avg_damage_taken: f64,
    pub avg_leftover_burn: f64,
    pub total_deaths: u32,
}

impl SimulationStats {
    pub fn new(results: &CumulativeResults) -> Self {
        let total_ticks: u64 = results.ttks_ticks.iter().map(|&t| t as u64).sum();
        let total_successful_fights = results.ttks_ticks.len();
        let ttk = (total_ticks as f64 / total_successful_fights as f64) * SECONDS_PER_TICK;

        let ttks_as_u32: Vec<u32> = results.ttks_ticks.iter().map(|&t| t as u32).collect();
        let ttk_dist = calculate_dist(&ttks_as_u32);
        let hit_dist = calculate_dist(&results.hit_amounts);
        let food_eaten_dist = calculate_dist(&results.food_eaten);

        let total_hits: u64 = results.hit_counts.iter().map(|&h| h as u64).sum();
        let total_attempts: u64 = results.hit_attempt_counts.iter().map(|&a| a as u64).sum();
        let accuracy = (total_hits as f64 / total_attempts as f64) * 100.0;

        let total_fights = total_successful_fights + results.player_deaths;
        let success_rate = 1.0 - (results.player_deaths as f64 / total_fights as f64);

        let avg_food_eaten =
            results.food_eaten.iter().sum::<u32>() as f64 / total_successful_fights as f64;
        let avg_damage_taken =
            results.damage_taken.iter().sum::<u32>() as f64 / total_successful_fights as f64;
        let avg_leftover_burn =
            results.leftover_burn.iter().sum::<u32>() as f64 / total_successful_fights as f64;

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

fn calculate_dist(data: &[u32]) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }

    let max_val = *data.iter().max().unwrap() as usize;
    let mut dist = vec![0.0; max_val + 1];

    for &item in data {
        dist[item as usize] += 1.0;
    }

    let total = data.len() as f64;
    for count in dist.iter_mut() {
        *count /= total;
    }

    dist
}
