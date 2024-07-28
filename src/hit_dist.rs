// Adapted from the wiki DPS calc - credit to the wiki team

use std::cmp::min;
use std::collections::HashMap;

use crate::utils::Fraction;

pub trait HitTransformer: Fn(&Hitsplat) -> HitDistribution {}

impl<F> HitTransformer for F where F: Fn(&Hitsplat) -> HitDistribution {}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProbabilisticDelay {
    pub probability: f64,
    pub delay: u32,
}

impl ProbabilisticDelay {
    pub fn new(probability: f64, delay: u32) -> Self {
        Self { probability, delay }
    }
}

pub type WeaponDelayProvider = dyn Fn(&WeightedHit) -> Vec<ProbabilisticDelay>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DelayedHit {
    pub wh: WeightedHit,
    pub delay: u32,
}

impl DelayedHit {
    pub fn new(wh: WeightedHit, delay: u32) -> Self {
        Self { wh, delay }
    }
}

// Options for distribution transforms (only one option currently)
#[derive(Debug, Clone)]
pub struct TransformOpts {
    // Determines whether to apply transform to inaccurate hits (0s/splashes)
    pub transform_inaccurate: bool,
}

impl Default for TransformOpts {
    fn default() -> Self {
        Self {
            transform_inaccurate: true,
        }
    }
}

// Single hitsplat with damage dealt and whether it passed the accuracy check
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hitsplat {
    pub damage: u32,
    pub accurate: bool,
}

impl Default for Hitsplat {
    fn default() -> Self {
        Self {
            damage: 0,
            accurate: true,
        }
    }
}

impl Hitsplat {
    pub fn new(damage: u32, accurate: bool) -> Self {
        Self { damage, accurate }
    }

    pub fn inaccurate() -> Self {
        // Construct an inaccurate Hitsplat
        Self {
            damage: 0,
            accurate: false,
        }
    }

    pub fn transform<F>(&self, t: &F, opts: &TransformOpts) -> HitDistribution
    where
        F: HitTransformer,
    {
        // Apply a generic transform function to the hitsplat and return a HitDistribution

        // Don't apply the transform if the hitsplat is inaccurate and the transform option is disabled
        if !self.accurate && !opts.transform_inaccurate {
            return HitDistribution::new(vec![WeightedHit::new(1.0, vec![*self])]);
        }

        t(self)
    }
}

// One hit with one or more total hitsplats and the overall probability for that hit
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedHit {
    pub probability: f64,         // The probability that this hit will occur
    pub hitsplats: Vec<Hitsplat>, // Allows for multi-hitsplat attacks
}

impl Default for WeightedHit {
    fn default() -> Self {
        Self {
            probability: 1.0,
            hitsplats: vec![Hitsplat::default()],
        }
    }
}

impl WeightedHit {
    pub fn new(probability: f64, hitsplats: Vec<Hitsplat>) -> Self {
        Self {
            probability,
            hitsplats,
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        // Scale a hit's proability by a factor
        Self::new(self.probability * factor, self.hitsplats.clone())
    }

    pub fn zip(&self, other: &Self) -> Self {
        // Zip two hits together into a single WeightedHit, combining the probabilities
        let mut hitsplats = self.hitsplats.clone();
        hitsplats.extend(other.hitsplats.iter().cloned());

        Self::new(self.probability * other.probability, hitsplats)
    }

    pub fn shift(&self) -> (Self, Self) {
        // Split a WeightedHit into a single hitsplat head and a tail containing all other hitsplats
        let head = Self::new(self.probability, vec![self.hitsplats[0]]);
        let tail = Self::new(1.0, self.hitsplats[1..].to_vec());
        (head, tail)
    }

    pub fn transform<F>(&self, t: &F, _opts: &TransformOpts) -> HitDistribution
    where
        F: HitTransformer,
    {
        // Apply a transform function to each hitsplat in the WeightedHit and return a HitDistribution
        if self.hitsplats.len() == 1 {
            return self.hitsplats[0]
                .transform(t, _opts)
                .scale_probability(self.probability);
        }

        // Recursively zip first hitsplat with remaining hitsplats
        let (head, tail) = self.shift();
        head.transform(t, _opts).zip(&tail.transform(t, _opts))
    }

    pub fn any_accurate(&self) -> bool {
        self.hitsplats.iter().any(|h| h.accurate)
    }

    pub fn get_sum(&self) -> u32 {
        self.hitsplats.iter().map(|h| h.damage).sum()
    }

    pub fn get_expected_value(&self) -> f64 {
        self.probability * self.get_sum() as f64
    }

    pub fn get_hash(&self) -> u64 {
        let mut acc = 0;
        for &hitsplat in &self.hitsplats {
            acc <<= 8;
            acc |= hitsplat.damage as u64;
            acc <<= 1;
            acc |= if hitsplat.accurate { 1 } else { 0 };
        }
        acc
    }
}

// Distribution of weighted hits
#[derive(Debug, Clone, Default)]
pub struct HitDistribution {
    pub hits: Vec<WeightedHit>,
}

impl HitDistribution {
    pub fn new(hits: Vec<WeightedHit>) -> Self {
        Self { hits }
    }

