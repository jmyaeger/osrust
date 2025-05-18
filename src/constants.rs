use crate::types::equipment::CombatType;

pub const TBOW_ACC_CAP: i32 = 140;

pub const TBOW_DMG_CAP: i32 = 250;

pub const ICE_DEMON_IDS: &[i32] = &[
    7584, // reg
    7585, // cm
];

pub const HUEYCOATL_TAIL_ID: i32 = 14014;

pub const VERZIK_IDS: [i32; 18] = [
    10830, 10831, 10832, // P1 Entry
    8369, 8370, 8371, // P1 Normal
    10847, 10848, 10849, // P1 Hard
    10833, 10834, 10835, // verzik entry mode
    8372, 8373, 8374, // verzik normal mode
    10850, 10851, 10852, // verzik hard mode
];

pub const MAGIC_DEF_EXCEPTIONS: [i32; 27] = [
    7584, // Ice demon (Normal)
    7585, // Ice demon (CM)
    8917, 8918, 8919, 8920, // Fragment of Seren
    10830, 10831, 10832, // P1 Entry Verzik
    8369, 8370, 8371, // P1 Normal Verzik
    10847, 10848, 10849, // P1 Hard Verzik
    10833, 10834, 10835, // verzik entry mode
    8372, 8373, 8374, // verzik normal mode
    10850, 10851, 10852, // verzik hard mode,
    11709, 11712, // Baboon brawler
    9118,  // Rabbit (Prifddinas)
];

pub const AKKHA_IDS: &[i32] = &[11789, 11790, 11791, 11792, 11793, 11794, 11795, 11796];

pub const AKKHA_SHADOW_IDS: &[i32] = &[11797, 11798, 11799];

pub const BABA_IDS: &[i32] = &[11778, 11779, 11780];

pub const KEPHRI_IDS: &[i32] = &[11719, 11721];

pub const KEPHRI_OVERLORD_IDS: &[i32] = &[11724, 11725, 11726];

pub const ZEBAK_IDS: &[i32] = &[11730, 11732, 11733];

pub const TOA_OBELISK_IDS: &[i32] = &[11750, 11751, 11752];

pub const P2_WARDEN_IDS: &[i32] = &[
    11753, 11754, // elidinis
    11756, 11757, // tumeken
];

pub const P3_WARDEN_IDS: &[i32] = &[
    11761, 11763, // elidinis
    11762, 11764, // tumeken
];

pub const TOA_WARDEN_CORE_EJECTED_IDS: &[i32] = &[
    11755, // elidinis
    11758, // tumeken
];

pub const TOA_MONSTERS: [i32; 35] = [
    11789, 11790, 11791, 11792, 11793, 11794, 11795, 11796, // Akkha
    11797, 11798, 11799, // Akkha shadows
    11778, 11779, 11780, // Ba-Ba
    11719, 11721, // Kephri
    11724, 11725, 11726, // Kephri overlords
    11730, 11732, 11733, // Zebak
    11750, 11751, 11752, // ToA obelisk
    11753, 11754, // Elidinis P2
    11756, 11757, // Tumeken P2
    11761, 11763, // Elidinis P3
    11762, 11764, // Tumeken P3
    11755, // Elidinis core
    11758, // Tumeken core
];

pub const TOA_PATH_MONSTERS: [i32; 22] = [
    11789, 11790, 11791, 11792, 11793, 11794, 11795, 11796, // Akkha
    11797, 11798, 11799, // Akkha shadows
    11778, 11779, 11780, // Ba-Ba
    11719, 11721, // Kephri
    11724, 11725, 11726, // Kephri overlords
    11730, 11732, 11733, // Zebak
];

