use osrs::equipment::{Armor, CombatStyle, Weapon};
use osrs::monster::Monster;
use osrs::player::{Gear, Player, PlayerStats};
use osrs::potions::Potion;
use osrs::prayers::{Prayer, PrayerBoost};
use osrs::spells::{AncientSpell, ArceuusSpell, Spell, StandardSpell};
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
pub fn zilyana() -> Monster {
    Monster::new("Commander Zilyana").unwrap()
}

#[fixture]
pub fn shaman_cox() -> Monster {
    Monster::new("Lizardman shaman (Chambers of Xeric) (Normal)").unwrap()
}

#[fixture]
pub fn abyssal_portal() -> Monster {
    Monster::new("Abyssal portal (Normal)").unwrap()
}

#[fixture]
pub fn skeletal_mystic() -> Monster {
    Monster::new("Skeletal Mystic (Normal)").unwrap()
}

#[fixture]
pub fn olm_head() -> Monster {
    Monster::new("Great Olm (Head)").unwrap()
}

#[fixture]
pub fn olm_head_cm() -> Monster {
    Monster::new("Great Olm (Head (Challenge Mode))").unwrap()
}

#[fixture]
pub fn shaman_cox_cm() -> Monster {
    Monster::new("Lizardman shaman (Chambers of Xeric) (Challenge Mode)").unwrap()
}

#[fixture]
pub fn skeletal_mystic_cm() -> Monster {
    Monster::new("Skeletal Mystic (Challenge Mode)").unwrap()
}

#[fixture]
pub fn wardens_p3() -> Monster {
    Monster::new("Elidinis' Warden (P3)").unwrap()
}

#[fixture]
pub fn vardorvis() -> Monster {
    Monster::new("Vardorvis (Post-Quest)").unwrap()
}

#[fixture]
pub fn kephri_400() -> Monster {
    let mut monster = Monster::new("Kephri (Shield up)").unwrap();
    monster.info.toa_level = 400;
    monster.scale_toa();
    monster
}

#[fixture]
pub fn urium_shade() -> Monster {
    Monster::new("Urium Shade (Shade)").unwrap()
}

#[fixture]
pub fn kalphite_queen_p1() -> Monster {
    Monster::new("Kalphite Queen (Crawling)").unwrap()
}

#[fixture]
pub fn zulrah_tanzanite() -> Monster {
    Monster::new("Zulrah (Tanzanite)").unwrap()
}

#[fixture]
pub fn zulrah_magma() -> Monster {
    Monster::new("Zulrah (Magma)").unwrap()
}

#[fixture]
pub fn seren() -> Monster {
    Monster::new("Fragment of Seren").unwrap()
}

#[fixture]
pub fn kraken() -> Monster {
    Monster::new("Kraken (Kraken)").unwrap()
}

#[fixture]
pub fn verzik_p1() -> Monster {
    Monster::new("Verzik Vitur (Normal P1)").unwrap()
}

#[fixture]
pub fn tekton() -> Monster {
    Monster::new("Tekton (Normal)").unwrap()
}

#[fixture]
pub fn vasa_crystal() -> Monster {
    Monster::new("Glowing crystal (Normal)").unwrap()
}

#[fixture]
pub fn olm_melee_hand() -> Monster {
    Monster::new("Great Olm (Left claw)").unwrap()
}

#[fixture]
pub fn olm_mage_hand() -> Monster {
    Monster::new("Great Olm (Right claw)").unwrap()
}

#[fixture]
pub fn ice_demon() -> Monster {
    Monster::new("Ice demon (Normal)").unwrap()
}

#[fixture]
pub fn slagilith() -> Monster {
    Monster::new("Slagilith (Hard)").unwrap()
}

#[fixture]
pub fn zogre() -> Monster {
    Monster::new("Zogre").unwrap()
}

#[fixture]
pub fn max_melee_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.prayers.add(PrayerBoost::new(Prayer::Piety));
    player.add_potion(Potion::SuperCombat);

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
        mining: 70,
    };
    player.prayers.add(PrayerBoost::new(Prayer::Piety));
    player.add_potion(Potion::SuperCombat);

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
pub fn max_ranged_zcb_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));
    player.add_potion(Potion::Ranging);

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
pub fn mid_level_ranged_rcb_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats {
        attack: 80,
        strength: 80,
        defence: 80,
        ranged: 80,
        magic: 80,
        hitpoints: 80,
        prayer: 70,
        mining: 70,
    };
    player.prayers.add(PrayerBoost::new(Prayer::EagleEye));
    player.add_potion(Potion::Ranging);

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