    pub fn add_hit(&mut self, w: WeightedHit) {
        self.hits.push(w);
    }

    pub fn zip(&self, other: &Self) -> Self {
        // Zip two HitDistributions together
        let hits = self
            .hits
            .iter()
            .flat_map(|hit1| other.hits.iter().map(move |hit2| hit1.zip(hit2)))
            .collect();

        Self::new(hits)
    }

    pub fn transform<F>(&self, t: &F, opts: &TransformOpts) -> Self
    where
        F: HitTransformer,
    {
        // Apply a transform function to each hit in the distribution
        let mut d = HitDistribution::default();
        for h in &self.hits {
            for transformed in &h.transform(&t, opts).hits {
                d.add_hit(transformed.clone());
            }
        }
        d.flatten()
    }

    pub fn scale_probability(&self, factor: f64) -> Self {
        // Scale the probability of each hit in the distribution by a factor
        let hits = self.hits.iter().map(|h| h.scale(factor)).collect();

        Self::new(hits)
    }

    pub fn scale_damage(&self, factor: Fraction) -> Self {
        // Scale the damage of the entire distribution by a fractional factor
        let hits = self
            .hits
            .iter()
            .map(|h| {
                let hitsplats = h
                    .hitsplats
                    .iter()
                    .map(|&s| {
                        Hitsplat::new((factor.multiply_to_int(s.damage as i32)) as u32, s.accurate)
                    })
                    .collect();
                WeightedHit::new(h.probability, hitsplats)
            })
            .collect();

        Self::new(hits)
    }

    pub fn flatten(&self) -> Self {
        // Combine all hits with the same hash into single weighted hits with summed probability
        let mut acc: HashMap<u64, f64> = HashMap::new();
        let mut hit_lists: HashMap<u64, Vec<Hitsplat>> = HashMap::new();

        for hit in &self.hits {
            let hash = hit.get_hash();
            let prev = acc.entry(hash).or_insert(0.0);
            *prev += hit.probability;
            hit_lists
                .entry(hash)
                .or_insert_with(|| hit.hitsplats.clone());
        }

        let hits = acc
            .into_iter()
            .map(|(hash, prob)| WeightedHit::new(prob, hit_lists[&hash].clone()))
            .collect();

        Self::new(hits)
    }

    pub fn cumulative(&self) -> HitDistribution {
        // Convert multi-hits into a single cumulative damage total
        let mut acc: HashMap<(u32, bool), f64> = HashMap::new();

        // if 1 hitsplat is accurate, treat the whole hit as accurate
        for hit in &self.hits {
            let key = (hit.get_sum(), hit.any_accurate());
            *acc.entry(key).or_insert(0.0) += hit.probability;
        }

        HitDistribution::new(
            acc.into_iter()
                .map(|((damage, accurate), probability)| {
                    WeightedHit::new(probability, vec![Hitsplat { damage, accurate }])
                })
                .collect(),
        )
    }

    pub fn expected_hit(&self) -> f64 {
        self.hits.iter().map(|h| h.get_expected_value()).sum()
    }

    pub fn size(&self) -> usize {
        self.hits.len()
    }