pub const WILDERNESS_MONSTERS: [&str; 35] = [
    "Artio",
    "Callisto",
    "Spindel",
    "Venenatis",
    "Calvar'ion",
    "Vet'ion",
    "Chaos Elemental",
    "Chaos Fanatic",
    "Crazy archaeologist",
    "Dark warrior",
    "Elder Chaos druid",
    "Ent",
    "Greater Skeleton Hellhound",
    "King Black Dragon",
    "Lava dragon",
    "Mammoth",
    "Revenant cyclops",
    "Revenant dark beast",
    "Revenant demon",
    "Revenant dragon",
    "Revenant goblin",
    "Revenant hellhound",
    "Revenant hobgoblin",
    "Revenant imp",
    "Revenant knight",
    "Revenant maledictus",
    "Revenant ork",
    "Revenant pyrefiend",
    "Runite Golem",
    "Scorpia",
    "Scorpia's guardian",
    "Scorpia's offspring",
    "Skeleton Hellhound",
    "Spindel's Spiderling",
    "Venenatis' Spiderling",
];

pub const ONE_HIT_MONSTERS: &[i32] = &[
    7223,  // Giant rat (Scurrius)
    8584,  // Flower
    11193, // Flower (A Night at the Theatre)
];

pub const IMMUNE_TO_MELEE_MONSTERS: &[i32] = &[
    494,  // kraken
    7533, // Abyssal portal
    7706, // zuk
    7708, // Jal-MejJak
    12214, 12215, 12219, // leviathan
    7852, 7853, 7884, 7885, // dawn
    7550, 7553, // Olm mage hand
    7551, 7554, // Olm head,
    2042, 2043, 2044, // zulrah
];

pub const IMMUNE_TO_NON_SALAMANDER_MELEE_DAMAGE_MONSTERS: &[i32] = &[
    3169, 3170, 3171, 3172, 3173, 3174, 3175, 3176, 3177, 3178, 3179, 3180, 3181, 3182,
    3183, // aviansie
    7037, // reanimated aviansie
];

pub const IMMUNE_TO_NORMAL_BURN_MONSTERS: &[i32] = &[
    5862, 5863, 5866, // Cerberus
    6593, // Lava dragon
    7700, 7704, 10623, // JalTok-Jad
    7697,  // Jal-ImKot
    7691,  // Jal-Nib
    7692,  // Jal-MejRah
    7693,  // Jal-Ak
    7698, 7702, // Jal-Xil
    7699, 7703, // Jal-Zek
    6762, 6795, // Pyrelord
    8094, 8095, 8177, 8097, 8096, 8098, 8178, 8179, // Galvek
];

pub const IMMUNE_TO_STRONG_BURN_MONSTERS: &[i32] = &[
    7706, // zuk
];

pub const DUSK_IDS: &[i32] = &[
    7851, 7854, 7855, 7882, 7883, 7886, // dusk first form
    7887, 7888, 7889, // dusk second form
];

pub const TEKTON_IDS: &[i32] = &[
    7540, 7543, // reg
    7544, 7545, // cm
];

pub const IMMUNE_TO_RANGED_MONSTERS: [i32; 14] = [
    7540, 7543, // Tekton (Normal)
    7544, 7545, // Tekton (CM)
    7851, 7854, 7855, 7882, 7883, 7886, // Dusk first form
    7887, 7888, 7889, // Dusk second form
    7568, // Glowing crystal
];

pub const ALWAYS_MAX_HIT_MELEE: &[i32] = &[
    11710, 11713, // Baboon thrower
    12814, // Fremennik warband archer
];

pub const ALWAYS_MAX_HIT_RANGED: &[i32] = &[
    11711, 11714, // Baboon mage
    12815, // Fremennik warband seer
];

pub const ALWAYS_MAX_HIT_MAGIC: &[i32] = &[
    11709, 11712, // Baboon brawler
    12816, // Fremennik warband berserker
];

pub const IMMUNE_TO_MAGIC_MONSTERS: &[i32] = DUSK_IDS;

pub const IMMUNE_TO_STAT_DRAIN: &[i32] = &[13011, 13012, 13013];

