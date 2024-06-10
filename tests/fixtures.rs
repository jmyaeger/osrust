use osrs::equipment::CombatStyle;
use osrs::monster::Monster;
use osrs::player::{Player, PlayerStats};
use osrs::potions::Potion;
use osrs::prayers::{Prayer, PrayerBoost};
use osrs::spells::{AncientSpell, ArceuusSpell, Spell, StandardSpell};
use rstest::fixture;

#[fixture]
pub fn vorkath() -> Monster {
    Monster::new("Vorkath", Some("Post-quest")).unwrap()
}

#[fixture]
pub fn kril() -> Monster {
    Monster::new("K'ril Tsutsaroth", None).unwrap()
}

#[fixture]
pub fn kalphite() -> Monster {
    Monster::new("Kalphite Soldier", Some("Kalphite Lair")).unwrap()
}

#[fixture]
pub fn ammonite_crab() -> Monster {
    Monster::new("Ammonite Crab", None).unwrap()
}

#[fixture]
pub fn vetion() -> Monster {
    Monster::new("Vet'ion", Some("Normal")).unwrap()
}

#[fixture]
pub fn spindel() -> Monster {
    Monster::new("Spindel", None).unwrap()
}

#[fixture]
pub fn duke() -> Monster {
    Monster::new("Duke Sucellus", Some("Post-Quest, Awake")).unwrap()
}

#[fixture]
pub fn kurask() -> Monster {
    Monster::new("Kurask", Some("Normal")).unwrap()
}

#[fixture]
pub fn scurrius() -> Monster {
    Monster::new("Scurrius", Some("Solo")).unwrap()
}

#[fixture]
pub fn revenant_dragon() -> Monster {
    Monster::new("Revenant dragon", None).unwrap()
}

#[fixture]
pub fn zebak() -> Monster {
    Monster::new("Zebak", None).unwrap()
}

#[fixture]
pub fn chaos_golem() -> Monster {
    Monster::new("Chaos Golem", Some("Golem")).unwrap()
}

#[fixture]
pub fn aberrant_spectre() -> Monster {
    Monster::new("Aberrant spectre", None).unwrap()
}

#[fixture]
pub fn abhorrent_spectre() -> Monster {
    Monster::new("Abhorrent spectre", None).unwrap()
}

#[fixture]
pub fn general_graardor() -> Monster {
    Monster::new("General Graardor", None).unwrap()
}

#[fixture]
pub fn rune_dragon() -> Monster {
    Monster::new("Rune dragon", None).unwrap()
}

#[fixture]
pub fn bloat() -> Monster {
    Monster::new("Pestilent Bloat", Some("Normal")).unwrap()
}

#[fixture]
pub fn count_draynor() -> Monster {
    Monster::new("Count Draynor", Some("Hard")).unwrap()
}

#[fixture]
pub fn vampyre_juvinate() -> Monster {
    Monster::new("Vampyre Juvinate", Some("Level 50")).unwrap()
}

#[fixture]
pub fn vanstrom_klause() -> Monster {
    Monster::new("Vanstrom Klause", Some("Sins of the Father")).unwrap()
}

#[fixture]
pub fn zilyana() -> Monster {
    Monster::new("Commander Zilyana", None).unwrap()
}

#[fixture]
pub fn shaman_cox() -> Monster {
    Monster::new("Lizardman shaman (Chambers of Xeric)", Some("Normal")).unwrap()
}

#[fixture]
pub fn abyssal_portal() -> Monster {
    Monster::new("Abyssal portal", Some("Normal")).unwrap()
}

#[fixture]
pub fn skeletal_mystic() -> Monster {
    Monster::new("Skeletal Mystic", Some("Normal")).unwrap()
}

#[fixture]
pub fn olm_head() -> Monster {
    Monster::new("Great Olm", Some("Head")).unwrap()
}

#[fixture]
pub fn olm_head_cm() -> Monster {
    Monster::new("Great Olm", Some("Head (Challenge Mode)")).unwrap()
}

#[fixture]
pub fn shaman_cox_cm() -> Monster {
    Monster::new(
        "Lizardman shaman (Chambers of Xeric)",
        Some("Challenge Mode"),
    )
    .unwrap()
}

#[fixture]
pub fn skeletal_mystic_cm() -> Monster {
    Monster::new("Skeletal Mystic", Some("Challenge Mode")).unwrap()
}

#[fixture]
pub fn wardens_p3() -> Monster {
    Monster::new("Elidinis' Warden", Some("Damaged")).unwrap()
}

