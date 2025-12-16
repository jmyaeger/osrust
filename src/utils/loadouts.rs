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

    let _ = player.equip("Torva full helm", None);
    let _ = player.equip("Torva platebody", None);
    let _ = player.equip("Torva platelegs", None);
    let _ = player.equip("Ferocious gloves", None);
    let _ = player.equip("Primordial boots", None);
    let _ = player.equip("Ghrazi rapier", None);
    let _ = player.equip("Avernic defender", None);
    let _ = player.equip("Rada's blessing 4", None);
    let _ = player.equip("Amulet of rancour", None);
    let _ = player.equip("Infernal cape", None);
    let _ = player.equip("Ultor ring", None);

    player.update_bonuses();
    player.set_active_style(CombatStyle::Lunge);

    player
}

pub fn max_ranged_zcb_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.add_prayer(Prayer::Rigour);
    player.add_potion(Potion::Ranging);

    let _ = player.equip("Masori mask (f)", None);
    let _ = player.equip("Necklace of anguish", None);
    let _ = player.equip("Dizana's quiver", Some("Charged"));
    let _ = player.equip("Dragon bolts", Some("Unpoisoned"));
    let _ = player.equip("Zaryte crossbow", None);
    let _ = player.equip("Twisted buckler", None);
    let _ = player.equip("Masori body (f)", None);
    let _ = player.equip("Masori chaps (f)", None);
    let _ = player.equip("Zaryte vambraces", None);
    let _ = player.equip("Pegasian boots", None);
    let _ = player.equip("Venator ring", None);

    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);

    player
}

pub fn max_mage_sang_staff_player() -> Player {
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.add_prayer(Prayer::Augury);
    player.add_potion(Potion::SaturatedHeart);

    let _ = player.equip("Ancestral hat", None);
    let _ = player.equip("Occult necklace", None);
    let _ = player.equip("Imbued guthix cape", None);
    let _ = player.equip("Rada's blessing 4", None);
    let _ = player.equip("Sanguinesti staff", Some("Charged"));
    let _ = player.equip("Elidinis' ward (f)", None);
    let _ = player.equip("Ancestral robe top", None);
    let _ = player.equip("Ancestral robe bottom", None);
    let _ = player.equip("Tormented bracelet", None);
    let _ = player.equip("Eternal boots", None);
    let _ = player.equip("Magus ring", None);

    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);

    player
}

pub fn bowfa_crystal_player() -> Player {
    let mut player = max_ranged_zcb_player();

    let _ = player.equip("Bow of faerdhinen (c)", None);
    let _ = player.equip("Crystal helm", Some("Active"));
    let _ = player.equip("Crystal body", Some("Active"));
    let _ = player.equip("Crystal legs", Some("Active"));
    Rc::make_mut(&mut player.gear).ammo = None;
    let _ = player.equip("Rada's blessing 4", None);

    player.update_bonuses();
    player.update_set_effects();

    player
}