pub const FULL_AHRIMS: [(&str, Option<&str>); 4] = [
    ("Ahrim's hood", None),
    ("Ahrim's robetop", None),
    ("Ahrim's robeskirt", None),
    ("Ahrim's staff", None),
];

pub const FULL_BLOOD_MOON: [(&str, Option<&str>); 4] = [
    ("Blood moon helm", None),
    ("Blood moon chestplate", None),
    ("Blood moon tassets", None),
    ("Dual macuahuitl", None),
];

pub const FULL_BLUE_MOON: [(&str, Option<&str>); 4] = [
    ("Blue moon helm", None),
    ("Blue moon chestplate", None),
    ("Blue moon tassets", None),
    ("Blue moon spear", None),
];

pub const FULL_ECLIPSE_MOON: [(&str, Option<&str>); 4] = [
    ("Eclipse moon helm", None),
    ("Eclipse moon chestplate", None),
    ("Eclipse moon tassets", None),
    ("Eclipse atlatl", None),
];

pub const FULL_DHAROKS: [(&str, Option<&str>); 4] = [
    ("Dharok's helm", None),
    ("Dharok's platebody", None),
    ("Dharok's platelegs", None),
    ("Dharok's greataxe", None),
];

pub const FULL_GUTHANS: [(&str, Option<&str>); 4] = [
    ("Guthan's helm", None),
    ("Guthan's platebody", None),
    ("Guthan's platelegs", None),
    ("Guthan's warspear", None),
];

pub const FULL_KARILS: [(&str, Option<&str>); 4] = [
    ("Karil's coif", None),
    ("Karil's leathertop", None),
    ("Karil's leatherskirt", None),
    ("Karil's crossbow", None),
];

pub const FULL_TORAGS: [(&str, Option<&str>); 4] = [
    ("Torag's helm", None),
    ("Torag's platebody", None),
    ("Torag's platelegs", None),
    ("Torag's hammers", None),
];

pub const FULL_VERACS: [(&str, Option<&str>); 4] = [
    ("Verac's helm", None),
    ("Verac's brassard", None),
    ("Verac's plateskirt", None),
    ("Verac's flail", None),
];

pub const FULL_JUSTICIAR: [(&str, Option<&str>); 3] = [
    ("Justiciar faceguard", None),
    ("Justiciar chestguard", None),
    ("Justiciar legguards", None),
];

pub const FULL_INQUISITOR: [(&str, Option<&str>); 3] = [
    ("Inquisitor's great helm", None),
    ("Inquisitor's hauberk", None),
    ("Inquisitor's plateskirt", None),
];

pub const FULL_OBSIDIAN: [(&str, Option<&str>); 3] = [
    ("Obsidian helmet", None),
    ("Obsidian platebody", None),
    ("Obsidian platelegs", None),
];

pub const FULL_VOID: [(&str, Option<&str>); 8] = [
    ("Void melee helm", None),
    ("Void ranger helm", None),
    ("Void mage helm", None),
    ("Void knight top", None),
    ("Elite void top", None),
    ("Elite void robe", None),
    ("Void knight robe", None),
    ("Void knight gloves", None),
];

pub const FULL_ELITE_VOID: [(&str, Option<&str>); 6] = [
    ("Void melee helm", None),
    ("Void ranger helm", None),
    ("Void mage helm", None),
    ("Elite void top", None),
    ("Elite void robe", None),
    ("Void knight gloves", None),
];

pub const BLOODBARK_ARMOR: [(&str, Option<&str>); 5] = [
    ("Bloodbark helm", None),
    ("Bloodbark body", None),
    ("Bloodbark legs", None),
    ("Bloodbark gauntlets", None),
    ("Bloodbark boots", None),
];

