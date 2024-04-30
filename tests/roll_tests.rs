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
fn spindel() -> Monster {
    Monster::new("Spindel").unwrap()
}

#[fixture]
fn duke() -> Monster {
    Monster::new("Duke Sucellus (Awake)").unwrap()
}

#[fixture]
fn kurask() -> Monster {
    Monster::new("Kurask (Normal)").unwrap()
}

#[fixture]
fn scurrius() -> Monster {
    Monster::new("Scurrius (Solo)").unwrap()
}

#[fixture]
fn revenant_dragon() -> Monster {
    Monster::new("Revenant dragon").unwrap()
}

#[fixture]
fn zebak() -> Monster {
    Monster::new("Zebak").unwrap()
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
        second_ammo: None,
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
        second_ammo: None,
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

#[fixture]
fn max_ranged_player() -> Player {
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
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));
    player.potions.ranged = Some(PotionBoost::new(Potion::Ranging));

    player.calc_potion_boosts();
    player.reset_live_stats();

    player.gear = Gear {
        head: Some(Armor::new("Masori mask (f)")),
        neck: Some(Armor::new("Necklace of anguish")),
        cape: Some(Armor::new("Dizana's quiver (charged)")),
        ammo: Some(Armor::new("Dragon bolts")),
        second_ammo: None,
        weapon: Weapon::new("Zaryte crossbow"),
        shield: Some(Armor::new("Twisted buckler")),
        body: Some(Armor::new("Masori body (f)")),
        legs: Some(Armor::new("Masori chaps (f)")),
        hands: Some(Armor::new("Zaryte vambraces")),
        feet: Some(Armor::new("Pegasian boots")),
        ring: Some(Armor::new("Venator ring")),
    };
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);

    player
}

#[fixture]
fn mid_level_ranged_player() -> Player {
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
    player.prayers.add(PrayerBoost::new(Prayer::EagleEye));
    player.potions.ranged = Some(PotionBoost::new(Potion::Ranging));

    player.calc_potion_boosts();
    player.reset_live_stats();

    player.gear = Gear {
        head: Some(Armor::new("Ancient coif")),
        neck: Some(Armor::new("Amulet of fury")),
        cape: Some(Armor::new("Ava's assembler")),
        ammo: Some(Armor::new("Adamant bolts")),
        second_ammo: None,
        weapon: Weapon::new("Rune crossbow"),
        shield: Some(Armor::new("Odium ward")),
        body: Some(Armor::new("Ancient d'hide body")),
        legs: Some(Armor::new("Ancient chaps")),
        hands: Some(Armor::new("Barrows gloves")),
        feet: Some(Armor::new("Ancient d'hide boots")),
        ring: Some(Armor::new("Archers ring (i)")),
    };
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);

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
fn test_max_melee_blue_keris_partisan_kalphite(mut max_melee_player: Player, kalphite: Monster) {
    max_melee_player.equip("Keris partisan of breaching");
    max_melee_player.set_active_style(CombatStyle::Pound);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &kalphite);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 36859);
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
#[case("Silverlight", (21456, 52))]
#[case("Darklight", (21754, 52))]
#[case("Arclight", (37297, 53))]
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

#[rstest]
fn test_lbba_against_kurask(mut mid_level_melee_player: Player, kurask: Monster) {
    mid_level_melee_player.equip("Leaf-bladed battleaxe");
    mid_level_melee_player.set_active_style(CombatStyle::Hack);
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &kurask);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Slash], 22692);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Slash], 49)
}

#[rstest]
#[case("Aberrant spectre", (29651, 60))]
#[case("Abhorrent spectre", (29651, 62))]
#[case("General Graardor", (29651, 64))]
#[case("Rune dragon", (29651, 66))]
fn test_colossal_blade(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Colossal blade");
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

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
fn test_bone_mace_scurrius(mut mid_level_melee_player: Player, scurrius: Monster) {
    mid_level_melee_player.equip("Bone mace");
    mid_level_melee_player.set_active_style(CombatStyle::Pummel);
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &scurrius);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Crush], 21080);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Crush], 44)
}

#[rstest]
fn test_max_melee_with_slayer_helm(mut max_melee_player: Player, ammonite_crab: Monster) {
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 39112);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 63);
}

#[rstest]
fn test_mid_level_melee_with_slayer_helm(
    mut mid_level_melee_player: Player,
    ammonite_crab: Monster,
) {
    mid_level_melee_player.equip("Slayer helmet (i)");
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &ammonite_crab);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Slash], 28145);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Slash], 45);
}

