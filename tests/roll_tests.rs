use osrs::equipment::{Armor, CombatStyle, CombatType, Weapon};
use osrs::monster::Monster;
use osrs::player::{Gear, Player, PlayerStats};
use osrs::potions::{Potion, PotionBoost};
use osrs::prayers::{Prayer, PrayerBoost};
use osrs::rolls::{
    calc_player_def_rolls, calc_player_magic_rolls, calc_player_melee_rolls,
    calc_player_ranged_rolls,
};
use rstest::{fixture, rstest};

#[fixture]
fn max_melee_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats {
        attack: 99,
        strength: 99,
        defence: 99,
        ranged: 99,
        magic: 99,
        hitpoints: 99,
        prayer: 99,
    };
    player.prayers.add(PrayerBoost::new(Prayer::Piety));
    player.potions.attack = Some(PotionBoost::new(Potion::SuperAttack));
    player.potions.strength = Some(PotionBoost::new(Potion::SuperStrength));
    player.potions.defence = Some(PotionBoost::new(Potion::SuperDefence));

    player.calc_potion_boosts();
    player.reset_live_stats();

    player.gear = Gear {
        head: Some(Armor::new("Torva full helm")),
        neck: Some(Armor::new("Amulet of torture")),
        cape: Some(Armor::new("Infernal cape")),
        ammo: Some(Armor::new("Rada's blessing 4")),
        weapon: Weapon::new("Ghrazi rapier"),
        shield: Some(Armor::new("Avernic defender")),
        body: Some(Armor::new("Torva platebody")),
        legs: Some(Armor::new("Torva platelegs")),
        hands: Some(Armor::new("Ferocious gloves")),
        feet: Some(Armor::new("Primordial boots")),
        ring: Some(Armor::new("Ultor ring")),
    };
    player.update_bonuses();
    player.set_active_style(CombatStyle::Lunge);

    player
}

#[fixture]
fn mid_level_melee_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats {
        attack: 80,
        strength: 80,
        defence: 80,
        ranged: 80,
        magic: 80,
        hitpoints: 80,
        prayer: 70,
    };
    player.prayers.add(PrayerBoost::new(Prayer::Piety));
    player.potions.attack = Some(PotionBoost::new(Potion::SuperAttack));
    player.potions.strength = Some(PotionBoost::new(Potion::SuperStrength));
    player.potions.defence = Some(PotionBoost::new(Potion::SuperDefence));

    player.calc_potion_boosts();
    player.reset_live_stats();

    player.gear = Gear {
        head: Some(Armor::new("Helm of neitiznot")),
        neck: Some(Armor::new("Amulet of fury")),
        cape: Some(Armor::new("Fire cape")),
        ammo: Some(Armor::new("Rada's blessing 3")),
        weapon: Weapon::new("Abyssal whip"),
        shield: Some(Armor::new("Dragon defender")),
        body: Some(Armor::new("Fighter torso")),
        legs: Some(Armor::new("Obsidian platelegs")),
        hands: Some(Armor::new("Barrows gloves")),
        feet: Some(Armor::new("Dragon boots")),
        ring: Some(Armor::new("Berserker ring (i)")),
    };
    player.update_bonuses();
    player.set_active_style(CombatStyle::Lash);

    player
}

#[rstest]
fn test_max_melee_player_rolls(mut max_melee_player: Player) {
    let monster = Monster::new("Ammonite Crab").unwrap();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 33525);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 56);
}

#[rstest]
fn test_mid_level_melee_player_rolls(mut mid_level_melee_player: Player) {
    let monster = Monster::new("Ammonite Crab").unwrap();
    calc_player_melee_rolls(&mut mid_level_melee_player, &monster);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Slash], 24125);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Stab], 39);
}