pub const NON_BOLT_OR_ARROW_AMMO: [(&str, Option<&str>); 21] = [
    ("Toxic blowpipe", Some("Bronze")),
    ("Toxic blowpipe", Some("Iron")),
    ("Toxic blowpipe", Some("Steel")),
    ("Toxic blowpipe", Some("Mithril")),
    ("Toxic blowpipe", Some("Adamant")),
    ("Toxic blowpipe", Some("Rune")),
    ("Toxic blowpipe", Some("Amethyst")),
    ("Toxic blowpipe", Some("Dragon")),
    ("Craw's bow", Some("Charged")),
    ("Crystal bow", Some("Active")),
    ("Webweaver bow", Some("Charged")),
    ("Bow of faerdhinen", Some("Charged")),
    ("Bow of faerdhinen (c)", None),
    ("Swamp lizard", None),
    ("Orange salamander", None),
    ("Red salamander", None),
    ("Black salamander", None),
    ("Tecu salamander", None),
    ("Eclipse atlatl", None),
    ("Light ballista", None),
    ("Heavy ballista", None),
];

pub const USES_OWN_AMMO: [(&str, Option<&str>); 16] = [
    ("Toxic blowpipe", Some("Bronze")),
    ("Toxic blowpipe", Some("Iron")),
    ("Toxic blowpipe", Some("Steel")),
    ("Toxic blowpipe", Some("Mithril")),
    ("Toxic blowpipe", Some("Adamant")),
    ("Toxic blowpipe", Some("Rune")),
    ("Toxic blowpipe", Some("Amethyst")),
    ("Toxic blowpipe", Some("Dragon")),
    ("Craw's bow", Some("Charged")),
    ("Crystal bow", Some("Active")),
    ("Crystal bow (basic)", None),
    ("Crystal bow (attuned)", None),
    ("Crystal bow (perfected)", None),
    ("Webweaver bow", Some("Charged")),
    ("Bow of faerdhinen", Some("Charged")),
    ("Bow of faerdhinen (c)", None),
];

pub const OPAL_PROC_CHANCE: f64 = 0.05;

pub const PEARL_PROC_CHANCE: f64 = 0.06;

pub const EMERALD_PROC_CHANCE: f64 = 0.55;

pub const RUBY_PROC_CHANCE: f64 = 0.06;

pub const DIAMOND_PROC_CHANCE: f64 = 0.1;

pub const ONYX_PROC_CHANCE: f64 = 0.11;

pub const DRAGONSTONE_PROC_CHANCE: f64 = 0.06;

pub const SOULREAPER_STACK_DAMAGE: u32 = 8;

pub const PICKAXE_BONUSES: [(&str, u32); 10] = [
    ("Bronze pickaxe", 1),
    ("Iron pickaxe", 1),
    ("Steel pickaxe", 5),
    ("Black pickaxe", 11),
    ("Mithril pickaxe", 21),
    ("Adamant pickaxe", 31),
    ("Rune pickaxe", 41),
    ("Gilded pickaxe", 41),
    ("Dragon pickaxe", 61),
    ("Crystal pickaxe", 61),
];

pub const SILVER_WEAPONS: [(&str, Option<&str>); 17] = [
    ("Blessed axe", None),
    ("Silver sickle", None),
    ("Silver sickle (b)", None),
    ("Emerald sickle", None),
    ("Emerald sickle (b)", None),
    ("Enchanted emerald sickle (b)", None),
    ("Ruby sickle (b)", None),
    ("Enchanted ruby sickle (b)", None),
    ("Silverlight", None),
    ("Silverlight", Some("Dyed")),
    ("Darklight", None),
    ("Arclight", None),
    ("Rod of ivandis", None),
    ("Wolfbane", None),
    ("Blisterwood flail", None),
    ("Blisterwood sickle", None),
    ("Ivandis flail", None),
];

pub const SECONDS_PER_TICK: f64 = 0.6;

pub const TTK_DIST_MAX_ITER_ROUNDS: usize = 1000;

pub const TTK_DIST_EPSILON: f64 = 0.0001;

pub const DEFAULT_ATTACK_ROLLS: [(CombatType, i32); 7] = [
    (CombatType::Stab, 0),
    (CombatType::Slash, 0),
    (CombatType::Crush, 0),
    (CombatType::Light, 0),
    (CombatType::Standard, 0),
    (CombatType::Heavy, 0),
    (CombatType::Magic, 0),
];

