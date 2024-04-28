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
fn vorkath() -> Monster {
    Monster::new("Vorkath").unwrap()
}

#[fixture]
fn kril() -> Monster {
    Monster::new("K'ril Tsutsaroth").unwrap()
}

#[fixture]
fn kalphite() -> Monster {
    Monster::new("Kalphite Soldier").unwrap()
}

#[fixture]
fn ammonite_crab() -> Monster {
    Monster::new("Ammonite Crab").unwrap()
}

#[fixture]
fn vetion() -> Monster {
    Monster::new("Vet'ion (Normal)").unwrap()
}

#[fixture]
fn duke() -> Monster {
    Monster::new("Duke Sucellus (Awake)").unwrap()
}

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
fn test_max_melee_player_rolls(mut max_melee_player: Player, ammonite_crab: Monster) {
    calc_player_melee_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 33525);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 56);
}

#[rstest]
fn test_mid_level_melee_player_rolls(mut mid_level_melee_player: Player, ammonite_crab: Monster) {
    calc_player_melee_rolls(&mut mid_level_melee_player, &ammonite_crab);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Slash], 24125);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Stab], 39);
}

#[rstest]
fn test_max_melee_dhl_vorkath(mut max_melee_player: Player, vorkath: Monster) {
    max_melee_player.equip("Dragon hunter lance");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vorkath);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 38880);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 60);
}

#[rstest]
fn test_max_melee_keris_partisan_kalphite(mut max_melee_player: Player, kalphite: Monster) {
    max_melee_player.equip("Keris partisan");
    max_melee_player.set_active_style(CombatStyle::Pound);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &kalphite);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 27714);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 59);
}

#[rstest]
fn test_mid_level_melee_barronite_golem(mut mid_level_melee_player: Player) {
    let monster = Monster::new("Chaos Golem").unwrap();
    mid_level_melee_player.equip("Barronite mace");
    mid_level_melee_player.set_active_style(CombatStyle::Pummel);
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &monster);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Crush], 18600);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Crush], 35);
}

#[rstest]
fn test_max_melee_ursine_chainmace_vetion(mut max_melee_player: Player, vetion: Monster) {
    max_melee_player.equip("Ursine chainmace");
    max_melee_player.set_active_style(CombatStyle::Pummel);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vetion);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 44700);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 78);
}

#[rstest]
fn test_max_melee_ursine_chainmace_non_wildy(mut max_melee_player: Player, ammonite_crab: Monster) {
    max_melee_player.equip("Ursine chainmace");
    max_melee_player.set_active_style(CombatStyle::Pummel);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 29800);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 52);
}

#[rstest]
fn test_berserker_necklace_obby_sword(mut max_melee_player: Player, ammonite_crab: Monster) {
    max_melee_player.equip("Berserker necklace");
    max_melee_player.equip("Toktz-xil-ak");
    max_melee_player.set_active_style(CombatStyle::Lunge);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 22797);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 54);
}

#[rstest]
#[case("Silverlight", (21456, 59))]
#[case("Darklight", (21754, 59))]
#[case("Arclight", (42554, 61))]
fn test_demonbane_against_kril(
    #[case] weapon: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
    kril: Monster,
) {
    max_melee_player.equip(weapon);
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &kril);

    assert_eq!(
        max_melee_player.att_rolls[&CombatType::Slash],
        expected_rolls.0
    );
    assert_eq!(
        max_melee_player.max_hits[&CombatType::Slash],
        expected_rolls.1
    );
}

#[rstest]
#[case("Silverlight", (21456, 59))]
#[case("Darklight", (21754, 52))]
#[case("Arclight", (36636, 53))]
fn test_demonbane_against_duke(
    #[case] weapon: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
    duke: Monster,
) {
    max_melee_player.equip(weapon);
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &duke);

    assert_eq!(
        max_melee_player.att_rolls[&CombatType::Slash],
        expected_rolls.0
    );
    assert_eq!(
        max_melee_player.max_hits[&CombatType::Slash],
        expected_rolls.1
    );
}
