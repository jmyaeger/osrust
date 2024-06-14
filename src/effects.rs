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
        }
    }
}

#[derive(Debug)]
pub struct Poison {
    pub tick_counter: Option<i32>,
    pub severity: u32,
}

fn apply_poison(tick_counter: &mut Option<i32>, severity: &mut u32) -> u32 {
    if *severity == 0 {
        *tick_counter = None;
        0
    } else if let Some(mut tick) = tick_counter {
        tick += 1;
        if tick == 30 {
            *tick_counter = Some(0);
            *severity -= 1;
            (*severity + 4) / 5
        } else {
            0
        }
    } else {
        *tick_counter = Some(0);
        *severity -= 1;
        (*severity + 4) / 5
    }
}

fn apply_venom(tick_counter: &mut Option<i32>, damage: &mut u32) -> u32 {
    if let Some(mut tick) = tick_counter {
        tick += 1;
        if tick == 30 {
            *tick_counter = Some(0);
            *damage += 2;
            *damage
        } else {
            0
        }
    } else {
        *tick_counter = Some(0);
        *damage = 6;
        *damage
    }
}

fn apply_burn(tick_counter: &mut Option<i32>, stacks: &mut Vec<u32>) -> u32 {
    let mut damage = 0;
    if let Some(tick) = tick_counter {
        if *tick % 4 == 0 {
            damage += stacks.len() as u32;
            *stacks = stacks.iter().map(|&s| s - 1).collect();
            stacks.retain(|s| *s > 0);
            if stacks.is_empty() {
                *tick_counter = None;
            }
        }
        *tick_counter = Some(tick_counter.unwrap_or(0) + 1);
    } else if !stacks.is_empty() {
        *tick_counter = Some(0);
        damage += stacks.len() as u32;
        *stacks = stacks.iter().map(|&s| s.saturating_sub(1)).collect();
        stacks.retain(|s| *s > 0);
    };
    damage
}