pub const DEFAULT_MAX_HITS: [(CombatType, u32); 7] = [
    (CombatType::Stab, 0),
    (CombatType::Slash, 0),
    (CombatType::Crush, 0),
    (CombatType::Light, 0),
    (CombatType::Standard, 0),
    (CombatType::Heavy, 0),
    (CombatType::Magic, 0),
];

pub const DEFAULT_DEF_ROLLS: [(CombatType, i32); 5] = [
    (CombatType::Stab, 0),
    (CombatType::Slash, 0),
    (CombatType::Crush, 0),
    (CombatType::Ranged, 0),
    (CombatType::Magic, 0),
];

pub const STAB_SPEC_WEAPONS: [&str; 5] = [
    "Arclight",
    "Emberlight",
    "Darklight",
    "Silverlight",
    "Dragon sword",
];

pub const SLASH_SPEC_WEAPONS: [&str; 13] = [
    "Ancient godsword",
    "Bandos godsword",
    "Armadyl godsword",
    "Saradomin godsword",
    "Zamorak godsword",
    "Crystal halberd",
    "Dragon halberd",
    "Dragon scimitar",
    "Dragon longsword",
    "Dragon dagger",
    "Abyssal dagger",
    "Dragon claws",
    "Saradomin sword",
];

pub const CRUSH_SPEC_WEAPONS: [&str; 3] = ["Dinh's bulwark", "Ancient mace", "Dragon mace"];

pub const MAGIC_SPEC_WEAPONS: [&str; 1] = ["Saradomin's blessed sword"];

const BURN_PATTERNS: [[u8; 3]; 8] = [
    [0, 0, 0],
    [0, 0, 1],
    [0, 1, 0],
    [0, 1, 1],
    [1, 0, 0],
    [1, 0, 1],
    [1, 1, 0],
    [1, 1, 1],
];

pub const BURN_EXPECTED: [f64; 3] = {
    let mut results = [0.0; 3];
    let mut acc_roll = 0;

    while acc_roll < 3 {
        let burn_chance = 0.15 * (acc_roll as f64 + 1.0);
        let mut sum = 0.0;

        let mut pattern_idx = 0;
        while pattern_idx < BURN_PATTERNS.len() {
            let pattern = BURN_PATTERNS[pattern_idx];
            let mut prob = 1.0;
            let mut i = 0;

            while i < 3 {
                prob *= if pattern[i] == 0 {
                    1.0 - burn_chance
                } else {
                    burn_chance
                };
                i += 1;
            }

            let mut damage = 0.0;
            let mut i = 0;

            while i < 3 {
                damage += pattern[i] as f64 * 10.0;
                i += 1;
            }

            if pattern[0] == 1 && pattern[1] == 1 {
                damage -= 1.0;
            }

            sum += prob * damage;
            pattern_idx += 1;
        }

        results[acc_roll] = sum;
        acc_roll += 1;
    }

    results
};

pub const PLAYER_REGEN_TICKS: i32 = 100;
pub const MAX_LEVEL: u32 = 99;
pub const MIN_LEVEL: u32 = 1;
pub const MIN_HITPOINTS: u32 = 10;
pub const FULL_SPEC: u8 = 100;
pub const SPEC_REGEN: u8 = 10;
pub const DEATH_CHARGE: u8 = 15;

pub const STAT_NAMES: [&str; 9] = [
    "hitpoints",
    "attack",
    "strength",
    "defence",
    "ranged",
    "magic",
    "prayer",
    "mining",
    "herblore",
];

pub const ANCIENT_SPECTRES: [(&str, Option<&str>); 5] = [
    ("Ancient sceptre", None),
    ("Smoke ancient sceptre", None),
    ("Shadow ancient sceptre", None),
    ("Blood ancient sceptre", None),
    ("Ice ancient sceptre", None),
];