#[rstest]
fn test_max_melee_dhl_slayer_vorkath(mut max_melee_player: Player, vorkath: Monster) {
    max_melee_player.equip("Dragon hunter lance");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vorkath);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 45360);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 67);
}

#[rstest]
fn test_max_melee_keris_partisan_slayer_kalphite(mut max_melee_player: Player, kalphite: Monster) {
    max_melee_player.equip("Keris partisan");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.set_active_style(CombatStyle::Pound);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &kalphite);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 32333);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 66);
}

#[rstest]
fn test_max_melee_blue_keris_partisan_slayer_kalphite(
    mut max_melee_player: Player,
    kalphite: Monster,
) {
    max_melee_player.equip("Keris partisan of breaching");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.set_active_style(CombatStyle::Pound);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &kalphite);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 43002);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 66);
}

#[rstest]
fn test_max_melee_ursine_chainmace_slayer_vetion(mut max_melee_player: Player, vetion: Monster) {
    max_melee_player.equip("Ursine chainmace");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.set_active_style(CombatStyle::Pummel);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vetion);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 52149);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 87);
}

#[rstest]
fn test_berserker_necklace_obby_sword_slayer(mut max_melee_player: Player, ammonite_crab: Monster) {
    max_melee_player.equip("Berserker necklace");
    max_melee_player.equip("Toktz-xil-ak");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.set_active_style(CombatStyle::Lunge);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 26596);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 60);
}

#[rstest]
#[case("Silverlight", (25032, 64))]
#[case("Darklight", (25379, 64))]
#[case("Arclight", (49646, 66))]
fn test_demonbane_against_kril_slayer(
    #[case] weapon: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
    kril: Monster,
) {
    max_melee_player.equip(weapon);
    max_melee_player.equip("Slayer helmet (i)");
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
#[case("Silverlight", (25032, 56))]
#[case("Darklight", (25379, 56))]
#[case("Arclight", (43513, 58))]
fn test_demonbane_against_duke_slayer(
    #[case] weapon: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
    duke: Monster,
) {
    max_melee_player.equip(weapon);
    max_melee_player.equip("Slayer helmet (i)");
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

#[rstest]
fn test_lbba_against_kurask_slayer(mut mid_level_melee_player: Player, kurask: Monster) {
    mid_level_melee_player.equip("Leaf-bladed battleaxe");
    mid_level_melee_player.equip("Slayer helmet (i)");
    mid_level_melee_player.set_active_style(CombatStyle::Hack);
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &kurask);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Slash], 26474);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Slash], 55)
}

#[rstest]
#[case("Aberrant spectre", (34592, 67))]
#[case("Abhorrent spectre", (34592, 69))]
#[case("General Graardor", (34592, 71))]
#[case("Rune dragon", (34592, 73))]
fn test_colossal_blade_slayer(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Colossal blade");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

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
fn test_bone_mace_scurrius_slayer(mut mid_level_melee_player: Player, scurrius: Monster) {
    mid_level_melee_player.equip("Bone mace");
    mid_level_melee_player.equip("Slayer helmet (i)");
    mid_level_melee_player.set_active_style(CombatStyle::Pummel);
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &scurrius);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Crush], 24593);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Crush], 49);
}

#[rstest]
fn test_max_melee_with_salve_ei(mut max_melee_player: Player, vorkath: Monster) {
    max_melee_player.equip("Salve amulet (ei)");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vorkath);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 37548);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 63);
}

#[rstest]
fn test_max_melee_with_salve_i(mut max_melee_player: Player, vorkath: Monster) {
    max_melee_player.equip("Salve amulet (i)");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vorkath);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 36505);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 61);
}

#[rstest]
fn test_mid_level_melee_with_salve_ei(mut mid_level_melee_player: Player, vorkath: Monster) {
    mid_level_melee_player.equip("Salve amulet (ei)");
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &vorkath);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Slash], 27450);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Slash], 45);
}

#[rstest]
fn test_mid_level_melee_with_salve_i(mut mid_level_melee_player: Player, vorkath: Monster) {
    mid_level_melee_player.equip("Salve amulet (i)");
    mid_level_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut mid_level_melee_player, &vorkath);

    assert_eq!(mid_level_melee_player.att_rolls[&CombatType::Slash], 26687);
    assert_eq!(mid_level_melee_player.max_hits[&CombatType::Slash], 44);
}

#[rstest]
fn test_max_melee_dhl_salve_vorkath(mut max_melee_player: Player, vorkath: Monster) {
    max_melee_player.equip("Dragon hunter lance");
    max_melee_player.equip("Salve amulet (ei)");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vorkath);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 43416);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 68);
}

