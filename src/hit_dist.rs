// Port of OSRS Wiki DPS calc's hit distribution code - credit to the wiki team

use std::cmp::min;
use std::collections::HashMap;

pub trait HitTransformer: Fn(Hitsplat) -> HitDistribution {}

impl<F> HitTransformer for F where F: Fn(Hitsplat) -> HitDistribution {}

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
#[derive(Debug, Clone, Copy)]
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

        t(*self)
    }
}

// Allows for cases where there are multiple hits with different probabilities to occur on an attack
#[derive(Debug, Clone)]
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
        if self.hitsplats.len() == 1 {
            return self.hitsplats[0]
                .transform(t, _opts)
                .scale_probability(self.probability);
        }

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
        let mut d = HitDistribution::default();
        for h in &self.hits {
            for transformed in &h.transform(&t, opts).hits {
                d.add_hit(transformed.clone());
            }
        }
        d.flatten()
    }

    pub fn scale_probability(&self, factor: f64) -> Self {
        let hits = self.hits.iter().map(|h| h.scale(factor)).collect();

        Self::new(hits)
    }

    pub fn scale_damage(&self, factor: i32, divisor: i32) -> Self {
        let hits = self
            .hits
            .iter()
            .map(|h| {
                let hitsplats = h
                    .hitsplats
                    .iter()
                    .map(|&s| {
                        Hitsplat::new((s.damage as i32 * factor / divisor) as u32, s.accurate)
                    })
                    .collect();
                WeightedHit::new(h.probability, hitsplats)
            })
            .collect();

        Self::new(hits)
    }

    pub fn flatten(&self) -> Self {
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
        let mut acc: HashMap<(u32, bool), f64> = HashMap::new();

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
        self.hits.iter().map(|h| h.get_sum()).max().unwrap_or(0)
    }

    pub fn linear(accuracy: f64, min: u32, max: u32) -> HitDistribution {
        let mut d = HitDistribution::default();
        let hit_prob = accuracy / (max - min + 1) as f64;
        for i in min..=max {
            d.add_hit(WeightedHit::new(hit_prob, vec![Hitsplat::new(i, true)]));
        }
        d.add_hit(WeightedHit::new(
            1.0 - accuracy,
            vec![Hitsplat::inaccurate()],
        ));
        d
    }

    pub fn single(accuracy: f64, hit: u32) -> HitDistribution {
        let mut d = HitDistribution::new(vec![WeightedHit::new(
            accuracy,
            vec![Hitsplat::new(hit, true)],
        )]);
        if accuracy != 1.0 {
            d.add_hit(WeightedHit::new(
                1.0 - accuracy,
                vec![Hitsplat::inaccurate()],
            ));
        }
        d
    }
}

#[derive(Debug, Clone)]
pub struct AttackDistribution {
    pub dists: Vec<HitDistribution>,
    single_hitsplat: Option<HitDistribution>,
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
        self.single_hitsplat = None;
    }

    pub fn get_single_hitsplat(&mut self) -> &HitDistribution {
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
        let dists = self.dists.iter().map(|d| d.transform(&t, opts)).collect();
        AttackDistribution::new(dists)
    }

    pub fn flatten(&self) -> AttackDistribution {
        let dists = self.dists.iter().map(|d| d.flatten()).collect();
        AttackDistribution::new(dists)
    }

    pub fn scale_probability(&self, factor: f64) -> AttackDistribution {
        let dists = self
            .dists
            .iter()
            .map(|d| d.scale_probability(factor))
            .collect();
        AttackDistribution::new(dists)
    }

    pub fn scale_damage(&self, factor: i32, divisor: i32) -> AttackDistribution {
        let dists = self
            .dists
            .iter()
            .map(|d| d.scale_damage(factor, divisor))
            .collect();
        AttackDistribution::new(dists)
    }

    pub fn get_max(&self) -> u32 {
        self.dists.iter().map(|d| d.get_max()).sum()
    }

    pub fn get_expected_damage(&self) -> f64 {
        self.dists.iter().map(|d| d.expected_hit()).sum()
    }

    pub fn as_histogram(&mut self, hide_misses: bool) -> Vec<ChartEntry> {
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
    move |h| {
        HitDistribution::new(vec![WeightedHit::new(
            1.0,
            vec![Hitsplat::new(h.damage.min(maximum), h.accurate)],
        )])
    }
}

pub fn linear_min_transformer(maximum: u32, offset: u32) -> impl HitTransformer {
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
    move |h| {
        if h.damage <= limit {
            return HitDistribution::new(vec![WeightedHit::new(1.0, vec![h])]);
        }

        let mut d = HitDistribution::default();
        let prob = 1.0 / (roll_max + 1) as f64;
        for i in 0..=roll_max {
            d.add_hit(WeightedHit::new(
                prob,
                vec![if h.damage > limit {
                    Hitsplat::new(i + offset, h.accurate)
                } else {
                    h
                }],
            ));
        }
        d.flatten()
    }
}

pub fn multiply_transformer(numerator: u32, divisor: u32, minimum: u32) -> impl HitTransformer {
    move |h| {
        HitDistribution::new(vec![WeightedHit::new(
            1.0,
            vec![Hitsplat::new(
                min(h.damage, (numerator * h.damage / divisor).max(minimum)),
                h.accurate,
            )],
        )])
    }
}

pub fn division_transformer(divisor: u32, minimum: u32) -> impl HitTransformer {
    multiply_transformer(1, divisor, minimum)
}

pub fn flat_add_transformer(addend: i32, minimum: i32) -> impl HitTransformer {
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

pub struct ChartEntry {
    pub name: String,
    pub value: f64,
}
