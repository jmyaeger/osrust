use std::collections::HashMap;

pub trait HitTransformer: Fn(u32) -> HitDistribution {}

#[derive(Debug, Clone)]
pub struct WeightedHit {
    pub probability: f64,
    pub hitsplats: Vec<u32>,
}

impl Default for WeightedHit {
    fn default() -> Self {
        Self {
            probability: 1.0,
            hitsplats: vec![0],
        }
    }
}

impl WeightedHit {
    pub fn new(probability: f64, hitsplats: Vec<u32>) -> Self {
        Self {
            probability,
            hitsplats,
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self::new(self.probability * factor, self.hitsplats.clone())
    }

    pub fn zip(&self, other: &Self) -> Self {
        let mut hitsplats = self.hitsplats.clone();
        hitsplats.extend(other.hitsplats.iter().cloned());

        Self::new(self.probability * other.probability, hitsplats)
    }

    pub fn shift(&self) -> (Self, Self) {
        let head = Self::new(self.probability, vec![self.hitsplats[0]]);
        let tail = Self::new(1.0, self.hitsplats[1..].to_vec());
        (head, tail)
    }

    pub fn transform<F>(&self, t: &F) -> HitDistribution
    where
        F: Fn(u32) -> HitDistribution,
    {
        if self.hitsplats.len() == 1 {
            return t(self.hitsplats[0]).scale_probability(self.probability);
        }

        let (head, tail) = self.shift();
        head.transform(t).zip(&tail.transform(t))
    }

    pub fn get_sum(&self) -> u32 {
        self.hitsplats.iter().sum()
    }

    pub fn get_expected_value(&self) -> f64 {
        self.probability * self.get_sum() as f64
    }

    pub fn get_hash(&self) -> u64 {
        let mut acc = 0;
        for &hitsplat in &self.hitsplats {
            acc <<= 8;
            acc |= hitsplat as u64;
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

    pub fn transform<F>(&self, t: F) -> Self
    where
        F: Fn(u32) -> HitDistribution,
    {
        let mut d = HitDistribution::new(Vec::new());
        for h in &self.hits {
            for transformed in &h.transform(&t).hits {
                d.add_hit(transformed.clone());
            }
        }
        d.flatten()
    }

    pub fn scale_probability(&self, factor: f64) -> Self {
        let hits = self.hits.iter().map(|h| h.scale(factor)).collect();

        Self::new(hits)
    }

    pub fn scale_damage(&self, factor: f64, divisor: i32) -> Self {
        let hits = self
            .hits
            .iter()
            .map(|h| {
                let hitsplats = h
                    .hitsplats
                    .iter()
                    .map(|&s| (s as f64 * factor / divisor as f64) as u32)
                    .collect();
                WeightedHit::new(h.probability, hitsplats)
            })
            .collect();

        Self::new(hits)
    }

    pub fn flatten(&self) -> Self {
        let mut acc: HashMap<u64, f64> = HashMap::new();
        let mut hit_lists: HashMap<u64, Vec<u32>> = HashMap::new();

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

    pub fn cumulative(&self) -> Self {
        let mut acc: HashMap<u32, f64> = HashMap::new();

        for hit in &self.hits {
            let key = hit.get_sum();
            let prev = acc.entry(key).or_insert(0.0);
            *prev += hit.probability;
        }

        let hits = acc
            .into_iter()
            .map(|(dmg, prob)| WeightedHit::new(prob, vec![dmg]))
            .collect();
        Self::new(hits)
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
        let mut d = HitDistribution::new(Vec::new());
        let hit_prob = accuracy / (max - min + 1) as f64;
        for i in min..=max {
            d.add_hit(WeightedHit::new(hit_prob, vec![i]));
        }
        d.add_hit(WeightedHit::new(1.0 - accuracy, vec![0]));
        d
    }

    pub fn single(accuracy: f64, hit: u32) -> HitDistribution {
        let mut d = HitDistribution::new(vec![WeightedHit::new(accuracy, vec![hit])]);
        if accuracy != 1.0 {
            d.add_hit(WeightedHit::new(1.0 - accuracy, vec![0]));
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

    pub fn transform<F>(&self, t: F) -> AttackDistribution
    where
        F: Fn(u32) -> HitDistribution,
    {
        let dists = self.dists.iter().map(|d| d.transform(&t)).collect();
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

    pub fn scale_damage(&self, factor: f64, divisor: i32) -> AttackDistribution {
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

    pub fn as_histogram(&mut self) -> Vec<ChartEntry> {
        let dist = self.get_single_hitsplat();

        let mut hit_map = HashMap::new();
        for h in &dist.hits {
            hit_map.insert(h.get_sum(), h.probability);
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

pub fn flat_limit_transformer(maximum: u32) -> impl Fn(u32) -> HitDistribution {
    move |h| HitDistribution::new(vec![WeightedHit::new(1.0, vec![h.min(maximum)])])
}

pub fn linear_min_transformer(maximum: u32, offset: u32) -> impl Fn(u32) -> HitDistribution {
    move |h| {
        let mut d = HitDistribution::new(Vec::new());
        let prob = 1.0 / (maximum + 1) as f64;
        for i in 0..=maximum {
            d.add_hit(WeightedHit::new(prob, vec![h.min(i + offset)]));
        }
        d.flatten()
    }
}

pub fn capped_reroll_transformer(
    limit: u32,
    roll_max: u32,
    offset: u32,
) -> impl Fn(u32) -> HitDistribution {
    move |h| {
        if h <= limit {
            return HitDistribution::single(1.0, h);
        }

        let mut d = HitDistribution::new(Vec::new());
        let prob = 1.0 / (roll_max + 1) as f64;
        for i in 0..=roll_max {
            d.add_hit(WeightedHit::new(
                prob,
                vec![if h > limit { i + offset } else { h }],
            ));
        }
        d.flatten()
    }
}

pub fn multiply_transformer(
    numerator: u32,
    divisor: u32,
    minimum: u32,
) -> impl Fn(u32) -> HitDistribution {
    move |h| {
        if h == 0 {
            HitDistribution::new(vec![WeightedHit::new(1.0, vec![0])])
        } else {
            HitDistribution::new(vec![WeightedHit::new(
                1.0,
                vec![(numerator * h / divisor).max(minimum)],
            )])
        }
    }
}

pub fn division_transformer(divisor: u32, minimum: u32) -> impl Fn(u32) -> HitDistribution {
    multiply_transformer(1, divisor, minimum)
}

pub fn flat_add_transformer(addend: u32) -> impl Fn(u32) -> HitDistribution {
    move |h| HitDistribution::new(vec![WeightedHit::new(1.0, vec![h + addend])])
}

pub struct ChartEntry {
    pub name: String,
    pub value: f64,
}