pub const BLACK_MASKS: [(&str, Option<&str>); 4] = [
    ("Black mask", None),
    ("Black mask (i)", None),
    ("Slayer helmet", None),
    ("Slayer helmet (i)", None),
];

pub const BLACK_MASKS_IMBUED: [(&str, Option<&str>); 2] =
    [("Black mask (i)", None), ("Slayer helmet (i)", None)];

pub const SALVE_UNENCHANTED: [(&str, Option<&str>); 2] =
    [("Salve amulet", None), ("Salve amulet(i)", None)];

pub const SALVE_IMBUED: [(&str, Option<&str>); 2] =
    [("Salve amulet(i)", None), ("Salve amulet(ei)", None)];

pub const SALVE_ENCHANTED: [(&str, Option<&str>); 2] =
    [("Salve amulet(e)", None), ("Salve amulet(ei)", None)];

pub const WILDY_MACES: [(&str, Option<&str>); 2] = [
    ("Viggora's chainmace", Some("Charged")),
    ("Ursine chainmace", Some("Charged")),
];

pub const WILDY_BOWS: [(&str, Option<&str>); 2] = [
    ("Craw's bow", Some("Charged")),
    ("Webweaver bow", Some("Charged")),
];

pub const WILDY_STAVES: [(&str, Option<&str>); 4] = [
    ("Thammaron's sceptre", Some("Charged")),
    ("Accursed sceptre", Some("Charged")),
    ("Thammaron's sceptre (a)", Some("Charged")),
    ("Accursed sceptre (a)", Some("Charged")),
];

pub const ELF_BOWS: [(&str, Option<&str>); 3] = [
    ("Crystal bow", Some("Active")),
    ("Bow of faerdhinen", Some("Charged")),
    ("Bow of faerdhinen (c)", None),
];

pub const SMOKE_STAVES: [(&str, Option<&str>); 3] = [
    ("Smoke battlestaff", None),
    ("Mystic smoke staff", None),
    ("Twinflame staff", None),
];

pub const IVANDIS_WEAPONS: [(&str, Option<&str>); 3] = [
    ("Blisterwood flail", None),
    ("Blisterwood sickle", None),
    ("Ivandis flail", None),
];

pub const KERIS_WEAPONS: [(&str, Option<&str>); 5] = [
    ("Keris", None),
    ("Keris partisan", None),
    ("Keris partisan of the sun", None),
    ("Keris partisan of corruption", None),
    ("Keris partisan of breaching", None),
];

pub const LEAF_BLADED_WEAPONS: [(&str, Option<&str>); 3] = [
    ("Leaf-bladed spear", None),
    ("Leaf-bladed sword", None),
    ("Leaf-bladed battleaxe", None),
];

pub const BROAD_BOLTS: [(&str, Option<&str>); 2] =
    [("Broad bolts", None), ("Amethyst broad bolts", None)];

pub const RATBANE_WEAPONS: [(&str, Option<&str>); 3] = [
    ("Bone mace", None),
    ("Bone shortbow", None),
    ("Bone staff", None),
];

pub const ALWAYS_HITS_SPEC: [(&str, Option<&str>); 2] =
    [("Voidwaker", None), ("Dawnbringer", None)];

pub const MAGIC_SHORTBOWS: [(&str, Option<&str>); 2] =
    [("Magic shortbow", None), ("Magic shortbow (i)", None)];

pub const DOUBLE_HIT_WEAPONS: [(&str, Option<&str>); 3] = [
    ("Torag's hammers", None),
    ("Sulphur blades", None),
    ("Glacial temotli", None),
];

pub const OPAL_BOLTS: [(&str, Option<&str>); 2] =
    [("Opal bolts (e)", None), ("Opal dragon bolts (e)", None)];

pub const PEARL_BOLTS: [(&str, Option<&str>); 2] =
    [("Pearl bolts (e)", None), ("Pearl dragon bolts (e)", None)];

