use crate::combat::attacks::effects::CombatEffect;
use crate::types::player::Player;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Foods {
    Anchovies,
    Sardine,
    Trout,
    Cod,
    UgthankiKebab,
    Kebab,
    Stew,
    Shrimps,
    Chicken,
    Meat,
    Bread,
    Herring,
    Mackerel,
    Pike,
    Peach,
    Salmon,
    Tuna,
    JugOfWine,
    Cake,
    MeatPie,
    Lobster,
    Bass,
    PlainPizza,
    Swordfish,
    ApplePie,
    ChocolateCake,
    SnowyKnight,
    PotatoWithCheese,
    Monkfish,
    Kyatt,
    AnchovyPizza,
    Karambwan,
    Curry,
    PyreFox,
    GuthixRest,
    Shark,
    SeaTurtle,
    SunlightAntelope,
    PineapplePizza,
    DragonfruitPie,
    SummerPie,
    WildPie,
    MantaRay,
    TunaPotato,
    DarkCrab,
    Anglerfish,
    DashingKebbit,
    MoonlightAntelope,
    BasketOfStrawberries,
    SaradominBrew,
    Paddlefish,
    CorruptedPaddlefish,
    WildKebbit,
    Larupia,
    BarbTailedKebbit,
    Graahk,
    PurpleSweets,
    RedberryPie,
    GardenPie,
    FishPie,
    BotanicalPie,
    MushroomPie,
    AdmiralPie,
    ToadCrunchies,
    SpicyCrunchies,
    WormCrunchies,
    ChocchipCrunchies,
    MeatPizza,
    FruitBatta,
    ToadBatta,
    WormBatta,
    VegetableBatta,
    CheeseTomBatta,
    WormHole,
    VegBall,
    ChocolateBomb,
    TangledToadsLegs,
    MushroomPotato,
    XericsAidMinus,
    XericsAid,
    XericsAidPlus,
    Nectar,
    Ambrosia,
    SilkDressing,
    MossLizard,
    Bream,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum FoodType {
    #[default]
    Normal,
    Combo,
    Potion,
    MultiBite,
    DelayedHeal,
}

#[derive(Debug, Clone, Default)]
pub struct FoodProperties {
    pub total_bites: Option<u32>,
    pub remaining_bites: Option<u32>,
    pub bite_heal_amount: Option<u32>,
    pub bite_eat_delay: Option<u32>,
    pub delayed_heal: Option<CombatEffect>,
    pub stat_effect: Option<fn(&mut Player) -> ()>,
    pub heal_function: Option<fn(&Player) -> u32>,
    pub overheal: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FoodPropertiesBuilder {
    properties: FoodProperties,
}

impl FoodPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            properties: FoodProperties::default(),
        }
    }

    pub fn total_bites(mut self, total_bites: u32) -> Self {
        self.properties.total_bites = Some(total_bites);
        self.properties.remaining_bites = Some(total_bites);
        self
    }

    pub fn bite_heal_amount(mut self, bite_heal_amount: u32) -> Self {
        self.properties.bite_heal_amount = Some(bite_heal_amount);
        self
    }

    pub fn bite_eat_delay(mut self, bite_eat_delay: u32) -> Self {
        self.properties.bite_eat_delay = Some(bite_eat_delay);
        self
    }

    pub fn delayed_heal(mut self, delayed_heal_effect: CombatEffect) -> Self {
        self.properties.delayed_heal = Some(delayed_heal_effect);
        self
    }

    pub fn stat_effect(mut self, stat_effect: fn(&mut Player) -> ()) -> Self {
        self.properties.stat_effect = Some(stat_effect);
        self
    }

    pub fn heal_function(mut self, heal_function: fn(&Player) -> u32) -> Self {
        self.properties.heal_function = Some(heal_function);
        self
    }

    pub fn overheal(mut self) -> Self {
        self.properties.overheal = true;
        self
    }

    pub fn build(self) -> FoodProperties {
        self.properties
    }
}

#[derive(Debug, Clone)]
pub struct Food {
    pub name: Foods,
    pub heal_amount: u32,
    pub eat_delay: u32,
    pub attack_delay: u32,
    pub food_type: FoodType,
    pub food_properties: FoodProperties,
}

