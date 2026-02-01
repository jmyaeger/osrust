use crate::types::equipment::CombatStyle;
use crate::types::player::Player;
use crate::types::potions::Potion;
use crate::types::prayers::Prayer;
use crate::types::stats::PlayerStats;

use std::rc::Rc;

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
    player.equip("Amulet of rancour", None).unwrap();
    player.equip("Infernal cape", None).unwrap();
    player.equip("Ultor ring", None).unwrap();

    player.update_bonuses();
    player.set_active_style(CombatStyle::Lunge);

    player
}

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

pub fn bowfa_crystal_player() -> Player {
    let mut player = max_ranged_zcb_player();

    player.equip("Bow of faerdhinen (c)", None).unwrap();
    player.equip("Crystal helm", Some("Active")).unwrap();
    player.equip("Crystal body", Some("Active")).unwrap();
    player.equip("Crystal legs", Some("Active")).unwrap();
    Rc::make_mut(&mut player.gear).ammo = None;
    player.equip("Rada's blessing 4", None).unwrap();

    player.update_bonuses();
    player.update_set_effects();

    player
}