#[rstest]
fn test_max_melee_ursine_chainmace_salve_vetion(mut max_melee_player: Player, vetion: Monster) {
    max_melee_player.equip("Ursine chainmace");
    max_melee_player.equip("Salve amulet (ei)");
    max_melee_player.set_active_style(CombatStyle::Pummel);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vetion);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 49617);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 87);
}

#[rstest]
#[case("Aberrant spectre", (32899, 68))]
#[case("Abhorrent spectre", (32899, 70))]
#[case("Pestilent Bloat (Normal)", (32899, 74))]
fn test_colossal_blade_salve(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Colossal blade");
    max_melee_player.equip("Salve amulet (ei)");
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

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
fn test_max_melee_dhl_slayer_and_salve_vorkath(mut max_melee_player: Player, vorkath: Monster) {
    max_melee_player.equip("Dragon hunter lance");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.equip("Salve amulet (ei)");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &vorkath);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 43416);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 66);
}

#[rstest]
fn test_avarice_forinthry_against_revenant(mut max_melee_player: Player, revenant_dragon: Monster) {
    max_melee_player.equip("Amulet of avarice");
    max_melee_player.boosts.forinthry_surge = true;
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &revenant_dragon);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 44253);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 74);
}

#[rstest]
fn test_avarice_forinthry_colossal_blade_against_revenant(
    mut max_melee_player: Player,
    revenant_dragon: Monster,
) {
    max_melee_player.equip("Amulet of avarice");
    max_melee_player.equip("Colossal blade");
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.boosts.forinthry_surge = true;
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &revenant_dragon);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Slash], 39023);
    assert_eq!(max_melee_player.max_hits[&CombatType::Slash], 84);
}

#[rstest]
fn test_avarice_forinthry_ursine_chainmace_against_revenant(
    mut max_melee_player: Player,
    revenant_dragon: Monster,
) {
    max_melee_player.equip("Amulet of avarice");
    max_melee_player.equip("Ursine chainmace");
    max_melee_player.set_active_style(CombatStyle::Pummel);
    max_melee_player.boosts.forinthry_surge = true;
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &revenant_dragon);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Crush], 58836);
    assert_eq!(max_melee_player.max_hits[&CombatType::Crush], 102);
}

#[rstest]
fn test_full_obsidian_with_sword(mut max_melee_player: Player, ammonite_crab: Monster) {
    max_melee_player.equip("Obsidian helmet");
    max_melee_player.equip("Obsidian platebody");
    max_melee_player.equip("Obsidian platelegs");
    max_melee_player.equip("Toktz-xil-ak");
    max_melee_player.set_active_style(CombatStyle::Lunge);
    max_melee_player.update_bonuses();
    max_melee_player.update_set_effects();
    calc_player_melee_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 29174);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 47);
}
#[rstest]

fn test_full_obsidian_with_sword_and_necklace(
    mut max_melee_player: Player,
    ammonite_crab: Monster,
) {
    max_melee_player.equip("Obsidian helmet");
    max_melee_player.equip("Obsidian platebody");
    max_melee_player.equip("Obsidian platelegs");
    max_melee_player.equip("Toktz-xil-ak");
    max_melee_player.equip("Berserker necklace");
    max_melee_player.set_active_style(CombatStyle::Lunge);
    max_melee_player.update_bonuses();
    max_melee_player.update_set_effects();
    calc_player_melee_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 25076);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 55);
}

#[rstest]
fn test_full_obsidian_with_sword_and_salve(mut max_melee_player: Player, vorkath: Monster) {
    max_melee_player.equip("Obsidian helmet");
    max_melee_player.equip("Obsidian platebody");
    max_melee_player.equip("Obsidian platelegs");
    max_melee_player.equip("Toktz-xil-ak");
    max_melee_player.equip("Salve amulet (ei)");
    max_melee_player.set_active_style(CombatStyle::Lunge);
    max_melee_player.update_bonuses();
    max_melee_player.update_set_effects();
    calc_player_melee_rolls(&mut max_melee_player, &vorkath);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Stab], 31572);
    assert_eq!(max_melee_player.max_hits[&CombatType::Stab], 53);
}