impl Food {
    pub fn new(
        name: Foods,
        heal_amount: u32,
        eat_delay: u32,
        attack_delay: u32,
        food_type: FoodType,
    ) -> Self {
        Self {
            name,
            heal_amount,
            eat_delay,
            attack_delay,
            food_type,
            food_properties: FoodProperties::default(),
        }
    }

    pub fn with_properties(mut self, food_properties: FoodProperties) -> Self {
        self.food_properties = food_properties;
        self
    }
}
macro_rules! simple_foods {
    ($map:ident; $($name:ident: $heal:expr, $eat:expr, $atk:expr, $type:ident),* $(,)?) => {
        $($map.insert(Foods::$name, Food::new(Foods::$name, $heal, $eat, $atk, FoodType::$type));)*
    };
}

macro_rules! multibite_foods {
    ($map:ident; $($name:ident: $heal:expr, $eat:expr, $atk:expr; $bites:expr, $bite_heal:expr, $bite_delay:expr),* $(,)?) => {
        $(
            $map.insert(Foods::$name, Food::new(Foods::$name, $heal, $eat, $atk, FoodType::MultiBite)
                .with_properties(FoodPropertiesBuilder::new()
                    .total_bites($bites).bite_heal_amount($bite_heal).bite_eat_delay($bite_delay).build()));
        )*
    };
}

macro_rules! delayed_heal_foods {
    ($map:ident; $($name:ident: $heal:expr => $delayed:expr; $tick_delay:expr),* $(,)?) => {
        $(
            $map.insert(Foods::$name, Food::new(Foods::$name, $heal, 3, 3, FoodType::DelayedHeal)
                .with_properties(FoodPropertiesBuilder::new()
                    .delayed_heal(CombatEffect::DelayedHeal {
                        tick_delay: $tick_delay, tick_counter: Some($tick_delay), num_heals: 1, heal: $delayed
                    }).build()));
        )*
    };
}