pub const DIAMOND_BOLTS: [(&str, Option<&str>); 2] = [
    ("Diamond bolts (e)", None),
    ("Diamond dragon bolts (e)", None),
];

pub const DRAGONSTONE_BOLTS: [(&str, Option<&str>); 2] = [
    ("Dragonstone bolts (e)", None),
    ("Dragonstone dragon bolts (e)", None),
];

pub const ONYX_BOLTS: [(&str, Option<&str>); 2] =
    [("Onyx bolts (e)", None), ("Onyx dragon bolts (e)", None)];

pub const RUBY_BOLTS: [(&str, Option<&str>); 2] =
    [("Ruby bolts (e)", None), ("Ruby dragon bolts (e)", None)];

pub const DEMONBANE_WEAPONS: [(&str, Option<&str>); 6] = [
    ("Silverlight", Some("Dyed")),
    ("Silverlight", None),
    ("Darklight", None),
    ("Arclight", Some("Charged")),
    ("Emberlight", None),
    ("Scorching bow", None),
];

pub const OGRE_BOWS: [(&str, Option<&str>); 2] = [("Ogre bow", None), ("Comp ogre bow", None)];
pub const EAT_DELAY: u32 = 3;

pub const SPEC_COSTS: [(&str, u8); 68] = [
    ("Ancient godsword", 50),
    ("Eldritch nightmare staff", 55),
    ("Keris partisan of the sun", 75),
    ("Purging staff", 25),
    ("Toxic blowpipe", 50),
    ("Saradomin godsword", 50),
    ("Dinh's bulwark", 50),
    ("Dragon crossbow", 60),
    ("Dragon halberd", 30),
    ("Crystal halberd", 30),
    ("Abyssal whip", 25),
    ("Accursed sceptre", 50),
    ("Ancient mace", 100),
    ("Bandos godsword", 50),
    ("Barrelchest anchor", 50),
    ("Bone dagger", 75),
    ("Darklight", 50),
    ("Arclight", 50),
    ("Emberlight", 50),
    ("Dorgeshuun crossbow", 75),
    ("Dragon scimitar", 55),
    ("Dragon warhammer", 50),
    ("Elder maul", 50),
    ("Seercull", 100),
    ("Staff of the dead", 100),
    ("Toxic staff of the dead", 100),
    ("Staff of light", 100),
    ("Staff of balance", 100),
    ("Tonalztics of ralos", 50),
    ("Abyssal bludgeon", 50),
    ("Armadyl crossbow", 50),
    ("Armadyl godsword", 50),
    ("Blue moon spear", 50),
    ("Dawnbringer", 35),
    ("Dragon longsword", 25),
    ("Dragon mace", 25),
    ("Dragon sword", 40),
    ("Dual macuahuitl", 25),
    ("Eclipse atlatl", 50),
    ("Granite hammer", 60),
    ("Keris partisan of corruption", 75),
    ("Light ballista", 65),
    ("Heavy ballista", 65),
    ("Magic longbow", 35),
    ("Magic comp bow", 35),
    ("Noxious halberd", 50),
    ("Osmumten's fang", 25),
    ("Rune claws", 25),
    ("Saradomin's blessed sword", 65),
    ("Soulflame horn", 25),
    ("Voidwaker", 50),
    ("Volatile nightmare staff", 55),
    ("Zaryte crossbow", 75),
    ("Abyssal dagger", 25),
    ("Burning claws", 35),
    ("Dark bow", 55),
    ("Dragon claws", 50),
    ("Dragon dagger", 25),
    ("Dragon knife", 25),
    ("Magic shortbow", 55),
    ("Magic shortbow (i)", 50),
    ("Saradomin sword", 100),
    ("Webweaver bow", 50),
    ("Abyssal tentacle", 50),
    ("Scorching bow", 25),
    ("Ursine chainmace", 50),
    ("Zamorak godsword", 50),
    ("Soulreaper axe", 0),
];
