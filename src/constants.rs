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
    "Tumeken's Warden (P2)",
    "Tumeken's Warden (P3)",
    "Tumeken's Warden (P4)",
    "Elidinis' Warden (P2)",
    "Elidinis' Warden (P3)",
    "Elidinis' Warden (P4)",
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

pub const WILDERNESS_MONSTERS: [&str; 39] = [
    "Artio",
    "Callisto",
    "Spindel",
    "Venenatis",
    "Calvar'ion (Normal)",
    "Calvar'ion (Enraged)",
    "Vet'ion (Normal)",
    "Vet'ion (Enraged)",
    "Chaos Elemental",
    "Chaos Fanatic",
    "Crazy archaeologist",
    "Dark warrior",
    "Elder Chaos druid",
    "Ent",
    "Greater Skeleton Hellhound (Calvar'ion)",
    "Greater Skeleton Hellhound (Vet'ion)",
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
    "Skeleton Hellhound (Calvar'ion)",
    "Skeleton Hellhound (Vet'ion)",
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

pub const FULL_AHRIMS: [&str; 4] = [
    "Ahrim's hood",
    "Ahrim's robetop",
    "Ahrim's robeskirt",
    "Ahrim's staff",
];

pub const FULL_BLOOD_MOON: [&str; 4] = [
    "Blood moon helm",
    "Blood moon chestplate",
    "Blood moon tassets",
    "Dual macuahuitl",
];

pub const FULL_BLUE_MOON: [&str; 4] = [
    "Blue moon helm",
    "Blue moon chestplate",
    "Blue moon tassets",
    "Blue moon spear",
];

pub const FULL_ECLIPSE_MOON: [&str; 4] = [
    "Eclipse moon helm",
    "Eclipse moon chestplate",
    "Eclipse moon tassets",
    "Eclipse atlatl",
];

pub const FULL_DHAROKS: [&str; 4] = [
    "Dharok's helm",
    "Dharok's platebody",
    "Dharok's platelegs",
    "Dharok's greataxe",
];

pub const FULL_GUTHANS: [&str; 4] = [
    "Guthan's helm",
    "Guthan's platebody",
    "Guthan's platelegs",
    "Guthan's warspear",
];

pub const FULL_KARILS: [&str; 4] = [
    "Karil's coif",
    "Karil's leathertop",
    "Karil's leatherskirt",
    "Karil's crossbow",
];

pub const FULL_TORAGS: [&str; 4] = [
    "Torag's helm",
    "Torag's platebody",
    "Torag's platelegs",
    "Torag's hammers",
];

pub const FULL_VERACS: [&str; 4] = [
    "Verac's helm",
    "Verac's brassard",
    "Verac's plateskirt",
    "Verac's flail",
];

pub const FULL_JUSTICIAR: [&str; 3] = [
    "Justiciar faceguard",
    "Justiciar chestguard",
    "Justiciar legguards",
];

pub const FULL_INQUISITOR: [&str; 3] = [
    "Inquisitor's great helm",
    "Inquisitor's hauberk",
    "Inquisitor's plateskirt",
];

pub const FULL_OBSIDIAN: [&str; 3] = [
    "Obsidian helmet",
    "Obsidian platebody",
    "Obsidian platelegs",
];

pub const FULL_VOID: [&str; 8] = [
    "Void melee helm",
    "Void ranger helm",
    "Void mage helm",
    "Void knight top",
    "Elite void top",
    "Elite void robe",
    "Void knight robe",
    "Void knight gloves",
];

pub const FULL_ELITE_VOID: [&str; 6] = [
    "Void melee helm",
    "Void ranger helm",
    "Void mage helm",
    "Elite void top",
    "Elite void robe",
    "Void knight gloves",
];

pub const BLOODBARK_ARMOR: [&str; 5] = [
    "Bloodbark helm",
    "Bloodbark body",
    "Bloodbark legs",
    "Bloodbark gauntlets",
    "Bloodbark boots",
];

pub const NON_BOLT_OR_ARROW_AMMO: [&str; 22] = [
    "Toxic blowpipe (empty)",
    "Toxic blowpipe (bronze)",
    "Toxic blowpipe (iron)",
    "Toxic blowpipe (steel)",
    "Toxic blowpipe (mithril)",
    "Toxic blowpipe (adamant)",
    "Toxic blowpipe (rune)",
    "Toxic blowpipe (amethyst)",
    "Toxic blowpipe (dragon)",
    "Craw's bow",
    "Crystal bow",
    "Webweaver bow",
    "Bow of faerdhinen",
    "Bow of faerdhinen (c)",
    "Swamp lizard",
    "Orange salamander",
    "Red salamander",
    "Black salamander",
    "Tecu salamander",
    "Eclipse atlatl",
    "Light ballista",
    "Heavy ballista",
];

pub const USES_OWN_AMMO: [&str; 14] = [
    "Toxic blowpipe (empty)",
    "Toxic blowpipe (bronze)",
    "Toxic blowpipe (iron)",
    "Toxic blowpipe (steel)",
    "Toxic blowpipe (mithril)",
    "Toxic blowpipe (adamant)",
    "Toxic blowpipe (rune)",
    "Toxic blowpipe (amethyst)",
    "Toxic blowpipe (dragon)",
    "Craw's bow",
    "Crystal bow",
    "Webweaver bow",
    "Bow of faerdhinen",
    "Bow of faerdhinen (c)",
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