#[fixture]
pub fn vardorvis() -> Monster {
    Monster::new("Vardorvis", Some("Post-Quest")).unwrap()
}

#[fixture]
pub fn kephri_400() -> Monster {
    let mut monster = Monster::new("Kephri", None).unwrap();
    monster.info.toa_level = 400;
    monster.scale_toa();
    monster
}

#[fixture]
pub fn urium_shade() -> Monster {
    Monster::new("Urium Shade", Some("Shade")).unwrap()
}

#[fixture]
pub fn kalphite_queen_p1() -> Monster {
    Monster::new("Kalphite Queen", Some("Crawling")).unwrap()
}

#[fixture]
pub fn zulrah_tanzanite() -> Monster {
    Monster::new("Zulrah", Some("Tanzanite")).unwrap()
}

#[fixture]
pub fn zulrah_magma() -> Monster {
    Monster::new("Zulrah", Some("Magma")).unwrap()
}

#[fixture]
pub fn seren() -> Monster {
    Monster::new("Fragment of Seren", None).unwrap()
}

#[fixture]
pub fn kraken() -> Monster {
    Monster::new("Kraken", Some("Kraken")).unwrap()
}

#[fixture]
pub fn verzik_p1() -> Monster {
    Monster::new("Verzik Vitur", Some("Normal mode, Phase 1")).unwrap()
}

#[fixture]
pub fn tekton() -> Monster {
    Monster::new("Tekton", Some("Normal")).unwrap()
}

#[fixture]
pub fn vasa_crystal() -> Monster {
    Monster::new("Glowing crystal", Some("Normal")).unwrap()
}

#[fixture]
pub fn olm_melee_hand() -> Monster {
    Monster::new("Great Olm", Some("Left claw")).unwrap()
}

#[fixture]
pub fn olm_mage_hand() -> Monster {
    Monster::new("Great Olm", Some("Right claw")).unwrap()
}

#[fixture]
pub fn ice_demon() -> Monster {
    Monster::new("Ice demon", Some("Normal")).unwrap()
}

#[fixture]
pub fn slagilith() -> Monster {
    Monster::new("Slagilith", Some("Hard")).unwrap()
}

#[fixture]
pub fn zogre() -> Monster {
    Monster::new("Zogre", None).unwrap()
}

#[fixture]
pub fn max_melee_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.prayers.add(PrayerBoost::new(Prayer::Piety));
    player.add_potion(Potion::SuperCombat);

    player.equip("Torva full helm", None);
    player.equip("Torva platebody", None);
    player.equip("Torva platelegs", None);
    player.equip("Ferocious gloves", None);
    player.equip("Primordial boots", None);
    player.equip("Ghrazi rapier", None);
    player.equip("Avernic defender", None);
    player.equip("Rada's blessing 4", None);
    player.equip("Amulet of torture", None);
    player.equip("Infernal cape", None);
    player.equip("Ultor ring", None);

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

    player.equip("Helm of neitiznot", None);
    player.equip("Amulet of fury", None);
    player.equip("Fire cape", None);
    player.equip("Rada's blessing 3", None);
    player.equip("Abyssal whip", None);
    player.equip("Dragon defender", None);
    player.equip("Fighter torso", None);
    player.equip("Obsidian platelegs", None);
    player.equip("Barrows gloves", None);
    player.equip("Dragon boots", None);
    player.equip("Berserker ring (i)", None);

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

    player.equip("Masori mask (f)", None);
    player.equip("Necklace of anguish", None);
    player.equip("Dizana's quiver", Some("Charged"));
    player.equip("Dragon bolts", Some("Unpoisoned"));
    player.equip("Zaryte crossbow", None);
    player.equip("Twisted buckler", None);
    player.equip("Masori body (f)", None);
    player.equip("Masori chaps (f)", None);
    player.equip("Zaryte vambraces", None);
    player.equip("Pegasian boots", None);
    player.equip("Venator ring", None);

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

    player.equip("Ancient coif", None);
    player.equip("Amulet of fury", None);
    player.equip("Ava's assembler", None);
    player.equip("Adamant bolts", Some("Unpoisoned"));
    player.equip("Rune crossbow", None);
    player.equip("Odium ward", None);
    player.equip("Ancient d'hide body", None);
    player.equip("Ancient chaps", None);
    player.equip("Barrows gloves", None);
    player.equip("Ancient d'hide boots", None);
    player.equip("Archers ring (i)", None);

    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);

    player
}

