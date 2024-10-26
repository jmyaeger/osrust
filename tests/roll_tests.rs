use osrs::equipment::CombatType;
use osrs::monster::Monster;
use osrs::player::Player;
use osrs::rolls::{calc_player_magic_rolls, calc_player_melee_rolls, calc_player_ranged_rolls};
use rstest::rstest;

mod fixtures;
use fixtures::*;

#[rstest]
#[case(max_melee_player(), ammonite_crab(), &CombatType::Stab, 33525, 56)]
#[case(mid_level_melee_player(), ammonite_crab(), &CombatType::Slash, 24125, 39)]
#[case(max_melee_dhl_player(), vorkath(), &CombatType::Stab, 38880, 60)]
#[case(max_melee_keris_partisan_player(), kalphite(), &CombatType::Crush, 27714, 59)]
#[case(max_melee_blue_keris_partisan_player(), kalphite(), &CombatType::Crush, 36859, 59)]
#[case(mid_level_melee_barronite_player(), chaos_golem(), &CombatType::Crush, 18600, 35)]
#[case(max_melee_ursine_chainmace_player(), vetion(), &CombatType::Crush, 44700, 78)]
#[case(max_melee_ursine_chainmace_player(), ammonite_crab(), &CombatType::Crush, 29800, 52)]
#[case(max_melee_silverlight_player(), kril(), &CombatType::Slash, 21456, 59)]
#[case(max_melee_darklight_player(), kril(), &CombatType::Slash, 21754, 59)]
#[case(max_melee_arclight_player(), kril(), &CombatType::Slash, 42554, 61)]
#[case(max_melee_silverlight_player(), duke(), &CombatType::Slash, 21456, 52)]
#[case(max_melee_darklight_player(), duke(), &CombatType::Slash, 21754, 52)]
#[case(max_melee_arclight_player(), duke(), &CombatType::Slash, 37297, 53)]
#[case(mid_level_melee_lbba_player(), kurask(), &CombatType::Slash, 22692, 49)]
#[case(max_melee_colossal_blade_player(), aberrant_spectre(), &CombatType::Slash, 29651, 60)]
#[case(max_melee_colossal_blade_player(), abhorrent_spectre(), &CombatType::Slash, 29651, 62)]
#[case(max_melee_colossal_blade_player(), general_graardor(), &CombatType::Slash, 29651, 64)]
#[case(max_melee_colossal_blade_player(), rune_dragon(), &CombatType::Slash, 29651, 66)]
#[case(mid_level_melee_bone_mace_player(), scurrius(), &CombatType::Crush, 21080, 44)]
#[case(slayer(max_melee_player()), ammonite_crab(), &CombatType::Stab, 39112, 63)]
#[case(slayer(mid_level_melee_player()), ammonite_crab(), &CombatType::Slash, 28145, 45)]
#[case(slayer(max_melee_dhl_player()), vorkath(), &CombatType::Stab, 45360, 67)]
#[case(slayer(max_melee_keris_partisan_player()), kalphite(), &CombatType::Crush, 32333, 66)]
#[case(slayer(max_melee_blue_keris_partisan_player()), kalphite(), &CombatType::Crush, 43002, 66)]
#[case(slayer(max_melee_ursine_chainmace_player()), vetion(), &CombatType::Crush, 52149, 87)]
#[case(slayer(max_melee_silverlight_player()), kril(), &CombatType::Slash, 25032, 64)]
#[case(slayer(max_melee_darklight_player()), kril(), &CombatType::Slash, 25379, 64)]
#[case(slayer(max_melee_arclight_player()), kril(), &CombatType::Slash, 49646, 66)]
#[case(slayer(max_melee_silverlight_player()), duke(), &CombatType::Slash, 25032, 56)]
#[case(slayer(max_melee_darklight_player()), duke(), &CombatType::Slash, 25379, 56)]
#[case(slayer(max_melee_arclight_player()), duke(), &CombatType::Slash, 43513, 58)]
#[case(slayer(mid_level_melee_lbba_player()), kurask(), &CombatType::Slash, 26474, 55)]
#[case(slayer(max_melee_colossal_blade_player()), aberrant_spectre(), &CombatType::Slash, 34592, 67)]
#[case(slayer(max_melee_colossal_blade_player()), abhorrent_spectre(), &CombatType::Slash, 34592, 69)]
#[case(slayer(max_melee_colossal_blade_player()), general_graardor(), &CombatType::Slash, 34592, 71)]
#[case(slayer(max_melee_colossal_blade_player()), rune_dragon(), &CombatType::Slash, 34592, 73)]
#[case(slayer(mid_level_melee_bone_mace_player()), scurrius(), &CombatType::Crush, 24593, 49)]
#[case(salve_ei(max_melee_player()), vorkath(), &CombatType::Stab, 37548, 63)]
#[case(salve_i(max_melee_player()), vorkath(), &CombatType::Stab, 36505, 61)]
#[case(salve_ei(mid_level_melee_player()), vorkath(), &CombatType::Slash, 27450, 45)]
#[case(salve_i(mid_level_melee_player()), vorkath(), &CombatType::Slash, 26687, 44)]
#[case(salve_ei(max_melee_dhl_player()), vorkath(), &CombatType::Stab, 43416, 68)]
#[case(salve_ei(max_melee_ursine_chainmace_player()), vetion(), &CombatType::Crush, 49617, 87)]
#[case(salve_ei(max_melee_colossal_blade_player()), aberrant_spectre(), &CombatType::Slash, 32899, 68)]
#[case(salve_ei(max_melee_colossal_blade_player()), abhorrent_spectre(), &CombatType::Slash, 32899, 70)]
#[case(salve_ei(max_melee_colossal_blade_player()), bloat(), &CombatType::Slash, 32899, 74)]
#[case(salve_ei(slayer(max_melee_dhl_player())), vorkath(), &CombatType::Stab, 43416, 66)]
#[case(avarice_forinthry(max_melee_player()), revenant_dragon(), &CombatType::Stab, 44253, 74)]
#[case(avarice_forinthry(max_melee_colossal_blade_player()), revenant_dragon(), &CombatType::Slash, 39023, 84)]
#[case(avarice_forinthry(max_melee_ursine_chainmace_player()), revenant_dragon(), &CombatType::Crush, 58836, 102)]
#[case(full_obby_with_sword_player(), ammonite_crab(), &CombatType::Stab, 29174, 47)]
#[case(salve_ei(full_obby_with_sword_player()), vorkath(), &CombatType::Stab, 31572, 53)]