    pub fn get_max(&self) -> u32 {
        // Get the max hit in the distribution, combining multi-hits
        self.hits.iter().map(|h| h.get_sum()).max().unwrap_or(0)
    }

    pub fn with_probabilistic_delays(
        &self,
        delay_provider: &WeaponDelayProvider,
    ) -> Vec<DelayedHit> {
        let mut hits: Vec<DelayedHit> = Vec::new();

        for wh in &self.hits {
            let delays = delay_provider(wh);
            for ProbabilisticDelay { probability, delay } in delays {
                hits.push(DelayedHit::new(
                    WeightedHit::new(
                        wh.probability * probability,
                        vec![Hitsplat::new(wh.get_sum(), wh.any_accurate())],
                    ),
                    delay,
                ));
            }
        }

        // Dedupe the results and merge entries
        let mut acc: HashMap<u64, f64> = HashMap::new();
        for DelayedHit { wh, delay } in hits {
            let key = (wh.get_sum() as u64 & 0xFFFFFF) | ((delay as u64) << 24);
            *acc.entry(key).or_default() += wh.probability;
        }

        acc.into_iter()
            .map(|(key, prob)| {
                let delay = (key & 0x8F000000) >> 24;
                let dmg = key & 0xFFFFFF;
                DelayedHit::new(
                    WeightedHit::new(prob, vec![Hitsplat::new(dmg as u32, true)]),
                    delay as u32,
                )
            })
            .collect()
    }

    pub fn linear(accuracy: f64, min: u32, max: u32) -> HitDistribution {
        // Create a linear hit distribution between two bounds with equal probabilities for all hits
        let mut d = HitDistribution::default();
        let hit_prob = accuracy / (max - min + 1) as f64;
        for i in min..=max {
            d.add_hit(WeightedHit::new(hit_prob, vec![Hitsplat::new(i, true)]));
        }

        // Add an inaccurate hit if the accuracy is not 1.0
        d.add_hit(WeightedHit::new(
            1.0 - accuracy,
            vec![Hitsplat::inaccurate()],
        ));
        d
    }

    pub fn single(accuracy: f64, hitsplats: Vec<Hitsplat>) -> HitDistribution {
        // Create a distribution with one possible hit
        let mut d = HitDistribution::new(vec![WeightedHit::new(accuracy, hitsplats)]);

        // Add an inaccurate hit if the accuracy is not 1.0
        if accuracy != 1.0 {
            d.add_hit(WeightedHit::new(
                1.0 - accuracy,
                vec![Hitsplat::inaccurate()],
            ));
        }
        d
    }
}

// Distibution for a single attack, allowing for multiple hits with their own hit distributions
#[derive(Debug, Clone, Default)]
pub struct AttackDistribution {
    pub dists: Vec<HitDistribution>,
    single_hitsplat: Option<HitDistribution>, // property accessed through getter method
}

impl AttackDistribution {
    pub fn new(dists: Vec<HitDistribution>) -> Self {
        AttackDistribution {
            dists,
            single_hitsplat: None,
        }
    }

    pub fn add_dist(&mut self, d: HitDistribution) {
        self.dists.push(d);

        // Reset single hitsplat so it gets recalculated when needed
        self.single_hitsplat = None;
    }

    pub fn get_single_hitsplat(&mut self) -> &HitDistribution {
        // Zips together all hit distributions and returns one hit distribution with cumulative hitsplats
        if self.single_hitsplat.is_none() {
            let mut dist = self.dists[0].clone();
            for d in &self.dists[1..] {
                dist = dist.zip(d);
            }
            self.single_hitsplat = Some(dist.cumulative());
        }
        self.single_hitsplat.as_ref().unwrap()
    }

    pub fn transform<F>(&self, t: &F, opts: &TransformOpts) -> AttackDistribution
    where
        F: HitTransformer,
    {
        // Apply a transform to all hit distributions
        let dists = self.dists.iter().map(|d| d.transform(&t, opts)).collect();
        AttackDistribution::new(dists)
    }

    pub fn flatten(&self) -> AttackDistribution {
        // Flatten all hit distributions
        let dists = self.dists.iter().map(|d| d.flatten()).collect();
        AttackDistribution::new(dists)
    }

