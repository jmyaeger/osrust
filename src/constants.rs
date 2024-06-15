use crate::equipment::CombatType;

pub const TBOW_ACC_CAP: i32 = 140;

pub const TBOW_DMG_CAP: i32 = 250;

pub const ICE_DEMON_IDS: &[i32] = &[
    7584, // reg
    7585, // cm
];

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

pub const FLAT_ARMOUR: [(i32, i32); 5] = [
    (13011, -2), // Blood moon
    (13012, 6),  // Eclipse moon (4 in clone phase)
    (13013, -5), // Blue moon
    (13033, -4), // Sulphur nagua
    (13029, -2), // Grimy lizard
];

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