fn test_melee_player_rolls(
    #[case] mut player: Player,
    #[case] monster: Monster,
    #[case] combat_type: &CombatType,
    #[case] att_roll: i32,
    #[case] max_hit: u32,
) {
    calc_player_melee_rolls(&mut player, &monster);
    assert_eq!(player.att_rolls[combat_type], att_roll);
    assert_eq!(player.max_hits[combat_type], max_hit);
}

#[rstest]
#[case(max_ranged_zcb_player(), ammonite_crab(), 50694, 49)]
#[case(mid_level_ranged_rcb_player(), ammonite_crab(), 29945, 30)]
#[case(max_ranged_blowpipe_dragon_darts_player(), ammonite_crab(), 35358, 31)]
#[case(max_ranged_tbow_player(), ammonite_crab(), 16983, 19)]
#[case(max_ranged_tbow_player(), general_graardor(), 36089, 43)]
#[case(max_ranged_tbow_player(), kril(), 54770, 70)]
#[case(max_ranged_tbow_player(), zilyana(), 59441, 79)]
#[case(max_ranged_tbow_overload_player(), shaman_cox(), 48174, 60)]
#[case(max_ranged_tbow_overload_player(), abyssal_portal(), 55446, 71)]
#[case(max_ranged_tbow_overload_player(), skeletal_mystic(), 50447, 63)]
#[case(max_ranged_tbow_overload_player(), olm_head(), 63627, 86)]
#[case(max_ranged_tbow_overload_player(), olm_head_cm(), 63627, 99)]
#[case(max_ranged_tbow_salts_player(), scale_toa(zebak(), 400), 63304, 83)]
#[case(max_ranged_dhcb_player(), vorkath(), 63133, 61)]
#[case(elite_void_dhcb_player(), vorkath(), 49077, 66)]
#[case(max_ranged_webweaver_player(), spindel(), 64752, 51)]
#[case(max_ranged_webweaver_player(), ammonite_crab(), 43168, 34)]
#[case(full_eclipse_atlatl_ranged_gear_player(), ammonite_crab(), 39760, 22)]
#[case(
    full_eclipse_atlatl_melee_gear_rigour_all_pots(),
    ammonite_crab(),
    27122,
    38
)]
#[case(
    full_eclipse_atlatl_melee_gear_rigour_all_pots_80_str(),
    ammonite_crab(),
    27122,
    32
)]
#[case(slayer(max_ranged_zcb_player()), ammonite_crab(), 56828, 56)]
#[case(slayer(mid_level_ranged_rcb_player()), ammonite_crab(), 33916, 34)]
#[case(
    slayer(max_ranged_blowpipe_dragon_darts_player()),
    ammonite_crab(),
    39192,
    35
)]
#[case(
    slayer(eclipse_atlatl_ranged_gear_player()),
    ammonite_crab(),
    48500,
    24
)]
#[case(
    slayer(eclipse_atlatl_melee_gear_rigour_all_pots()),
    ammonite_crab(),
    18452,
    45
)]
#[case(slayer(max_ranged_dhcb_player()), vorkath(), 68564, 68)]
#[case(slayer(max_ranged_webweaver_player()), spindel(), 69118, 54)]
#[case(mid_level_ranged_bone_shortbow_player(), scurrius(), 26216, 30)]
#[case(slayer(mid_level_ranged_bone_shortbow_player()), scurrius(), 29628, 33)]
#[case(slayer(max_ranged_tbow_overload_player()), shaman_cox_cm(), 64885, 82)]
#[case(salve_ei(max_ranged_zcb_player()), vorkath(), 58276, 57)]
#[case(salve_ei(mid_level_ranged_rcb_player()), vorkath(), 34578, 36)]
#[case(
    salve_ei(max_ranged_blowpipe_dragon_darts_player()),
    vorkath(),
    39873,
    36
)]
#[case(salve_ei(elite_void_dhcb_player()), vorkath(), 55242, 76)]
#[case(salve_ei(max_ranged_webweaver_player()), vetion(), 73867, 58)]
#[case(
    salve_ei(max_ranged_tbow_overload_player()),
    skeletal_mystic_cm(),
    68895,
    90
)]
#[case(salve_ei(slayer(max_ranged_dhcb_player())), vorkath(), 70443, 71)]
#[case(
    avarice_forinthry(max_ranged_zcb_player()),
    revenant_dragon(),
    67478,
    64
)]
#[case(
    avarice_forinthry(mid_level_ranged_rcb_player()),
    revenant_dragon(),
    40425,
    40
)]
#[case(
    avarice_forinthry(max_ranged_blowpipe_dragon_darts_player()),
    revenant_dragon(),
    46774,
    40
)]
#[case(
    avarice_forinthry(max_ranged_webweaver_player()),
    revenant_dragon(),
    85977,
    66
)]
#[case(
    avarice_forinthry(slayer(max_ranged_webweaver_player())),
    revenant_dragon(),
    83389,
    64
)]
fn test_ranged_player_rolls(
    #[case] mut player: Player,
    #[case] monster: Monster,
    #[case] att_roll: i32,
    #[case] max_hit: u32,
) {
    calc_player_ranged_rolls(&mut player, &monster);
    let combat_type = player.combat_type();
    assert_eq!(player.att_rolls[&combat_type], att_roll);
    assert_eq!(player.max_hits[&combat_type], max_hit);
}

