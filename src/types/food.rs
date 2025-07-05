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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoodType {
    Normal,
    Combo,
    Potion,
    MultiBite,
    DelayedHeal,
}

impl Default for FoodType {
    fn default() -> Self {
        Self::Normal
    }
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
lazy_static! {
    static ref FOOD_DB: HashMap<Foods, Food> = {
        let mut map = HashMap::new();
        map.insert(Foods::Anchovies, Food::new(Foods::Anchovies, 1, 3, 3, FoodType::Normal));
        map.insert(Foods::Sardine, Food::new(Foods::Sardine, 4, 3, 3, FoodType::Normal));
        map.insert(Foods::Trout, Food::new(Foods::Trout, 7, 3, 3, FoodType::Normal));
        map.insert(Foods::Cod, Food::new(Foods::Cod, 7, 3, 3, FoodType::Normal));
        map.insert(Foods::UgthankiKebab, Food::new(Foods::UgthankiKebab, 19, 3, 3, FoodType::Normal));
        map.insert(Foods::Kebab, Food::new(Foods::Kebab, 3, 3, 3, FoodType::Normal));
        map.insert(Foods::Stew, Food::new(Foods::Stew, 11, 3, 3, FoodType::Normal));
        map.insert(Foods::Shrimps, Food::new(Foods::Shrimps, 3, 3, 3, FoodType::Normal));
        map.insert(Foods::Chicken, Food::new(Foods::Chicken, 3, 3, 3, FoodType::Normal));
        map.insert(Foods::Meat, Food::new(Foods::Meat, 3, 3, 3, FoodType::Normal));
        map.insert(Foods::Bread, Food::new(Foods::Bread, 5, 3, 3, FoodType::Normal));
        map.insert(Foods::Herring, Food::new(Foods::Herring, 5, 3, 3, FoodType::Normal));
        map.insert(Foods::Mackerel, Food::new(Foods::Mackerel, 6, 3, 3, FoodType::Normal));
        map.insert(Foods::Pike, Food::new(Foods::Pike, 8, 3, 3, FoodType::Normal));
        map.insert(Foods::Peach, Food::new(Foods::Peach, 8, 3, 3, FoodType::Normal));
        map.insert(Foods::Salmon, Food::new(Foods::Salmon, 9, 3, 3, FoodType::Normal));
        map.insert(Foods::Tuna, Food::new(Foods::Tuna, 10, 3, 3, FoodType::Normal));
        map.insert(Foods::JugOfWine, Food::new(Foods::JugOfWine, 11, 3, 3, FoodType::Normal));
        map.insert(
            Foods::Cake,
            Food::new(Foods::Cake, 12, 2, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(3)
                        .bite_heal_amount(4)
                        .bite_eat_delay(2)
                        .build()
                ),
        );
        map.insert(
            Foods::MeatPie,
            Food::new(Foods::MeatPie, 12, 2, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(6)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(Foods::Lobster, Food::new(Foods::Lobster, 12, 3, 3, FoodType::Normal));
        map.insert(Foods::Bass, Food::new(Foods::Bass, 13, 3, 3, FoodType::Normal));
        map.insert(
            Foods::PlainPizza,
            Food::new(Foods::PlainPizza, 14, 2, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(7)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(Foods::Swordfish, Food::new(Foods::Swordfish, 14, 3, 3, FoodType::Normal));
        map.insert(
            Foods::ApplePie,
            Food::new(Foods::ApplePie, 14, 2, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(7)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::ChocolateCake,
            Food::new(Foods::ChocolateCake, 15, 3, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(3)
                        .bite_heal_amount(5)
                        .bite_eat_delay(2)
                        .build()
                )
        );
        map.insert(Foods::SnowyKnight, Food::new(Foods::SnowyKnight, 15, 3, 0, FoodType::Potion));
        map.insert(Foods::PotatoWithCheese, Food::new(Foods::PotatoWithCheese, 16, 3, 3, FoodType::Normal));
        map.insert(Foods::Monkfish, Food::new(Foods::Monkfish, 16, 3, 3, FoodType::Normal));
        map.insert(
            Foods::Kyatt,
            Food::new(Foods::Kyatt, 9, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 8
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::AnchovyPizza,
            Food::new(Foods::AnchovyPizza, 18, 2, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(9)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(Foods::Karambwan, Food::new(Foods::Karambwan, 18, 3, 2, FoodType::Combo));
        map.insert(Foods::Curry, Food::new(Foods::Curry, 19, 3, 3, FoodType::Normal));
        map.insert(
            Foods::PyreFox,
            Food::new(Foods::PyreFox, 11, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 8
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::GuthixRest,
            Food::new(Foods::GuthixRest, 20, 3, 0, FoodType::Potion)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .bite_heal_amount(5)
                        .total_bites(4)
                        .build()
                )
        );
        map.insert(Foods::Shark, Food::new(Foods::Shark, 20, 3, 3, FoodType::Normal));
        map.insert(Foods::SeaTurtle, Food::new(Foods::SeaTurtle, 21, 3, 3, FoodType::Normal));
        map.insert(
            Foods::SunlightAntelope,
            Food::new(Foods::SunlightAntelope, 12, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 9
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::PineapplePizza,
            Food::new(Foods::PineapplePizza, 22, 2, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(11)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::DragonfruitPie,
            Food::new(Foods::DragonfruitPie, 20, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(10)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::SummerPie,
            Food::new(Foods::SummerPie, 22, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(11)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::BasketOfStrawberries,
            Food::new(Foods::BasketOfStrawberries, 0, 3, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(5)
                        .bite_eat_delay(3)
                        .heal_function(|player| player.stats.hitpoints.base * 6 / 100 + 1)
                        .build()
                )
        );



        map.insert(
            Foods::SaradominBrew,
            Food::new(Foods::SaradominBrew, 0, 3, 0, FoodType::Potion)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(4)
                        .bite_eat_delay(3)
                        .heal_function(|player| player.stats.hitpoints.base * 3 / 20 + 2)
                        .stat_effect(
                            |player| {
                                let def_boost = player.stats.defence.base / 5 + 2;
                                player.stats.defence.restore(
                                    def_boost, Some(def_boost + player.stats.defence.base)
                                );

                                player.stats.attack.drain(player.stats.attack.current / 10 + 2);
                                player.stats.strength.drain(player.stats.strength.current / 10 + 2);
                                player.stats.ranged.drain(player.stats.ranged.current / 10 + 2);
                                player.stats.magic.drain(player.stats.magic.current / 10 + 2);
                            }
                        )
                        .overheal()
                        .build()
                )
        );
        map.insert(Foods::Paddlefish, Food::new(Foods::Paddlefish, 20, 3, 3, FoodType::Normal));
        map.insert(Foods::CorruptedPaddlefish, Food::new(Foods::CorruptedPaddlefish, 16, 3, 2, FoodType::Combo));
        map.insert(
            Foods::WildKebbit,
            Food::new(Foods::WildKebbit, 44, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 4
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::Larupia,
            Food::new(Foods::Larupia, 6, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 5
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::BarbTailedKebbit,
            Food::new(Foods::BarbTailedKebbit, 7, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 5
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::Graahk,
            Food::new(Foods::Graahk, 8, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 6
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::WildPie,
            Food::new(Foods::WildPie, 22, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(11)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(Foods::MantaRay, Food::new(Foods::MantaRay, 22, 3, 3, FoodType::Normal));
        map.insert(Foods::TunaPotato, Food::new(Foods::TunaPotato, 22, 3, 3, FoodType::Normal));
        map.insert(Foods::DarkCrab, Food::new(Foods::DarkCrab, 22, 3, 3, FoodType::Normal));
        map.insert(
            Foods::Anglerfish,
            Food::new(Foods::Anglerfish, 0, 3, 3, FoodType::Normal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .heal_function(|player| player.stats.hitpoints.base * 6 / 100 + 1)
                        .overheal()
                        .build()
                )
        );

        map.insert(
            Foods::DashingKebbit,
            Food::new(Foods::DashingKebbit, 13, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 10
                            }
                        )
                        .build()
                )
        );
        map.insert(
            Foods::MoonlightAntelope,
            Food::new(Foods::MoonlightAntelope, 14, 3, 3, FoodType::DelayedHeal)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .delayed_heal(
                            CombatEffect::DelayedHeal {
                                tick_delay: 7, tick_counter: Some(7), num_heals: 1, heal: 12
                            }
                        )
                        .build()
                )
        );
        map.insert(Foods::PurpleSweets, Food::new(Foods::PurpleSweets, 1, 3, 3, FoodType::Normal));
        map.insert(
            Foods::RedberryPie,
            Food::new(Foods::RedberryPie, 10, 2, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(5)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::GardenPie,
            Food::new(Foods::GardenPie, 12, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(6)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::FishPie,
            Food::new(Foods::FishPie, 12, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(6)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::BotanicalPie,
            Food::new(Foods::BotanicalPie, 14, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(7)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::MushroomPie,
            Food::new(Foods::MushroomPie, 16, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(8)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(
            Foods::AdmiralPie,
            Food::new(Foods::AdmiralPie, 16, 1, 3, FoodType::MultiBite)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(2)
                        .bite_heal_amount(8)
                        .bite_eat_delay(1)
                        .build()
                )
        );
        map.insert(Foods::ToadCrunchies, Food::new(Foods::ToadCrunchies, 8, 3, 2, FoodType::Combo));
        map.insert(Foods::SpicyCrunchies, Food::new(Foods::SpicyCrunchies, 7, 3, 2, FoodType::Combo));
        map.insert(Foods::WormCrunchies, Food::new(Foods::WormCrunchies, 8, 3, 2, FoodType::Combo));
        map.insert(Foods::ChocchipCrunchies, Food::new(Foods::ChocchipCrunchies, 7, 3, 2, FoodType::Combo));
        map.insert(Foods::MeatPizza, Food::new(Foods::MeatPizza, 16, 1, 3, FoodType::Combo));
        map.insert(Foods::FruitBatta, Food::new(Foods::FruitBatta, 11, 3, 2, FoodType::Combo));
        map.insert(Foods::ToadBatta, Food::new(Foods::ToadBatta, 11, 3, 2, FoodType::Combo));
        map.insert(Foods::WormBatta, Food::new(Foods::WormBatta, 11, 3, 2, FoodType::Combo));
        map.insert(Foods::VegetableBatta, Food::new(Foods::VegetableBatta, 11, 3, 2, FoodType::Combo));
        map.insert(Foods::CheeseTomBatta, Food::new(Foods::CheeseTomBatta, 11, 3, 2, FoodType::Combo));
        map.insert(Foods::WormHole, Food::new(Foods::WormHole, 12, 3, 2, FoodType::Combo));
        map.insert(Foods::VegBall, Food::new(Foods::VegBall, 12, 3, 2, FoodType::Combo));
        map.insert(Foods::ChocolateBomb, Food::new(Foods::ChocolateBomb, 15, 3, 2, FoodType::Combo));
        map.insert(Foods::TangledToadsLegs, Food::new(Foods::TangledToadsLegs, 15, 3, 2, FoodType::Combo));
        map.insert(Foods::MushroomPotato, Food::new(Foods::MushroomPotato, 20, 3, 3, FoodType::Normal));
        map.insert(
            Foods::XericsAidMinus,
            Food::new(Foods::XericsAidMinus, 0, 3, 0, FoodType::Potion)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(4)
                        .bite_eat_delay(3)
                        .heal_function(|player| player.stats.hitpoints.base * 7 / 100 + 1)
                        .stat_effect(
                            |player| {
                                let def_boost = player.stats.defence.base * 14 / 100 + 1;
                                player.stats.defence.restore(
                                    def_boost, Some(def_boost + player.stats.defence.base)
                                );

                                player.stats.attack.drain(player.stats.attack.current * 7 / 100 + 1);
                                player.stats.strength.drain(player.stats.strength.current * 7 / 100 + 1);
                                player.stats.ranged.drain(player.stats.ranged.current * 7 / 100 + 1);
                                player.stats.magic.drain(player.stats.magic.current * 7 / 100 + 1);
                            }
                        )
                        .overheal()
                        .build()
                )
        );
        map.insert(
            Foods::XericsAid,
            Food::new(Foods::XericsAid, 0, 3, 0, FoodType::Potion)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(4)
                        .bite_eat_delay(3)
                        .heal_function(|player| player.stats.hitpoints.base * 12 / 100 + 2)
                        .stat_effect(
                            |player| {
                                let def_boost = player.stats.defence.base * 18 / 100 + 2;
                                player.stats.defence.restore(
                                    def_boost, Some(def_boost + player.stats.defence.base)
                                );

                                player.stats.attack.drain(player.stats.attack.current * 9 / 100 + 2);
                                player.stats.strength.drain(player.stats.strength.current * 9 / 100 + 2);
                                player.stats.ranged.drain(player.stats.ranged.current * 9 / 100 + 2);
                                player.stats.magic.drain(player.stats.magic.current * 9 / 100 + 2);
                            }
                        )
                        .overheal()
                        .build()
                )
        );
        map.insert(
            Foods::XericsAidPlus,
            Food::new(Foods::XericsAidPlus, 0, 3, 0, FoodType::Potion)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(4)
                        .bite_eat_delay(3)
                        .heal_function(|player| player.stats.hitpoints.base * 3 / 20 + 5)
                        .stat_effect(
                            |player| {
                                let def_boost = player.stats.defence.base / 5 + 5;
                                player.stats.defence.restore(
                                    def_boost, Some(def_boost + player.stats.defence.base)
                                );

                                player.stats.attack.drain(player.stats.attack.current / 10 + 4);
                                player.stats.strength.drain(player.stats.strength.current / 10 + 4);
                                player.stats.ranged.drain(player.stats.ranged.current / 10 + 4);
                                player.stats.magic.drain(player.stats.magic.current / 10 + 4);
                            }
                        )
                        .overheal()
                        .build()
                )
        );
        map.insert(
            Foods::Nectar,
            Food::new(Foods::Nectar, 0, 3, 0, FoodType::Potion)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(4)
                        .bite_eat_delay(3)
                        .heal_function(|player| player.stats.hitpoints.base * 3 / 20 + 3)
                        .stat_effect(
                            |player| {
                                player.stats.attack.drain(player.stats.attack.current / 20 + 5);
                                player.stats.strength.drain(player.stats.strength.current / 20 + 5);
                                player.stats.defence.drain(player.stats.defence.current / 20 + 5);
                                player.stats.ranged.drain(player.stats.ranged.current / 20 + 5);
                                player.stats.magic.drain(player.stats.magic.current / 20 + 5);
                            }
                        )
                        .overheal()
                        .build()
                )
        );
        map.insert(
            Foods::Nectar,
            Food::new(Foods::Nectar, 0, 3, 0, FoodType::Potion)
                .with_properties(
                    FoodPropertiesBuilder::new()
                        .total_bites(4)
                        .bite_eat_delay(3)
                        .heal_function(|player| player.stats.hitpoints.base * 3 / 20 + 3)
                        .stat_effect(
                            |player| {
                                player.stats.attack.drain(player.stats.attack.current / 20 + 5);
                                player.stats.strength.drain(player.stats.strength.current / 20 + 5);
                                player.stats.defence.drain(player.stats.defence.current / 20 + 5);
                                player.stats.ranged.drain(player.stats.ranged.current / 20 + 5);
                                player.stats.magic.drain(player.stats.magic.current / 20 + 5);
                            }
                        )
                        .overheal()
                        .build()
                )
        );

        map.insert(Foods::SilkDressing, Food::new(Foods::SilkDressing, 0, 3, 0, FoodType::DelayedHeal)
            .with_properties(
                FoodPropertiesBuilder::new()
                    .total_bites(2)
                    .delayed_heal(
                        CombatEffect::DelayedHeal {
                            tick_delay: 5, tick_counter: Some(5), num_heals: 20, heal: 5
                        }
                    )
                    .build()
            )
        );
        map.insert(Foods::Ambrosia, Food::new(Foods::Ambrosia, 0, 3, 0, FoodType::Potion)
            .with_properties(
                FoodPropertiesBuilder::new()
                    .total_bites(2)
                    .bite_eat_delay(3)
                    .stat_effect(
                        |player| {
                            let hp_boost = player.stats.hitpoints.base / 4 + 2;
                            player.stats.hitpoints.current = player.stats.hitpoints.base + hp_boost;
                            let prayer_boost = player.stats.prayer.base / 5 + 5;
                            player.stats.prayer.current = player.stats.prayer.base + prayer_boost;
                        }
                    )
                    .overheal()
                    .build()
            ));
        // TODO: Add player fishing and hunter levels for these foods
        map.insert(Foods::MossLizard, Food::new(Foods::MossLizard, 33, 3, 3, FoodType::Normal));
        map.insert(Foods::Bream, Food::new(Foods::Bream, 33, 3, 3, FoodType::Normal));

        map
    };
}
