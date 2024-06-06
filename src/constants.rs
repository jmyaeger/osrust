pub const TBOW_ACC_CAP: i32 = 140;

pub const TBOW_DMG_CAP: i32 = 250;

pub const MAGIC_DEF_EXCEPTIONS: [&str; 15] = [
    "Baboon Brawler (Level 56)",
    "Baboon Brawler (Level 68)",
    "Fragment of Seren",
    "Rabbit",
    "Verzik Vitur (Normal P1)",
    "Verzik Vitur (Normal P2)",
    "Verzik Vitur (Normal P3)",
    "Verzik Vitur (Entry P1)",
    "Verzik Vitur (Entry P2)",
    "Verzik Vitur (Entry P3)",
    "Verzik Vitur (Hard P1)",
    "Verzik Vitur (Hard P2)",
    "Verzik Vitur (Hard P3)",
    "Ice demon (Normal)",
    "Ice demon (Challenge Mode)",
];

pub const TOA_MONSTERS: [&str; 32] = [
    "Akkha",
    "Akkha's Shadow",
    "Ba-Ba",
    "Baboon",
    "Baboon Brawler (Level 56)",
    "Baboon Brawler (Level 68)",
    "Baboon Mage (Level 56)",
    "Baboon Mage (Level 68)",
    "Baboon Shaman",
    "Baboon Thrall",
    "Baboon Thrower (Level 56)",
    "Baboon Thrower (Level 68)",
    "Cursed Baboon",
    "Volatile Baboon",
    "Kephri (Shield down)",
    "Kephri (Shield up)",
    "Agile Scarab",
    "Arcane Scarab",
    "Scarab Swarm",
    "Soldier Scarab",
    "Spitting Scarab",
    "Scarab",
    "Zebak",
    "Crocodile (Tombs of Amascut)",
    "Obelisk",
    "Tumeken's Warden (Active)",
    "Tumeken's Warden (Damaged)",
    "Tumeken's Warden (Enraged))",
    "Elidinis' Warden (Active)",
    "Elidinis' Warden (Damaged)",
    "Elidinis' Warden (Enraged)",
    "Core (Wardens)",
];

pub const TOA_PATH_MONSTERS: [&str; 9] = [
    "Akkha",
    "Akkha's Shadow",
    "Ba-Ba",
    "Kephri (Shield down)",
    "Kephri (Shield up)",
    "Arcane Scarab",
    "Spitting Scarab",
    "Soldier Scarab",
    "Zebak",
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

pub const ONE_HIT_MONSTERS: [&str; 3] = [
    "Giant rat (Scurrius)",
    "Flower",
    "Flower (A Night at the Theatre)",
];

pub const IMMUNE_TO_MELEE_MONSTERS: [&str; 17] = [
    "Kraken (Kraken)",
    "Abyssal portal (Normal)",
    "Abyssal portal (Challenge Mode)",
    "TzKal-Zuk (Normal)",
    "TzKal-Zuk (Enraged)",
    "Jal-MejJak",
    "Leviathan (Post-Quest)",
    "Leviathan (Quest)",
    "Leviathan (Awakened)",
    "Dawn",
    "Great Olm (Right claw)",
    "Great Olm (Right claw (Challenge Mode))",
    "Great Olm (Head)",
    "Great Olm (Head (Challenge Mode))",
    "Zulrah (Serpentine)",
    "Zulrah (Magma)",
    "Zulrah (Tanzanite)",
];

pub const IMMUNE_TO_NON_SALAMANDER_MELEE_DAMAGE_MONSTERS: [&str; 16] = [
    "Aviansie (Level 69)",
    "Aviansie (Level 71)",
    "Aviansie (Level 73)",
    "Aviansie (Level 79 (1))",
    "Aviansie (Level 79 (2))",
    "Aviansie (Level 83)",
    "Aviansie (Level 84)",
    "Aviansie (Level 89)",
    "Aviansie (Level 92)",
    "Aviansie (Level 94)",
    "Aviansie (Level 97 (1))",
    "Aviansie (Level 97 (2))",
    "Aviansie (Level 131)",
    "Aviansie (Level 137)",
    "Aviansie (Level 148)",
    "Reanimated aviansie",
];

pub const IMMUNE_TO_RANGED_MONSTERS: [&str; 8] = [
    "Tekton (Normal)",
    "Tekton (Enraged)",
    "Tekton (Normal (Challenge Mode))",
    "Tekton (Enraged (Challenge Mode))",
    "Dusk (First form)",
    "Dusk (Second form)",
    "Glowing crystal (Normal)",
    "Glowing crystal (Challenge Mode)",
];

pub const IMMUNE_TO_MAGIC_MONSTERS: [&str; 2] = ["Dusk (First form)", "Dusk (Second form)"];

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