lazy_static! {
    static ref FOOD_DB: HashMap<Foods, Food> = {
        let mut map = HashMap::new();

        // Simple foods (Normal, Combo, Potion without special properties)
        // Format: name: heal, eat_delay, atk_delay, food_type
        simple_foods!(map;
            Anchovies: 1, 3, 3, Normal,
            Sardine: 4, 3, 3, Normal,
            Trout: 7, 3, 3, Normal,
            Cod: 7, 3, 3, Normal,
            UgthankiKebab: 19, 3, 3, Normal,
            Kebab: 3, 3, 3, Normal,
            Stew: 11, 3, 3, Normal,
            Shrimps: 3, 3, 3, Normal,
            Chicken: 3, 3, 3, Normal,
            Meat: 3, 3, 3, Normal,
            Bread: 5, 3, 3, Normal,
            Herring: 5, 3, 3, Normal,
            Mackerel: 6, 3, 3, Normal,
            Pike: 8, 3, 3, Normal,
            Peach: 8, 3, 3, Normal,
            Salmon: 9, 3, 3, Normal,
            Tuna: 10, 3, 3, Normal,
            JugOfWine: 11, 3, 3, Normal,
            Lobster: 12, 3, 3, Normal,
            Bass: 13, 3, 3, Normal,
            Swordfish: 14, 3, 3, Normal,
            PotatoWithCheese: 16, 3, 3, Normal,
            Monkfish: 16, 3, 3, Normal,
            Curry: 19, 3, 3, Normal,
            Shark: 20, 3, 3, Normal,
            SeaTurtle: 21, 3, 3, Normal,
            MantaRay: 22, 3, 3, Normal,
            TunaPotato: 22, 3, 3, Normal,
            DarkCrab: 22, 3, 3, Normal,
            PurpleSweets: 1, 3, 3, Normal,
            MushroomPotato: 20, 3, 3, Normal,
            Paddlefish: 20, 3, 3, Normal,
            MossLizard: 33, 3, 3, Normal,
            Bream: 33, 3, 3, Normal,
            SnowyKnight: 15, 3, 0, Potion,
            Karambwan: 18, 3, 2, Combo,
            CorruptedPaddlefish: 16, 3, 2, Combo,
            ToadCrunchies: 8, 3, 2, Combo,
            SpicyCrunchies: 7, 3, 2, Combo,
            WormCrunchies: 8, 3, 2, Combo,
            ChocchipCrunchies: 7, 3, 2, Combo,
            MeatPizza: 16, 1, 3, Combo,
            FruitBatta: 11, 3, 2, Combo,
            ToadBatta: 11, 3, 2, Combo,
            WormBatta: 11, 3, 2, Combo,
            VegetableBatta: 11, 3, 2, Combo,
            CheeseTomBatta: 11, 3, 2, Combo,
            WormHole: 12, 3, 2, Combo,
            VegBall: 12, 3, 2, Combo,
            ChocolateBomb: 15, 3, 2, Combo,
            TangledToadsLegs: 15, 3, 2, Combo,
        );

        // MultiBite foods (pies, cakes, pizzas)
        // Format: name: heal, eat_delay, atk_delay; bites, bite_heal, bite_delay
        multibite_foods!(map;
            Cake: 12, 2, 3; 3, 4, 2,
            ChocolateCake: 15, 3, 3; 3, 5, 2,
            MeatPie: 12, 2, 3; 2, 6, 1,
            PlainPizza: 14, 2, 3; 2, 7, 1,
            ApplePie: 14, 2, 3; 2, 7, 1,
            AnchovyPizza: 18, 2, 3; 2, 9, 1,
            PineapplePizza: 22, 2, 3; 2, 11, 1,
            RedberryPie: 10, 2, 3; 2, 5, 1,
            DragonfruitPie: 20, 1, 3; 2, 10, 1,
            SummerPie: 22, 1, 3; 2, 11, 1,
            WildPie: 22, 1, 3; 2, 11, 1,
            GardenPie: 12, 1, 3; 2, 6, 1,
            FishPie: 12, 1, 3; 2, 6, 1,
            BotanicalPie: 14, 1, 3; 2, 7, 1,
            MushroomPie: 16, 1, 3; 2, 8, 1,
            AdmiralPie: 16, 1, 3; 2, 8, 1,
        );

        // Delayed heal foods (hunter kebbits/antelope)
        // Format: name: initial_heal => delayed_heal; tick_delay
        delayed_heal_foods!(map;
            Kyatt: 9 => 8; 7,
            PyreFox: 11 => 8; 7,
            SunlightAntelope: 12 => 9; 7,
            WildKebbit: 44 => 4; 7,
            Larupia: 6 => 5; 7,
            BarbTailedKebbit: 7 => 5; 7,
            Graahk: 8 => 6; 7,
            DashingKebbit: 13 => 10; 7,
            MoonlightAntelope: 14 => 12; 7,
        );

        // Complex foods with custom properties (heal functions, stat effects, etc.)
        map.insert(Foods::GuthixRest, Food::new(Foods::GuthixRest, 20, 3, 0, FoodType::Potion)
            .with_properties(FoodPropertiesBuilder::new()
                .bite_heal_amount(5).total_bites(4).build()));

        map.insert(Foods::BasketOfStrawberries, Food::new(Foods::BasketOfStrawberries, 0, 3, 3, FoodType::MultiBite)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(5).bite_eat_delay(3)
                .heal_function(|player| player.stats.hitpoints.base * 6 / 100 + 1).build()));

        map.insert(Foods::Anglerfish, Food::new(Foods::Anglerfish, 0, 3, 3, FoodType::Normal)
            .with_properties(FoodPropertiesBuilder::new()
                .heal_function(|player| player.stats.hitpoints.base * 6 / 100 + 1)
                .overheal().build()));

        map.insert(Foods::SaradominBrew, Food::new(Foods::SaradominBrew, 0, 3, 0, FoodType::Potion)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(4).bite_eat_delay(3)
                .heal_function(|player| player.stats.hitpoints.base * 3 / 20 + 2)
                .stat_effect(|player| {
                    let def_boost = player.stats.defence.base / 5 + 2;
                    player.stats.defence.restore(def_boost, Some(def_boost + player.stats.defence.base));
                    player.stats.attack.drain(player.stats.attack.current / 10 + 2);
                    player.stats.strength.drain(player.stats.strength.current / 10 + 2);
                    player.stats.ranged.drain(player.stats.ranged.current / 10 + 2);
                    player.stats.magic.drain(player.stats.magic.current / 10 + 2);
                })
                .overheal().build()));

        map.insert(Foods::XericsAidMinus, Food::new(Foods::XericsAidMinus, 0, 3, 0, FoodType::Potion)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(4).bite_eat_delay(3)
                .heal_function(|player| player.stats.hitpoints.base * 7 / 100 + 1)
                .stat_effect(|player| {
                    let def_boost = player.stats.defence.base * 14 / 100 + 1;
                    player.stats.defence.restore(def_boost, Some(def_boost + player.stats.defence.base));
                    player.stats.attack.drain(player.stats.attack.current * 7 / 100 + 1);
                    player.stats.strength.drain(player.stats.strength.current * 7 / 100 + 1);
                    player.stats.ranged.drain(player.stats.ranged.current * 7 / 100 + 1);
                    player.stats.magic.drain(player.stats.magic.current * 7 / 100 + 1);
                })
                .overheal().build()));

        map.insert(Foods::XericsAid, Food::new(Foods::XericsAid, 0, 3, 0, FoodType::Potion)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(4).bite_eat_delay(3)
                .heal_function(|player| player.stats.hitpoints.base * 12 / 100 + 2)
                .stat_effect(|player| {
                    let def_boost = player.stats.defence.base * 18 / 100 + 2;
                    player.stats.defence.restore(def_boost, Some(def_boost + player.stats.defence.base));
                    player.stats.attack.drain(player.stats.attack.current * 9 / 100 + 2);
                    player.stats.strength.drain(player.stats.strength.current * 9 / 100 + 2);
                    player.stats.ranged.drain(player.stats.ranged.current * 9 / 100 + 2);
                    player.stats.magic.drain(player.stats.magic.current * 9 / 100 + 2);
                })
                .overheal().build()));

        map.insert(Foods::XericsAidPlus, Food::new(Foods::XericsAidPlus, 0, 3, 0, FoodType::Potion)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(4).bite_eat_delay(3)
                .heal_function(|player| player.stats.hitpoints.base * 3 / 20 + 5)
                .stat_effect(|player| {
                    let def_boost = player.stats.defence.base / 5 + 5;
                    player.stats.defence.restore(def_boost, Some(def_boost + player.stats.defence.base));
                    player.stats.attack.drain(player.stats.attack.current / 10 + 4);
                    player.stats.strength.drain(player.stats.strength.current / 10 + 4);
                    player.stats.ranged.drain(player.stats.ranged.current / 10 + 4);
                    player.stats.magic.drain(player.stats.magic.current / 10 + 4);
                })
                .overheal().build()));

        map.insert(Foods::Nectar, Food::new(Foods::Nectar, 0, 3, 0, FoodType::Potion)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(4).bite_eat_delay(3)
                .heal_function(|player| player.stats.hitpoints.base * 3 / 20 + 3)
                .stat_effect(|player| {
                    player.stats.attack.drain(player.stats.attack.current / 20 + 5);
                    player.stats.strength.drain(player.stats.strength.current / 20 + 5);
                    player.stats.defence.drain(player.stats.defence.current / 20 + 5);
                    player.stats.ranged.drain(player.stats.ranged.current / 20 + 5);
                    player.stats.magic.drain(player.stats.magic.current / 20 + 5);
                })
                .overheal().build()));

        map.insert(Foods::SilkDressing, Food::new(Foods::SilkDressing, 0, 3, 0, FoodType::DelayedHeal)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(2)
                .delayed_heal(CombatEffect::DelayedHeal {
                    tick_delay: 5, tick_counter: Some(5), num_heals: 20, heal: 5
                }).build()));

        map.insert(Foods::Ambrosia, Food::new(Foods::Ambrosia, 0, 3, 0, FoodType::Potion)
            .with_properties(FoodPropertiesBuilder::new()
                .total_bites(2).bite_eat_delay(3)
                .stat_effect(|player| {
                    let hp_boost = player.stats.hitpoints.base / 4 + 2;
                    player.stats.hitpoints.current = player.stats.hitpoints.base + hp_boost;
                    let prayer_boost = player.stats.prayer.base / 5 + 5;
                    player.stats.prayer.current = player.stats.prayer.base + prayer_boost;
                })
                .overheal().build()));

        map
    };
}