#[fixture]
pub fn max_ranged_blowpipe_dragon_darts_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Toxic blowpipe (dragon)");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tbow_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Twisted bow");
    player.gear.ammo = None;
    player.equip("Dragon arrow");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tbow_overload_player() -> Player {
    let mut player = max_ranged_tbow_player();
    player.add_potion(Potion::OverloadPlus);
    player
}

#[fixture]
pub fn max_ranged_tbow_salts_player() -> Player {
    let mut player = max_ranged_tbow_player();
    player.add_potion(Potion::SmellingSalts);
    player
}

#[fixture]
pub fn max_ranged_dhcb_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Dragon hunter crossbow");
    player.update_bonuses();
    player
}

#[fixture]
pub fn elite_void_dhcb_player() -> Player {
    let mut player = max_ranged_dhcb_player();
    player.equip("Elite void top");
    player.equip("Elite void robe");
    player.equip("Void knight gloves");
    player.equip("Void ranger helm");
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_webweaver_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Webweaver bow");
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_eclipse_atlatl_ranged_gear_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Eclipse atlatl");
    player.equip("Eclipse moon helm");
    player.equip("Eclipse moon chestplate");
    player.equip("Eclipse moon tassets");
    player.equip("Atlatl dart");
    player.update_bonuses();
    player
}

#[fixture]
pub fn eclipse_atlatl_ranged_gear_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Eclipse atlatl");
    player.equip("Atlatl dart");
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_eclipse_atlatl_melee_gear_rigour_all_pots() -> Player {
    let mut player = max_melee_player();
    player.equip("Eclipse atlatl");
    player.equip("Eclipse moon helm");
    player.equip("Eclipse moon chestplate");
    player.equip("Eclipse moon tassets");
    player.equip("Atlatl dart");
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));
    player.add_potion(Potion::Ranging);
    player
}

#[fixture]
pub fn full_eclipse_atlatl_melee_gear_rigour_all_pots_80_str() -> Player {
    let mut player = full_eclipse_atlatl_melee_gear_rigour_all_pots();
    player.stats.strength = 80;
    player.calc_potion_boosts();
    player.reset_live_stats();
    player
}

#[fixture]
pub fn eclipse_atlatl_melee_gear_rigour_all_pots() -> Player {
    let mut player = max_melee_player();
    player.equip("Eclipse atlatl");
    player.equip("Atlatl dart");
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));
    player.add_potion(Potion::Ranging);
    player
}

#[fixture]
pub fn mid_level_ranged_bone_shortbow_player() -> Player {
    let mut player = mid_level_ranged_rcb_player();
    player.equip("Bone shortbow");
    player.equip("Rune arrow");
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_ranged_rcb_silver_bolts_player() -> Player {
    let mut player = mid_level_ranged_rcb_player();
    player.equip("Silver bolts");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_sang_staff_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.prayers.add(PrayerBoost::new(Prayer::Augury));
    player.add_potion(Potion::SaturatedHeart);

    player.gear = Gear {
        head: Some(Armor::new("Ancestral hat")),
        neck: Some(Armor::new("Occult necklace")),
        cape: Some(Armor::new("Imbued guthix cape")),
        ammo: Some(Armor::new("Rada's blessing 4")),
        second_ammo: None,
        weapon: Weapon::new("Sanguinesti staff"),
        shield: Some(Armor::new("Elidinis' ward (f)")),
        body: Some(Armor::new("Ancestral robe top")),
        legs: Some(Armor::new("Ancestral robe bottom")),
        hands: Some(Armor::new("Tormented bracelet")),
        feet: Some(Armor::new("Eternal boots")),
        ring: Some(Armor::new("Magus ring")),
    };
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);

    player
}

#[fixture]
pub fn max_mage_sang_staff_brimstone_ring_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Brimstone ring");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_toxic_trident_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Trident of the swamp");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_trident_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Trident of the seas");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_fire_surge_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Harmonised nightmare staff");
    player.update_bonuses();
    player.set_spell(Spell::Standard(StandardSpell::FireSurge));
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn max_mage_kodai_ice_barrage_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Kodai wand");
    player.update_bonuses();
    player.set_spell(Spell::Ancient(AncientSpell::IceBarrage));
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn mid_level_magic_warped_sceptre_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats {
        attack: 80,
        strength: 80,
        defence: 80,
        ranged: 80,
        magic: 80,
        hitpoints: 80,
        prayer: 70,
        mining: 70,
    };
    player.prayers.add(PrayerBoost::new(Prayer::MysticMight));

    player.calc_potion_boosts();
    player.reset_live_stats();

    player.gear = Gear {
        head: Some(Armor::new("Ahrim's hood")),
        neck: Some(Armor::new("Occult necklace")),
        cape: Some(Armor::new("Imbued guthix cape")),
        ammo: Some(Armor::new("Rada's blessing 3")),
        second_ammo: None,
        weapon: Weapon::new("Warped sceptre"),
        shield: Some(Armor::new("Malediction ward")),
        body: Some(Armor::new("Ahrim's robetop")),
        legs: Some(Armor::new("Ahrim's robeskirt")),
        hands: Some(Armor::new("Barrows gloves")),
        feet: Some(Armor::new("Infinity boots")),
        ring: Some(Armor::new("Seers ring (i)")),
    };
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);

    player
}