#[rstest]
#[case("Count Draynor (Hard)", (25032, 39))]
#[case("Vampyre Juvinate (Level 50)", (25032, 36))]
#[case("Vanstrom Klause", (0, 0))]
fn test_arclight_against_vampyres(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Arclight");
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

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
#[case("Count Draynor (Hard)", (33525, 58))]
#[case("Vampyre Juvinate (Level 50)", (33525, 53))] // 10-damage limit applied post-roll
#[case("Vanstrom Klause", (0, 0))]
fn test_rapier_and_efaritays_aid_against_vampyres(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Efaritay's aid");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

    assert_eq!(
        max_melee_player.att_rolls[&CombatType::Stab],
        expected_rolls.0
    );
    assert_eq!(
        max_melee_player.max_hits[&CombatType::Stab],
        expected_rolls.1
    );
}

#[rstest]
#[case("Count Draynor (Hard)", (33525, 56))]
#[case("Vampyre Juvinate (Level 50)", (0, 0))]
#[case("Vanstrom Klause", (0, 0))]
fn test_rapier_without_efaritays_aid_against_vampyres(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

    assert_eq!(
        max_melee_player.att_rolls[&CombatType::Stab],
        expected_rolls.0
    );
    assert_eq!(
        max_melee_player.max_hits[&CombatType::Stab],
        expected_rolls.1
    );
}

#[rstest]
#[case("Count Draynor (Hard)", (30820, 56))]
#[case("Vampyre Juvinate (Level 50)", (30820, 56))]
#[case("Vanstrom Klause", (30820, 56))]
fn test_blisterwood_flail_against_vampyres(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Blisterwood flail");
    max_melee_player.set_active_style(CombatStyle::Pound);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

    assert_eq!(
        max_melee_player.att_rolls[&CombatType::Crush],
        expected_rolls.0
    );
    assert_eq!(
        max_melee_player.max_hits[&CombatType::Crush],
        expected_rolls.1
    );
}

#[rstest]
#[case("Count Draynor (Hard)", (29204, 42))]
#[case("Vampyre Juvinate (Level 50)", (29204, 39))]
#[case("Vanstrom Klause", (0, 0))]
fn test_arclight_against_vampyres_slayer(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Arclight");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.set_active_style(CombatStyle::Slash);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

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
#[case("Count Draynor (Hard)", (39112, 64))]
#[case("Vampyre Juvinate (Level 50)", (39112, 59))] // 10-damage limit applied post-roll
#[case("Vanstrom Klause", (0, 0))]
fn test_rapier_and_efaritays_aid_against_vampyres_slayer(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Efaritay's aid");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

    assert_eq!(
        max_melee_player.att_rolls[&CombatType::Stab],
        expected_rolls.0
    );
    assert_eq!(
        max_melee_player.max_hits[&CombatType::Stab],
        expected_rolls.1
    );
}

#[rstest]
#[case("Count Draynor (Hard)", (35957, 62))]
#[case("Vampyre Juvinate (Level 50)", (35957, 62))]
#[case("Vanstrom Klause", (35957, 62))]
fn test_blisterwood_flail_against_vampyres_slayer(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_melee_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_melee_player.equip("Blisterwood flail");
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.set_active_style(CombatStyle::Pound);
    max_melee_player.update_bonuses();
    calc_player_melee_rolls(&mut max_melee_player, &monster);

    assert_eq!(
        max_melee_player.att_rolls[&CombatType::Crush],
        expected_rolls.0
    );
    assert_eq!(
        max_melee_player.max_hits[&CombatType::Crush],
        expected_rolls.1
    );
}

#[rstest]
fn test_max_ranged_zcb(mut max_ranged_player: Player, ammonite_crab: Monster) {
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 50694);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 49);
}

#[rstest]
fn test_mid_level_ranged_rcb(mut mid_level_ranged_player: Player, ammonite_crab: Monster) {
    calc_player_ranged_rolls(&mut mid_level_ranged_player, &ammonite_crab);

    assert_eq!(
        mid_level_ranged_player.att_rolls[&CombatType::Ranged],
        29945
    );
    assert_eq!(mid_level_ranged_player.max_hits[&CombatType::Ranged], 30);
}

#[rstest]
fn test_max_ranged_blowpipe_dragon_darts(mut max_ranged_player: Player, ammonite_crab: Monster) {
    max_ranged_player.equip("Toxic blowpipe (dragon)");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 35358);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 31);
}

