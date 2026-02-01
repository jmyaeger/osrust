use osrs::types::equipment::{CombatStyle, GearSlot};
use osrs::types::monster::Monster;
use osrs::types::player::Player;
use osrs::types::potions::Potion;
use osrs::types::prayers::Prayer;
use osrs::types::spells::{AncientSpell, ArceuusSpell, Spell, StandardSpell};
use osrs::types::stats::{PlayerStats, SpecEnergy, Stat};
use rstest::fixture;

#[fixture]
pub fn vorkath() -> Monster {
    Monster::new("Vorkath", Some("Post-quest")).expect("Error creating monster.")
}

#[fixture]
pub fn kril() -> Monster {
    Monster::new("K'ril Tsutsaroth", None).expect("Error creating monster.")
}

#[fixture]
pub fn kalphite() -> Monster {
    Monster::new("Kalphite Soldier", Some("Kalphite Lair")).expect("Error creating monster.")
}

#[fixture]
pub fn ammonite_crab() -> Monster {
    Monster::new("Ammonite Crab", None).expect("Error creating monster.")
}

#[fixture]
pub fn vetion() -> Monster {
    Monster::new("Vet'ion", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn spindel() -> Monster {
    Monster::new("Spindel", None).expect("Error creating monster.")
}

#[fixture]
pub fn duke() -> Monster {
    Monster::new("Duke Sucellus", Some("Post-quest, Awake")).expect("Error creating monster.")
}

#[fixture]
pub fn kurask() -> Monster {
    Monster::new("Kurask", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn scurrius() -> Monster {
    Monster::new("Scurrius", Some("Solo")).expect("Error creating monster.")
}

#[fixture]
pub fn revenant_dragon() -> Monster {
    Monster::new("Revenant dragon", None).expect("Error creating monster.")
}

#[fixture]
pub fn zebak() -> Monster {
    Monster::new("Zebak", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn chaos_golem() -> Monster {
    Monster::new("Chaos Golem", Some("Golem")).expect("Error creating monster.")
}

#[fixture]
pub fn aberrant_spectre() -> Monster {
    Monster::new("Aberrant spectre", None).expect("Error creating monster.")
}

#[fixture]
pub fn abhorrent_spectre() -> Monster {
    Monster::new("Abhorrent spectre", None).expect("Error creating monster.")
}

#[fixture]
pub fn general_graardor() -> Monster {
    Monster::new("General Graardor", None).expect("Error creating monster.")
}

#[fixture]
pub fn rune_dragon() -> Monster {
    Monster::new("Rune dragon", None).expect("Error creating monster.")
}

#[fixture]
pub fn bloat() -> Monster {
    Monster::new("Pestilent Bloat", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn count_draynor() -> Monster {
    Monster::new("Count Draynor (Nightmare Zone)", Some("Hard Mode"))
        .expect("Error creating monster.")
}

#[fixture]
pub fn vampyre_juvinate() -> Monster {
    Monster::new("Vampyre Juvinate", Some("Level 50")).expect("Error creating monster.")
}

#[fixture]
pub fn vanstrom_klause() -> Monster {
    Monster::new("Vanstrom Klause", Some("Sins of the Father")).expect("Error creating monster.")
}

#[fixture]
pub fn zilyana() -> Monster {
    Monster::new("Commander Zilyana", None).expect("Error creating monster.")
}

#[fixture]
pub fn shaman_cox() -> Monster {
    Monster::new("Lizardman shaman (Chambers of Xeric)", Some("Normal"))
        .expect("Error creating monster.")
}

#[fixture]
pub fn abyssal_portal() -> Monster {
    Monster::new("Abyssal portal", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn skeletal_mystic() -> Monster {
    Monster::new("Skeletal Mystic", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn olm_head() -> Monster {
    Monster::new("Great Olm", Some("Head (Normal)")).expect("Error creating monster.")
}

#[fixture]
pub fn olm_head_cm() -> Monster {
    Monster::new("Great Olm", Some("Head (Challenge Mode)")).expect("Error creating monster.")
}

#[fixture]
pub fn shaman_cox_cm() -> Monster {
    Monster::new(
        "Lizardman shaman (Chambers of Xeric)",
        Some("Challenge Mode"),
    )
    .expect("Error creating monster.")
}

#[fixture]
pub fn skeletal_mystic_cm() -> Monster {
    Monster::new("Skeletal Mystic", Some("Challenge Mode")).expect("Error creating monster.")
}

#[fixture]
pub fn wardens_p3() -> Monster {
    Monster::new("Elidinis' Warden", Some("Damaged")).expect("Error creating monster.")
}

#[fixture]
pub fn vardorvis() -> Monster {
    Monster::new("Vardorvis", Some("Post-quest")).expect("Error creating monster.")
}

#[fixture]
pub fn kephri_400() -> Monster {
    let mut monster = Monster::new("Kephri", Some("Shielded")).expect("Error creating monster.");
    monster.info.toa_level = 400;
    monster.scale_toa();
    monster
}

#[fixture]
pub fn urium_shade() -> Monster {
    Monster::new("Urium Shade", Some("Shade")).expect("Error creating monster.")
}

#[fixture]
pub fn kalphite_queen_p1() -> Monster {
    Monster::new("Kalphite Queen", Some("Crawling")).expect("Error creating monster.")
}

#[fixture]
pub fn zulrah_tanzanite() -> Monster {
    Monster::new("Zulrah", Some("Tanzanite")).expect("Error creating monster.")
}

#[fixture]
pub fn zulrah_magma() -> Monster {
    Monster::new("Zulrah", Some("Magma")).expect("Error creating monster.")
}

#[fixture]
pub fn seren() -> Monster {
    Monster::new("Fragment of Seren", None).expect("Error creating monster.")
}

#[fixture]
pub fn kraken() -> Monster {
    Monster::new("Kraken", Some("Kraken")).expect("Error creating monster.")
}

#[fixture]
pub fn verzik_p1() -> Monster {
    Monster::new("Verzik Vitur", Some("Normal mode, Phase 1")).expect("Error creating monster.")
}

#[fixture]
pub fn tekton() -> Monster {
    Monster::new("Tekton", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn vasa_crystal() -> Monster {
    Monster::new("Glowing crystal", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn olm_melee_hand() -> Monster {
    Monster::new("Great Olm", Some("Left claw (Normal)")).expect("Error creating monster.")
}

#[fixture]
pub fn olm_mage_hand() -> Monster {
    Monster::new("Great Olm", Some("Right claw (Normal)")).expect("Error creating monster.")
}

#[fixture]
pub fn ice_demon() -> Monster {
    Monster::new("Ice demon", Some("Normal")).expect("Error creating monster.")
}

#[fixture]
pub fn slagilith() -> Monster {
    Monster::new("Slagilith (Nightmare Zone)", Some("Hard Mode")).expect("Error creating monster.")
}

#[fixture]
pub fn zogre() -> Monster {
    Monster::new("Zogre", None).expect("Error creating monster.")
}

#[fixture]
pub fn corp() -> Monster {
    Monster::new("Corporeal Beast", None).expect("Error creating monster.")
}

#[fixture]
pub fn baba_300() -> Monster {
    let mut monster = Monster::new("Ba-Ba", None).expect("Error creating monster.");
    monster.info.toa_level = 300;
    monster.scale_toa();
    monster
}

#[fixture]
pub fn max_melee_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.add_prayer(Prayer::Piety);
    player.add_potion(Potion::SuperCombat);

    player.equip("Torva full helm", None).unwrap();
    player.equip("Torva platebody", None).unwrap();
    player.equip("Torva platelegs", None).unwrap();
    player.equip("Ferocious gloves", None).unwrap();
    player.equip("Primordial boots", None).unwrap();
    player.equip("Ghrazi rapier", None).unwrap();
    player.equip("Avernic defender", None).unwrap();
    player.equip("Rada's blessing 4", None).unwrap();
    player.equip("Amulet of torture", None).unwrap();
    player.equip("Infernal cape", None).unwrap();
    player.equip("Ultor ring", None).unwrap();

    player.update_bonuses();
    player.set_active_style(CombatStyle::Lunge);

    player
}

#[fixture]
pub fn mid_level_melee_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats {
        attack: Stat::new(80, None),
        strength: Stat::new(80, None),
        defence: Stat::new(80, None),
        ranged: Stat::new(80, None),
        magic: Stat::new(80, None),
        hitpoints: Stat::new(80, None),
        prayer: Stat::new(70, None),
        mining: Stat::new(70, None),
        herblore: Stat::new(70, None),
        spec: SpecEnergy::default(),
    };
    player.add_prayer(Prayer::Piety);
    player.add_potion(Potion::SuperCombat);

    player.equip("Helm of neitiznot", None).unwrap();
    player.equip("Amulet of fury", None).unwrap();
    player.equip("Fire cape", None).unwrap();
    player.equip("Rada's blessing 3", None).unwrap();
    player.equip("Abyssal whip", None).unwrap();
    player.equip("Dragon defender", None).unwrap();
    player.equip("Fighter torso", None).unwrap();
    player.equip("Obsidian platelegs", None).unwrap();
    player.equip("Barrows gloves", None).unwrap();
    player.equip("Dragon boots", None).unwrap();
    player.equip("Berserker ring (i)", None).unwrap();

    player.update_bonuses();
    player.set_active_style(CombatStyle::Lash);

    player
}

#[fixture]
pub fn max_ranged_zcb_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.add_prayer(Prayer::Rigour);
    player.add_potion(Potion::Ranging);

    player.equip("Masori mask (f)", None).unwrap();
    player.equip("Necklace of anguish", None).unwrap();
    player.equip("Dizana's quiver", Some("Charged")).unwrap();
    player.equip("Dragon bolts", Some("Unpoisoned")).unwrap();
    player.equip("Zaryte crossbow", None).unwrap();
    player.equip("Twisted buckler", None).unwrap();
    player.equip("Masori body (f)", None).unwrap();
    player.equip("Masori chaps (f)", None).unwrap();
    player.equip("Zaryte vambraces", None).unwrap();
    player.equip("Pegasian boots", None).unwrap();
    player.equip("Venator ring", None).unwrap();

    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);

    player
}

#[fixture]
pub fn mid_level_ranged_rcb_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats {
        attack: Stat::new(80, None),
        strength: Stat::new(80, None),
        defence: Stat::new(80, None),
        ranged: Stat::new(80, None),
        magic: Stat::new(80, None),
        hitpoints: Stat::new(80, None),
        prayer: Stat::new(70, None),
        mining: Stat::new(70, None),
        herblore: Stat::new(70, None),
        spec: SpecEnergy::default(),
    };
    player.add_prayer(Prayer::EagleEye);
    player.add_potion(Potion::Ranging);

    player.equip("Ancient coif", None).unwrap();
    player.equip("Amulet of fury", None).unwrap();
    player.equip("Ava's assembler", None).unwrap();
    player.equip("Adamant bolts", Some("Unpoisoned")).unwrap();
    player.equip("Rune crossbow", None).unwrap();
    player.equip("Odium ward", None).unwrap();
    player.equip("Ancient d'hide body", None).unwrap();
    player.equip("Ancient chaps", None).unwrap();
    player.equip("Barrows gloves", None).unwrap();
    player.equip("Ancient d'hide boots", None).unwrap();
    player.equip("Archers ring (i)", None).unwrap();

    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);

    player
}

#[fixture]
pub fn max_melee_dhl_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dragon hunter lance", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_keris_partisan_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Keris partisan", None).unwrap();
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blue_keris_partisan_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Keris partisan of breaching", None).unwrap();
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_barronite_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Barronite mace", None).unwrap();
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_ursine_chainmace_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Ursine chainmace", Some("Charged")).unwrap();
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_berserker_neck_obby_sword_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Berserker necklace", None).unwrap();
    player.equip("Toktz-xil-ak", None).unwrap();
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_silverlight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Silverlight", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_darklight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Darklight", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_arclight_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Arclight", Some("Charged")).unwrap();
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_lbba_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Leaf-bladed battleaxe", None).unwrap();
    player.set_active_style(CombatStyle::Hack);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_colossal_blade_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Colossal blade", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_melee_bone_mace_player() -> Player {
    let mut player = mid_level_melee_player();
    player.equip("Bone mace", None).unwrap();
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_obby_with_sword_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Obsidian helmet", None).unwrap();
    player.equip("Obsidian platebody", None).unwrap();
    player.equip("Obsidian platelegs", None).unwrap();
    player.equip("Toktz-xil-ak", None).unwrap();
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player.update_set_effects();
    player
}

#[fixture]
pub fn full_obby_with_sword_and_necklace_player() -> Player {
    let mut player = full_obby_with_sword_player();
    player.equip("Berserker necklace", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blisterwood_flail_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Blisterwood flail", None).unwrap();
    player.set_active_style(CombatStyle::Pound);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_blowpipe_dragon_darts_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Toxic blowpipe", Some("Dragon")).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tbow_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Twisted bow", None).unwrap();
    player.unequip_slot(&GearSlot::Ammo);
    player.equip("Dragon arrow", Some("Unpoisoned")).unwrap();
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
    player.equip("Dragon hunter crossbow", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn elite_void_dhcb_player() -> Player {
    let mut player = max_ranged_dhcb_player();
    player.equip("Elite void top", None).unwrap();
    player.equip("Elite void robe", None).unwrap();
    player.equip("Void knight gloves", None).unwrap();
    player.equip("Void ranger helm", None).unwrap();
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_webweaver_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Webweaver bow", Some("Charged")).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_eclipse_atlatl_ranged_gear_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Eclipse atlatl", None).unwrap();
    player.equip("Eclipse moon helm", None).unwrap();
    player.equip("Eclipse moon chestplate", None).unwrap();
    player.equip("Eclipse moon tassets", None).unwrap();
    player.equip("Atlatl dart", None).unwrap();
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn eclipse_atlatl_ranged_gear_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Eclipse atlatl", None).unwrap();
    player.equip("Atlatl dart", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_eclipse_atlatl_melee_gear_rigour_all_pots() -> Player {
    let mut player = max_melee_player();
    player.equip("Eclipse atlatl", None).unwrap();
    player.equip("Eclipse moon helm", None).unwrap();
    player.equip("Eclipse moon chestplate", None).unwrap();
    player.equip("Eclipse moon tassets", None).unwrap();
    player.equip("Atlatl dart", None).unwrap();
    player.update_bonuses();
    player.update_set_effects();
    player.set_active_style(CombatStyle::Rapid);
    player.add_prayer(Prayer::Rigour);
    player.add_potion(Potion::Ranging);
    player
}

#[fixture]
pub fn full_eclipse_atlatl_melee_gear_rigour_all_pots_80_str() -> Player {
    let mut player = full_eclipse_atlatl_melee_gear_rigour_all_pots();
    player.stats.strength = Stat::new(80, None);
    player.calc_potion_boosts();
    player.reset_current_stats(false);
    player
}

#[fixture]
pub fn eclipse_atlatl_melee_gear_rigour_all_pots() -> Player {
    let mut player = max_melee_player();
    player.equip("Eclipse atlatl", None).unwrap();
    player.equip("Atlatl dart", None).unwrap();
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.add_prayer(Prayer::Rigour);
    player.add_potion(Potion::Ranging);
    player
}

#[fixture]
pub fn mid_level_ranged_bone_shortbow_player() -> Player {
    let mut player = mid_level_ranged_rcb_player();
    player.equip("Bone shortbow", None).unwrap();
    player.equip("Rune arrow", Some("Unpoisoned")).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn mid_level_ranged_rcb_silver_bolts_player() -> Player {
    let mut player = mid_level_ranged_rcb_player();
    player.equip("Silver bolts", Some("Unpoisoned")).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_sang_staff_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.add_prayer(Prayer::Augury);
    player.add_potion(Potion::SaturatedHeart);

    player.equip("Ancestral hat", None).unwrap();
    player.equip("Occult necklace", None).unwrap();
    player.equip("Imbued guthix cape", None).unwrap();
    player.equip("Rada's blessing 4", None).unwrap();
    player.equip("Sanguinesti staff", Some("Charged")).unwrap();
    player.equip("Elidinis' ward (f)", None).unwrap();
    player.equip("Ancestral robe top", None).unwrap();
    player.equip("Ancestral robe bottom", None).unwrap();
    player.equip("Tormented bracelet", None).unwrap();
    player.equip("Eternal boots", None).unwrap();
    player.equip("Magus ring", None).unwrap();

    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);

    player
}

#[fixture]
pub fn max_mage_sang_staff_brimstone_ring_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Brimstone ring", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_toxic_trident_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player
        .equip("Trident of the swamp", Some("Charged"))
        .unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_trident_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player
        .equip("Trident of the seas", Some("Charged"))
        .unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_fire_surge_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Harmonised nightmare staff", None).unwrap();
    player.update_bonuses();
    player
        .set_spell(Spell::Standard(StandardSpell::FireSurge))
        .unwrap();
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn max_mage_kodai_ice_barrage_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Kodai wand", None).unwrap();
    player.update_bonuses();
    player
        .set_spell(Spell::Ancient(AncientSpell::IceBarrage))
        .unwrap();
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn mid_level_magic_warped_sceptre_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats {
        attack: Stat::new(80, None),
        strength: Stat::new(80, None),
        defence: Stat::new(80, None),
        ranged: Stat::new(80, None),
        magic: Stat::new(80, None),
        hitpoints: Stat::new(80, None),
        prayer: Stat::new(70, None),
        mining: Stat::new(70, None),
        herblore: Stat::new(70, None),
        spec: SpecEnergy::default(),
    };
    player.add_prayer(Prayer::MysticMight);

    player.calc_potion_boosts();
    player.reset_current_stats(false);

    player.equip("Ahrim's hood", None).unwrap();
    player.equip("Occult necklace", None).unwrap();
    player.equip("Imbued guthix cape", None).unwrap();
    player.equip("Rada's blessing 3", None).unwrap();
    player.equip("Warped sceptre", Some("Charged")).unwrap();
    player.equip("Malediction ward", None).unwrap();
    player.equip("Ahrim's robetop", None).unwrap();
    player.equip("Ahrim's robeskirt", None).unwrap();
    player.equip("Barrows gloves", None).unwrap();
    player.equip("Infinity boots", None).unwrap();
    player.equip("Seers ring (i)", None).unwrap();

    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);

    player
}

#[fixture]
pub fn mid_level_mage_chaos_gauntlets_fire_bolt_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Fire battlestaff", None).unwrap();
    player.equip("Chaos gauntlets", None).unwrap();
    player.update_bonuses();
    player
        .set_spell(Spell::Standard(StandardSpell::FireBolt))
        .unwrap();
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn mid_level_mage_god_spell_charge_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Guthix staff", None).unwrap();
    player.update_bonuses();
    player.boosts.charge_active = true;
    player
        .set_spell(Spell::Standard(StandardSpell::ClawsOfGuthix))
        .unwrap();
    player.set_active_style(CombatStyle::Spell);
    player
}

#[fixture]
pub fn max_mage_shadow_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Tumeken's shadow", Some("Charged")).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_shadow_salts_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Tumeken's shadow", Some("Charged")).unwrap();
    player.add_potion(Potion::SmellingSalts);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_virtus_kodai_ice_barrage_player() -> Player {
    let mut player = max_mage_kodai_ice_barrage_player();
    player.equip("Virtus mask", None).unwrap();
    player.equip("Virtus robe top", None).unwrap();
    player.equip("Virtus robe bottom", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_virtus_kodai_fire_surge_player() -> Player {
    let mut player = full_virtus_kodai_ice_barrage_player();
    player
        .set_spell(Spell::Standard(StandardSpell::FireSurge))
        .unwrap();
    player
}

#[fixture]
pub fn max_mage_smoke_staff_fire_surge_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Smoke battlestaff", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_accursed_sceptre_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Accursed sceptre", Some("Charged")).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_tome_of_water_surge_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Tome of water", Some("Charged")).unwrap();
    player
        .set_spell(Spell::Standard(StandardSpell::WaterSurge))
        .unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_fire_surge_tome_player() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player.equip("Tome of fire", Some("Charged")).unwrap();
    player.update_bonuses();
    player
}
#[fixture]
pub fn mid_level_mage_chaos_gauntlets_fire_bolt_tome_player() -> Player {
    let mut player = mid_level_mage_chaos_gauntlets_fire_bolt_player();
    player.equip("Tome of fire", Some("Charged")).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_blade_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Blade of saeldor (c)", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_scythe_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Scythe of vitur", Some("Charged")).unwrap();
    player.set_active_style(CombatStyle::Chop);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_fang_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Osmumten's fang", None).unwrap();
    player.set_active_style(CombatStyle::Lunge);
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_ahrims_aotd_player() -> Player {
    let mut player = mid_level_magic_warped_sceptre_player();
    player.equip("Ahrim's staff", None).unwrap();
    player.equip("Amulet of the damned", Some("Full")).unwrap();
    player.attrs.spell = Some(Spell::Arceuus(ArceuusSpell::UndeadGrasp));
    player.set_active_style(CombatStyle::Spell);
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_ahrims_aotd_sunfire_player() -> Player {
    let mut player = full_ahrims_aotd_player();
    player.stats.magic = Stat::new(95, None);
    player
        .set_spell(Spell::Standard(StandardSpell::FireSurge))
        .unwrap();
    player.boosts.sunfire.active = true;
    player
}

#[fixture]
pub fn full_dharoks_1hp_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dharok's helm", None).unwrap();
    player.equip("Dharok's platebody", None).unwrap();
    player.equip("Dharok's platelegs", None).unwrap();
    player.equip("Dharok's greataxe", None).unwrap();
    player.set_active_style(CombatStyle::Hack);
    player.update_set_effects();
    player.update_bonuses();
    player.state.current_hp = Some(1);
    player.reset_current_stats(false);
    player
}

#[fixture]
pub fn full_veracs_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Verac's helm", None).unwrap();
    player.equip("Verac's plateskirt", None).unwrap();
    player.equip("Verac's brassard", None).unwrap();
    player.equip("Verac's flail", None).unwrap();
    player.set_active_style(CombatStyle::Pummel);
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_karils_aotd_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Karil's coif", None).unwrap();
    player.equip("Karil's leathertop", None).unwrap();
    player.equip("Karil's leatherskirt", None).unwrap();
    player.equip("Karil's crossbow", None).unwrap();
    player.equip("Amulet of the damned", Some("Full")).unwrap();
    player.update_set_effects();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_torags_hammers_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Torag's hammers", None).unwrap();
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tonalztics_charged_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player
        .equip("Tonalztics of ralos", Some("Charged"))
        .unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_tonalztics_uncharged_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player
        .equip("Tonalztics of ralos", Some("Uncharged"))
        .unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_melee_macuahuitl_player() -> Player {
    let mut player = max_melee_player();
    player.equip("Dual macuahuitl", None).unwrap();
    player.set_active_style(CombatStyle::Pummel);
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_ranged_zcb_ruby_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Ruby dragon bolts (e)", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_dawnbringer_player() -> Player {
    let mut player = max_mage_sang_staff_player();
    player.equip("Dawnbringer", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn max_mage_harm_crumble_undead() -> Player {
    let mut player = max_mage_harm_fire_surge_player();
    player
        .set_spell(Spell::Standard(StandardSpell::CrumbleUndead))
        .unwrap();
    player
}

#[fixture]
pub fn max_range_comp_ogre_bow_player() -> Player {
    let mut player = max_ranged_zcb_player();
    player.equip("Comp ogre bow", None).unwrap();
    player.equip("Rune brutal", None).unwrap();
    player.update_bonuses();
    player
}

#[fixture]
pub fn full_blood_moon_player() -> Player {
    let mut player = max_melee_macuahuitl_player();
    player.equip("Blood moon helm", None).unwrap();
    player.equip("Blood moon chestplate", None).unwrap();
    player.equip("Blood moon tassets", None).unwrap();
    player.update_bonuses();
    player.update_set_effects();
    player
}

#[allow(unused)]
pub fn slayer(mut player: Player) -> Player {
    player.equip("Slayer helmet (i)", None);
    player.update_bonuses();
    player
}

#[allow(unused)]
pub fn salve_ei(mut player: Player) -> Player {
    player.equip("Salve amulet(ei)", None);
    player.update_bonuses();
    player
}

#[allow(unused)]
pub fn salve_i(mut player: Player) -> Player {
    player.equip("Salve amulet(i)", None);
    player.update_bonuses();
    player
}

#[allow(unused)]
pub fn avarice_forinthry(mut player: Player) -> Player {
    player.equip("Amulet of avarice", None);
    player.boosts.forinthry_surge = true;
    player.update_bonuses();
    player
}

#[allow(unused)]
pub fn efaritays_aid(mut player: Player) -> Player {
    player.equip("Efaritay's aid", None);
    player.update_bonuses();
    player
}

#[allow(unused)]
pub fn scale_toa(mut monster: Monster, toa_level: u32) -> Monster {
    monster.info.toa_level = toa_level;
    monster.scale_toa();
    monster
}