#[fixture]
pub fn max_melee_dhl_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dragon hunter lance", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_keris_partisan_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Keris partisan", None);
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blue_keris_partisan_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Keris partisan of breaching", None);
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_barronite_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Barronite mace", None);
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_ursine_chainmace_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Ursine chainmace", Some("Charged"));
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_berserker_neck_obby_sword_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Berserker necklace", None);
    player.equip("Toktz-xil-ak", None);
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_silverlight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Silverlight", None);
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_darklight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Darklight", None);
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_arclight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Arclight", None);
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_lbba_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Leaf-bladed battleaxe", None);
    player.set_active_style(CombatStyle::Hack);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_colossal_blade_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Colossal blade", None);
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_bone_mace_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Bone mace", None);
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_obby_with_sword_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Obsidian helmet", None);
    player.equip("Obsidian platebody", None);
    player.equip("Obsidian platelegs", None);
    player.equip("Toktz-xil-ak", None);
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player.update_set_effects();
    player
}

#[fixture]
pub fn full_obby_with_sword_and_necklace_player() -> Player {
    let mut player = full_obby_with_sword_player();
    player.equip("Berserker necklace", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blisterwood_flail_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Blisterwood flail", None);
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_blowpipe_dragon_darts_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Toxic blowpipe", Some("Dragon"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tbow_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Twisted bow", None);
    player.gear.ammo = None;
    player.equip("Dragon arrow", Some("Unpoisoned"));
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
    player.equip("Dragon hunter crossbow", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn elite_void_dhcb_player() -> Player {
    let mut player = max_ranged_dhcb_player();
    player.equip("Elite void top", None);
    player.equip("Elite void robe", None);
    player.equip("Void knight gloves", None);
    player.equip("Void ranger helm", None);
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_webweaver_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Webweaver bow", Some("Charged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_eclipse_atlatl_ranged_gear_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Eclipse atlatl", None);
    player.equip("Eclipse moon helm", Some("New"));
    player.equip("Eclipse moon chestplate", Some("New"));
    player.equip("Eclipse moon tassets", Some("New"));
    player.equip("Atlatl dart", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn eclipse_atlatl_ranged_gear_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Eclipse atlatl", None);
    player.equip("Atlatl dart", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_eclipse_atlatl_melee_gear_rigour_all_pots() -> Player {
    let mut player = max_melee_player();
    player.equip("Eclipse atlatl", None);
    player.equip("Eclipse moon helm", Some("New"));
    player.equip("Eclipse moon chestplate", Some("New"));
    player.equip("Eclipse moon tassets", Some("New"));
    player.equip("Atlatl dart", None);
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
    player.equip("Eclipse atlatl", None);
    player.equip("Atlatl dart", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));
    player.add_potion(Potion::Ranging);
    player
}

#[fixture]
pub fn mid_level_ranged_bone_shortbow_player() -> Player {
    let mut player = mid_level_ranged_rcb_player();
    player.equip("Bone shortbow", None);
    player.equip("Rune arrow", Some("Unpoisoned"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_ranged_rcb_silver_bolts_player() -> Player {
    let mut player = mid_level_ranged_rcb_player();
    player.equip("Silver bolts", Some("Unpoisoned"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_sang_staff_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.prayers.add(PrayerBoost::new(Prayer::Augury));
    player.add_potion(Potion::SaturatedHeart);

    player.equip("Ancestral hat", None);
    player.equip("Occult necklace", None);
    player.equip("Imbued guthix cape", None);
    player.equip("Rada's blessing 4", None);
    player.equip("Sanguinesti staff", Some("Charged"));
    player.equip("Elidinis' ward (f)", None);
    player.equip("Ancestral robe top", None);
    player.equip("Ancestral robe bottom", None);
    player.equip("Tormented bracelet", None);
    player.equip("Eternal boots", None);
    player.equip("Magus ring", None);

    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);

    player
}

#[fixture]
pub fn max_mage_sang_staff_brimstone_ring_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Brimstone ring", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_toxic_trident_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Trident of the swamp", Some("Charged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_trident_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Trident of the seas", Some("Charged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_fire_surge_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Harmonised nightmare staff", None);
    player.update_bonuses();
    player.set_spell(Spell::Standard(StandardSpell::FireSurge));
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn max_mage_kodai_ice_barrage_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Kodai wand", None);
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

    player.equip("Ahrim's hood", None);
    player.equip("Occult necklace", None);
    player.equip("Imbued guthix cape", None);
    player.equip("Rada's blessing 3", None);
    player.equip("Warped sceptre", Some("Charged"));
    player.equip("Malediction ward", None);
    player.equip("Ahrim's robetop", None);
    player.equip("Ahrim's robeskirt", None);
    player.equip("Barrows gloves", None);
    player.equip("Infinity boots", None);
    player.equip("Seers ring (i)", None);

    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);

    player
}

#[fixture]
pub fn mid_level_mage_chaos_gauntlets_fire_bolt_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Fire battlestaff", None);
    player.equip("Chaos gauntlets", None);
    player.update_bonuses();
    player.set_spell(Spell::Standard(StandardSpell::FireBolt));
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn mid_level_mage_god_spell_charge_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Guthix staff", None);
    player.update_bonuses();
    player.boosts.charge_active = true;
    player.set_spell(Spell::Standard(StandardSpell::ClawsOfGuthix));
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn max_mage_shadow_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Tumeken's shadow", Some("Charged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_shadow_salts_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Tumeken's shadow", Some("Charged"));
    player.add_potion(Potion::SmellingSalts);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_virtus_kodai_ice_barrage_player() -> Player {
    let mut player = max_mage_kodai_ice_barrage_player();
    player.equip("Virtus mask", None);
    player.equip("Virtus robe top", None);
    player.equip("Virtus robe bottom", None);
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
    player.equip("Smoke battlestaff", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_accursed_sceptre_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Accursed sceptre", Some("Charged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_tome_of_water_surge_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Tome of water", Some("Charged"));
    player.set_spell(Spell::Standard(StandardSpell::WaterSurge));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_fire_surge_tome_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Tome of fire", Some("Charged"));
    player.update_bonuses();
    player
}
#[fixture]
pub fn mid_level_mage_chaos_gauntlets_fire_bolt_tome_player() -> Player {
    let mut player = mid_level_mage_chaos_gauntlets_fire_bolt_player();
    player.equip("Tome of fire", Some("Charged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blade_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Blade of saeldor (c)", None);
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_scythe_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Scythe of vitur", Some("Charged"));
    player.set_active_style(CombatStyle::Chop);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_fang_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Osmumten's fang", None);
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_ahrims_aotd_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Ahrim's staff", None);
    player.equip("Amulet of the damned", Some("Full"));
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
    player.equip("Dharok's helm", None);
    player.equip("Dharok's platebody", None);
    player.equip("Dharok's platelegs", None);
    player.equip("Dharok's greataxe", None);
    player.set_active_style(CombatStyle::Hack);
    player.update_set_effects();
    player.update_bonuses();
    player.live_stats.hitpoints = 1;
    player
}

#[fixture]
pub fn full_veracs_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Verac's helm", None);
    player.equip("Verac's plateskirt", None);
    player.equip("Verac's brassard", None);
    player.equip("Verac's flail", None);
    player.set_active_style(CombatStyle::Pummel);
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_karils_aotd_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Karil's coif", None);
    player.equip("Karil's leathertop", None);
    player.equip("Karil's leatherskirt", None);
    player.equip("Karil's crossbow", None);
    player.equip("Amulet of the damned", None);
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_torags_hammers_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Torag's hammers", None);
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tonalztics_charged_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Tonalztics of Ralos", Some("Charged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tonalztics_uncharged_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Tonalztics of Ralos", Some("Uncharged"));
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_macuahuitl_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dual macuahuitl", None);
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_zcb_ruby_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Ruby dragon bolts (e)", None);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_dawnbringer_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Dawnbringer", None);
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
    player.equip("Comp ogre bow", None);
    player.equip("Rune brutal", None);
    player.update_bonuses();
    player
}

pub fn slayer(mut player: Player) -> Player {
    player.equip("Slayer helmet (i)", None);
    player.update_bonuses();
    player
}

pub fn salve_ei(mut player: Player) -> Player {
    player.equip("Salve amulet(ei)", None);
    player.update_bonuses();
    player
}

pub fn salve_i(mut player: Player) -> Player {
    player.equip("Salve amulet(i)", None);
    player.update_bonuses();
    player
}

pub fn avarice_forinthry(mut player: Player) -> Player {
    player.equip("Amulet of avarice", None);
    player.boosts.forinthry_surge = true;
    player.update_bonuses();
    player
}

pub fn efaritays_aid(mut player: Player) -> Player {
    player.equip("Efaritay's aid", None);
    player.update_bonuses();
    player
}

pub fn scale_toa(mut monster: Monster, toa_level: u32) -> Monster {
    monster.info.toa_level = toa_level;
    monster.scale_toa();
    monster
}
