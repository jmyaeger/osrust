use crate::combat::simulation::CumulativeResults;
use core::f64;
use plotly::histogram::Bins;
use plotly::layout::{Axis, Layout};
use plotly::{Histogram, Plot, Scatter};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct SimulationStats {
    pub ttk: f64,
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

pub enum TtkUnits {
    Ticks,
    Seconds,
}

pub fn plot_ttk_dist(results: &CumulativeResults, ttk_units: TtkUnits, show: bool) -> Plot {
    let min_ttk_ticks = results.ttks_ticks.iter().min().unwrap_or(&0);
    let max_ttk_ticks = results.ttks_ticks.iter().max().unwrap_or(&0);
    let min_ttk = match ttk_units {
        TtkUnits::Ticks => *min_ttk_ticks as f64,
        TtkUnits::Seconds => *min_ttk_ticks as f64 * 0.6,
    };
    let max_ttk = match ttk_units {
        TtkUnits::Ticks => *max_ttk_ticks as f64,
        TtkUnits::Seconds => *max_ttk_ticks as f64 * 0.6,
    };
    let ttks: Vec<f64> = match ttk_units {
        TtkUnits::Ticks => results.ttks_ticks.iter().map(|&ttk| ttk as f64).collect(),
        TtkUnits::Seconds => results
            .ttks_ticks
            .iter()
            .map(|&ttk| ttk as f64 * 0.6)
            .collect(),
    };
    let bin_size = match ttk_units {
        TtkUnits::Ticks => 5.0,
        TtkUnits::Seconds => 3.0,
    };
    let bins = Bins::new(min_ttk, max_ttk, bin_size);
    let trace = Histogram::new(ttks.clone())
        .name("ttk")
        .x_bins(bins)
        .hist_norm(plotly::histogram::HistNorm::ProbabilityDensity);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let x_label = match ttk_units {
        TtkUnits::Ticks => "TTK (ticks)",
        TtkUnits::Seconds => "TTK (s)",
    };
    let layout = Layout::new()
        .x_axis(Axis::new().title(x_label))
        .y_axis(Axis::new().title("Probability Density"))
        .title("TTK Distribution")
        .template(&*plotly::layout::themes::PLOTLY_DARK)
        .width(1200)
        .height(600);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    plot
}

pub fn plot_ttk_cdf(results: &CumulativeResults, ttk_units: TtkUnits, show: bool) -> Plot {
    let max_ttk_ticks = results.ttks_ticks.iter().copied().max().unwrap_or(0);
    let tick_values = 0..=(max_ttk_ticks * 11 / 10);
    let time_values = match ttk_units {
        TtkUnits::Ticks => tick_values.clone().map(|ttk| ttk as f64).collect(),
        TtkUnits::Seconds => tick_values.clone().map(|ttk| ttk as f64 * 0.6).collect(),
    };

    let mut cdf = Vec::new();
    for time in tick_values {
        let count = results
            .ttks_ticks
            .iter()
            .filter(|&ttk| *ttk <= time)
            .count() as f64;
        cdf.push(count / results.ttks_ticks.len() as f64);
    }
    let trace = Scatter::new(time_values, cdf).mode(plotly::common::Mode::Lines);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let x_label = match ttk_units {
        TtkUnits::Ticks => "TTK (ticks)",
        TtkUnits::Seconds => "TTK (s)",
    };
    let layout = Layout::new()
        .x_axis(Axis::new().title(x_label))
        .y_axis(Axis::new().title("Cumulative Probability"))
        .title("TTK CDF")
        .template(&*plotly::layout::themes::PLOTLY_DARK)
        .width(1200)
        .height(600);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    plot
}