    pub fn scale_probability(&self, factor: f64) -> AttackDistribution {
        // Scale probabilities of all hit distributions by a factor
        let dists = self
            .dists
            .iter()
            .map(|d| d.scale_probability(factor))
            .collect();
        AttackDistribution::new(dists)
    }

    pub fn scale_damage(&self, factor: Fraction) -> AttackDistribution {
        // Scale damage of all hit distributions by a fractional factor
        let dists = self.dists.iter().map(|d| d.scale_damage(factor)).collect();
        AttackDistribution::new(dists)
    }

    pub fn get_max(&self) -> u32 {
        self.dists.iter().map(|d| d.get_max()).sum()
    }

    pub fn get_expected_damage(&self) -> f64 {
        self.dists.iter().map(|d| d.expected_hit()).sum()
    }

    pub fn as_histogram(&mut self, hide_misses: bool) -> Vec<ChartEntry> {
        // Not really used now, but may be useful when a GUI is added
        let dist = self.get_single_hitsplat();

        let mut hit_map = HashMap::new();
        for h in &dist.hits {
            if !hide_misses || h.any_accurate() {
                hit_map.insert(
                    h.get_sum(),
                    hit_map.get(&h.get_sum()).unwrap_or(&0.0) + h.probability,
                );
            }
        }

        let mut ret = Vec::new();
        for i in 0..=dist.get_max() {
            let prob = hit_map.get(&i).copied().unwrap_or(0.0);
            ret.push(ChartEntry {
                name: i.to_string(),
                value: prob,
            });
        }

        ret
    }
}

pub fn flat_limit_transformer(maximum: u32) -> impl HitTransformer {
    // Hard cap the damage to a maximum value
    move |h| {
        HitDistribution::new(vec![WeightedHit::new(
            1.0,
            vec![Hitsplat::new(h.damage.min(maximum), h.accurate)],
        )])
    }
}

pub fn linear_min_transformer(maximum: u32, offset: u32) -> impl HitTransformer {
    // Roll damage within a range and take the minimum of that damage and the original damage roll
    move |h| {
        let mut d = HitDistribution::default();
        let prob = 1.0 / (maximum + 1) as f64;
        for i in 0..=maximum {
            d.add_hit(WeightedHit::new(
                prob,
                vec![Hitsplat::new(h.damage.min(i + offset), h.accurate)],
            ));
        }
        d.flatten()
    }
}

pub fn capped_reroll_transformer(limit: u32, roll_max: u32, offset: u32) -> impl HitTransformer {
    // Reroll damage within a specified range if it exceeds the limit value
    move |h| {
        if h.damage <= limit {
            return HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])]);
        }

        let mut d = HitDistribution::default();
        let prob = 1.0 / (roll_max + 1) as f64;
        for i in 0..=roll_max {
            d.add_hit(WeightedHit::new(
                prob,
                vec![if h.damage > limit {
                    Hitsplat::new(i + offset, h.accurate)
                } else {
                    *h
                }],
            ));
        }
        d.flatten()
    }
}

pub fn multiply_transformer(factor: Fraction, minimum: u32) -> impl HitTransformer {
    move |h| {
        // Multiply each hit's damage by a fractional factor and clamp it to a minimum value
        HitDistribution::new(vec![WeightedHit::new(
            1.0,
            vec![Hitsplat::new(
                min(h.damage, factor.multiply_to_int(h.damage)).max(minimum),
                h.accurate,
            )],
        )])
    }
}

pub fn division_transformer(divisor: u32, minimum: u32) -> impl HitTransformer {
    // Divide each hit's damage by a divisor and clamp it to a minimum value
    multiply_transformer(Fraction::new(1, divisor as i32), minimum)
}

pub fn flat_add_transformer(addend: i32, minimum: i32) -> impl HitTransformer {
    // Add a value to each hit's damage and clamp it to a minimum value
    move |h| {
        HitDistribution::new(vec![WeightedHit::new(
            1.0,
            vec![Hitsplat::new(
                (h.damage as i32 + addend).max(minimum) as u32,
                h.accurate,
            )],
        )])
    }
}

// Struct for storing histogram data (to be used in the future)
#[derive(Debug, PartialEq, Clone)]
pub struct ChartEntry {
    pub name: String,
    pub value: f64,
}