#[rstest]
#[case("Ammonite Crab", (16983, 19))]
#[case("General Graardor", (36089, 43))]
#[case("K'ril Tsutsaroth", (54770, 70))]
#[case("Commander Zilyana", (59441, 79))]
fn test_max_ranged_tbow(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_ranged_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_ranged_player.equip("Twisted bow");
    max_ranged_player.gear.ammo = None;
    max_ranged_player.equip("Dragon arrow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &monster);

    assert_eq!(
        max_ranged_player.att_rolls[&CombatType::Ranged],
        expected_rolls.0
    );
    assert_eq!(
        max_ranged_player.max_hits[&CombatType::Ranged],
        expected_rolls.1
    );
}

#[rstest]
#[case("Lizardman shaman (Chambers of Xeric) (Normal)", (48174, 60))]
#[case("Abyssal portal (Normal)", (55446, 71))]
#[case("Skeletal Mystic (Normal)", (50447, 63))]
#[case("Great Olm (Head)", (63627, 86))]
#[case("Great Olm (Head (Challenge Mode))", (63627, 99))]
fn test_max_ranged_tbow_cox(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_ranged_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_ranged_player.equip("Twisted bow");
    max_ranged_player.gear.ammo = None;
    max_ranged_player.equip("Dragon arrow");
    max_ranged_player.potions.ranged = Some(PotionBoost::new(Potion::OverloadPlus));
    max_ranged_player.calc_potion_boosts();
    max_ranged_player.reset_live_stats();
    max_ranged_player.update_bonuses();

    calc_player_ranged_rolls(&mut max_ranged_player, &monster);

    assert_eq!(
        max_ranged_player.att_rolls[&CombatType::Ranged],
        expected_rolls.0
    );
    assert_eq!(
        max_ranged_player.max_hits[&CombatType::Ranged],
        expected_rolls.1
    );
}

#[rstest]
fn test_max_ranged_tbow_zebak_400(mut max_ranged_player: Player, mut zebak: Monster) {
    zebak.info.toa_level = 400;
    zebak.scale_toa();

    max_ranged_player.equip("Twisted bow");
    max_ranged_player.gear.ammo = None;
    max_ranged_player.equip("Dragon arrow");
    max_ranged_player.potions.ranged = Some(PotionBoost::new(Potion::SmellingSalts));
    max_ranged_player.calc_potion_boosts();
    max_ranged_player.reset_live_stats();
    max_ranged_player.update_bonuses();

    calc_player_ranged_rolls(&mut max_ranged_player, &zebak);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 63304);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 83);
}

#[rstest]
fn test_dhcb_vorkath(mut max_ranged_player: Player, vorkath: Monster) {
    max_ranged_player.equip("Dragon hunter crossbow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &vorkath);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 63133);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 61);
}

#[rstest]
fn test_webweaver_spindel(mut max_ranged_player: Player, spindel: Monster) {
    max_ranged_player.equip("Webweaver bow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &spindel);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 64752);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 51);
}

#[rstest]
fn test_webweaver_non_wildy_monster(mut max_ranged_player: Player, ammonite_crab: Monster) {
    max_ranged_player.equip("Webweaver bow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 43168);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 34);
}

#[rstest]
fn test_full_eclipse_atlatl_ranged_gear(mut max_ranged_player: Player, ammonite_crab: Monster) {
    max_ranged_player.equip("Eclipse moon helm");
    max_ranged_player.equip("Eclipse moon chestplate");
    max_ranged_player.equip("Eclipse moon tassets");
    max_ranged_player.equip("Eclipse atlatl");
    max_ranged_player.equip("Atlatl dart");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 39760);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 22);
}

#[rstest]
fn test_full_eclipse_atlatl_melee_gear_rigour_all_pots(
    mut max_melee_player: Player,
    ammonite_crab: Monster,
) {
    max_melee_player.equip("Eclipse moon helm");
    max_melee_player.equip("Eclipse moon chestplate");
    max_melee_player.equip("Eclipse moon tassets");
    max_melee_player.equip("Eclipse atlatl");
    max_melee_player.equip("Atlatl dart");
    max_melee_player.update_bonuses();
    max_melee_player.set_active_style(CombatStyle::Rapid);
    max_melee_player
        .prayers
        .add(PrayerBoost::new(Prayer::Rigour));
    max_melee_player.potions.ranged = Some(PotionBoost::new(Potion::Ranging));
    max_melee_player.calc_potion_boosts();
    max_melee_player.reset_live_stats();
    calc_player_ranged_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Ranged], 27122);
    assert_eq!(max_melee_player.max_hits[&CombatType::Ranged], 38);

    max_melee_player.stats.strength = 80;
    max_melee_player.calc_potion_boosts();
    max_melee_player.reset_live_stats();

    calc_player_ranged_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Ranged], 27122);
    assert_eq!(max_melee_player.max_hits[&CombatType::Ranged], 32);
}