#[rstest]
#[case(max_mage_sang_staff_player(), ammonite_crab(), 36693, 47)]
#[case(max_mage_toxic_trident_player(), ammonite_crab(), 36693, 46)]
#[case(max_mage_trident_player(), ammonite_crab(), 35183, 42)]
#[case(max_mage_harm_fire_surge_player(), ammonite_crab(), 34866, 35)]
#[case(max_mage_kodai_ice_barrage_player(), ammonite_crab(), 36654, 44)]
#[case(mid_level_magic_warped_sceptre_player(), ammonite_crab(), 20188, 21)]
#[case(
    mid_level_mage_chaos_gauntlets_fire_bolt_player(),
    ammonite_crab(),
    19190,
    17
)]
#[case(mid_level_mage_god_spell_charge_player(), ammonite_crab(), 19190, 34)]
#[case(max_mage_shadow_player(), ammonite_crab(), 83956, 66)]
#[case(
    max_mage_shadow_salts_player(),
    scale_toa(wardens_p3(), 400),
    120240,
    84
)]
#[case(max_mage_smoke_staff_fire_surge_player(), ammonite_crab(), 37697, 34)]
#[case(full_virtus_kodai_ice_barrage_player(), ammonite_crab(), 36654, 46)]
#[case(full_virtus_kodai_fire_surge_player(), ammonite_crab(), 36654, 34)]
#[case(salve_ei(max_mage_harm_fire_surge_player()), vorkath(), 39693, 39)]
#[case(salve_ei(max_mage_shadow_player()), vorkath(), 94224, 68)]
#[case(
    salve_ei(mid_level_mage_chaos_gauntlets_fire_bolt_player()),
    vorkath(),
    21573,
    19
)]
#[case(
    salve_ei(max_mage_smoke_staff_fire_surge_player()),
    vorkath(),
    42226,
    37
)]
#[case(
    salve_ei(mid_level_mage_god_spell_charge_player()),
    vorkath(),
    21573,
    38
)]
#[case(salve_ei(full_virtus_kodai_ice_barrage_player()), vorkath(), 41839, 50)]
// #[case(
//     avarice_forinthry(max_mage_sang_staff_player()),
//     revenant_dragon(),
//     49127,salve
//     58
// )] // Reimplement after confirming avarice boost placement
#[case(max_mage_accursed_sceptre_player(), vetion(), 54360, 61)]
#[case(salve_ei(max_mage_accursed_sceptre_player()), vetion(), 61969, 67)]
// #[case(avarice_forinthry(max_mage_accursed_sceptre_player()), revenant_dragon(), 72774, 73)]
#[case(slayer(max_mage_accursed_sceptre_player()), vetion(), 61210, 69)]
// #[case(max_mage_harm_tome_of_water_surge_player(), ammonite_crab(), 38799, 37)]
// Commented out until we know the actual currrent tome of water accuracy boost
#[case(slayer(max_mage_shadow_player()), ammonite_crab(), 93944, 72)]
#[case(
    slayer(mid_level_mage_chaos_gauntlets_fire_bolt_player()),
    ammonite_crab(),
    21720,
    19
)]
#[case(
    slayer(mid_level_mage_god_spell_charge_player()),
    ammonite_crab(),
    21720,
    39
)]
#[case(
    slayer(max_mage_smoke_staff_fire_surge_player()),
    ammonite_crab(),
    42408,
    37
)]
#[case(
    slayer(full_virtus_kodai_ice_barrage_player()),
    ammonite_crab(),
    41295,
    50
)]
#[case(slayer(salve_ei(max_mage_sang_staff_player())), vorkath(), 40951, 52)]
#[case(max_mage_harm_fire_surge_tome_player(), ammonite_crab(), 32333, 37)]
#[case(
    slayer(max_mage_harm_fire_surge_tome_player()),
    ammonite_crab(),
    36326,
    40
)]
#[case(salve_ei(max_mage_harm_fire_surge_tome_player()), vorkath(), 36654, 40)]
#[case(
    mid_level_mage_chaos_gauntlets_fire_bolt_tome_player(),
    ammonite_crab(),
    18786,
    17
)]
fn test_magic_player_rolls(
    #[case] mut player: Player,
    #[case] monster: Monster,
    #[case] att_roll: i32,
    #[case] max_hit: u32,
) {
    calc_player_magic_rolls(&mut player, &monster);
    assert_eq!(player.att_rolls[&CombatType::Magic], att_roll);
    assert_eq!(player.max_hits[&CombatType::Magic], max_hit);
}
