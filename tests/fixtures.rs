use osrs::equipment::{Armor, CombatStyle, Weapon};
use osrs::monster::Monster;
use osrs::player::{Gear, Player, PlayerStats};
use osrs::potions::{Potion, PotionBoost};
use osrs::prayers::{Prayer, PrayerBoost};
use rstest::fixture;

#[fixture]
pub fn vorkath() -> Monster {
    Monster::new("Vorkath").unwrap()
}

#[fixture]
pub fn kril() -> Monster {
    Monster::new("K'ril Tsutsaroth").unwrap()
}

#[fixture]
pub fn kalphite() -> Monster {
    Monster::new("Kalphite Soldier").unwrap()
}

#[fixture]
pub fn ammonite_crab() -> Monster {
    Monster::new("Ammonite Crab").unwrap()
}

#[fixture]
pub fn vetion() -> Monster {
    Monster::new("Vet'ion (Normal)").unwrap()
}

#[fixture]
pub fn spindel() -> Monster {
    Monster::new("Spindel").unwrap()
}

#[fixture]
pub fn duke() -> Monster {
    Monster::new("Duke Sucellus (Awake)").unwrap()
}

#[fixture]
pub fn kurask() -> Monster {
    Monster::new("Kurask (Normal)").unwrap()
}

#[fixture]
pub fn scurrius() -> Monster {
    Monster::new("Scurrius (Solo)").unwrap()
}

#[fixture]
pub fn revenant_dragon() -> Monster {
    Monster::new("Revenant dragon").unwrap()
}

#[fixture]
pub fn zebak() -> Monster {
    Monster::new("Zebak").unwrap()
}

#[fixture]
pub fn chaos_golem() -> Monster {
    Monster::new("Chaos Golem").unwrap()
}

#[fixture]
pub fn aberrant_spectre() -> Monster {
    Monster::new("Aberrant spectre").unwrap()
}

#[fixture]
pub fn abhorrent_spectre() -> Monster {
    Monster::new("Abhorrent spectre").unwrap()
}

#[fixture]
pub fn general_graardor() -> Monster {
    Monster::new("General Graardor").unwrap()
}

#[fixture]
pub fn rune_dragon() -> Monster {
    Monster::new("Rune dragon").unwrap()
}

#[fixture]
pub fn bloat() -> Monster {
    Monster::new("Pestilent Bloat (Normal)").unwrap()
}

#[fixture]
pub fn count_draynor() -> Monster {
    Monster::new("Count Draynor (Hard)").unwrap()
}

#[fixture]
pub fn vampyre_juvinate() -> Monster {
    Monster::new("Vampyre Juvinate (Level 50)").unwrap()
}

#[fixture]
pub fn vanstrom_klause() -> Monster {
    Monster::new("Vanstrom Klause").unwrap()
}

#[fixture]
pub fn max_melee_player() -> Player {
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
pub fn mid_level_melee_player() -> Player {
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
pub fn max_ranged_player() -> Player {
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
pub fn mid_level_ranged_player() -> Player {
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

#[fixture]
pub fn max_melee_dhl_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dragon hunter lance");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_keris_partisan_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Keris partisan");
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blue_keris_partisan_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Keris partisan of breaching");
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_barronite_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Barronite mace");
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_ursine_chainmace_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Ursine chainmace");
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_berserker_neck_obby_sword_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Berserker necklace");
    player.equip("Toktz-xil-ak");
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_silverlight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Silverlight");
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_darklight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Darklight");
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_arclight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Arclight");
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_lbba_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Leaf-bladed battleaxe");
    player.set_active_style(CombatStyle::Hack);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_colossal_blade_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Colossal blade");
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_bone_mace_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Bone mace");
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_obby_with_sword_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Obsidian helmet");
    player.equip("Obsidian platebody");
    player.equip("Obsidian platelegs");
    player.equip("Toktz-xil-ak");
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player.update_set_effects();
    player
}

#[fixture]
pub fn full_obby_with_sword_and_necklace_player() -> Player {
    let mut player = full_obby_with_sword_player();
    player.equip("Berserker necklace");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blisterwood_flail_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Blisterwood flail");
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

pub fn slayer(mut player: Player) -> Player {
    player.equip("Slayer helmet (i)");
    player.update_bonuses();
    player
}

pub fn salve_ei(mut player: Player) -> Player {
    player.equip("Salve amulet (ei)");
    player.update_bonuses();
    player
}

pub fn salve_i(mut player: Player) -> Player {
    player.equip("Salve amulet (i)");
    player.update_bonuses();
    player
}

pub fn avarice_forinthry(mut player: Player) -> Player {
    player.equip("Amulet of avarice");
    player.boosts.forinthry_surge = true;
    player.update_bonuses();
    player
}

pub fn efaritays_aid(mut player: Player) -> Player {
    player.equip("Efaritay's aid");
    player.update_bonuses();
    player
}