#[fixture]
pub fn mid_level_mage_chaos_gauntlets_fire_bolt_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Fire battlestaff");
    player.equip("Chaos gauntlets");
    player.update_bonuses();
    player.set_spell(Spell::Standard(StandardSpell::FireBolt));
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn mid_level_mage_god_spell_charge_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Guthix staff");
    player.update_bonuses();
    player.boosts.charge_active = true;
    player.set_spell(Spell::Standard(StandardSpell::ClawsOfGuthix));
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn max_mage_shadow_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Tumeken's shadow");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_shadow_salts_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Tumeken's shadow");
    player.add_potion(Potion::SmellingSalts);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_virtus_kodai_ice_barrage_player() -> Player {
    let mut player = max_mage_kodai_ice_barrage_player();
    player.equip("Virtus mask");
    player.equip("Virtus robe top");
    player.equip("Virtus robe bottom");
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_virtus_kodai_fire_surge_player() -> Player {
    let mut player = full_virtus_kodai_ice_barrage_player();
    player.set_spell(Spell::Standard(StandardSpell::FireSurge));
    player
}

#[fixture]
pub fn max_mage_smoke_staff_fire_surge_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Smoke battlestaff");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_accursed_sceptre_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Accursed sceptre");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_tome_of_water_surge_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Tome of water (charged)");
    player.set_spell(Spell::Standard(StandardSpell::WaterSurge));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_fire_surge_tome_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Tome of fire (burnt)");
    player.update_bonuses();
    player
}
#[fixture]
pub fn mid_level_mage_chaos_gauntlets_fire_bolt_tome_player() -> Player {
    let mut player = mid_level_mage_chaos_gauntlets_fire_bolt_player();
    player.equip("Tome of fire (burnt)");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blade_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Blade of saeldor (c)");
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_scythe_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Scythe of vitur");
    player.set_active_style(CombatStyle::Chop);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_fang_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Osmumten's fang");
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_ahrims_aotd_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Ahrim's staff");
    player.equip("Amulet of the damned");
    player.attrs.spell = Some(Spell::Arceuus(ArceuusSpell::UndeadGrasp));
    player.set_active_style(CombatStyle::Spell);
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_ahrims_aotd_sunfire_player() -> Player {
    let mut player = full_ahrims_aotd_player();
    player.set_spell(Spell::Standard(StandardSpell::FireSurge));
    player.boosts.sunfire_runes = true;
    player
}

#[fixture]
pub fn full_dharoks_1hp_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dharok's helm");
    player.equip("Dharok's platebody");
    player.equip("Dharok's platelegs");
    player.equip("Dharok's greataxe");
    player.set_active_style(CombatStyle::Hack);
    player.update_set_effects();
    player.update_bonuses();
    player.live_stats.hitpoints = 1;
    player
}

#[fixture]
pub fn full_veracs_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Verac's helm");
    player.equip("Verac's plateskirt");
    player.equip("Verac's brassard");
    player.equip("Verac's flail");
    player.set_active_style(CombatStyle::Pummel);
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_karils_aotd_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Karil's coif");
    player.equip("Karil's leathertop");
    player.equip("Karil's leatherskirt");
    player.equip("Karil's crossbow");
    player.equip("Amulet of the damned");
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_torags_hammers_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Torag's hammers");
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tonalztics_charged_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Tonalztics of Ralos (charged)");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tonalztics_uncharged_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Tonalztics of Ralos (uncharged)");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_macuahuitl_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dual macuahuitl");
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_zcb_ruby_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Ruby dragon bolts (e)");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_dawnbringer_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Dawnbringer");
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_crumble_undead() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.set_spell(Spell::Standard(StandardSpell::CrumbleUndead));
    player
}

#[fixture]
pub fn max_range_comp_ogre_bow_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Comp ogre bow");
    player.equip("Rune brutal");
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

pub fn scale_toa(mut monster: Monster, toa_level: u32) -> Monster {
    monster.info.toa_level = toa_level;
    monster.scale_toa();
    monster
}
