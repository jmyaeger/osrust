use crate::equipment::CombatStyle;
use crate::player::{Player, PlayerStats};
use crate::potions::Potion;
use crate::prayers::{Prayer, PrayerBoost};

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
