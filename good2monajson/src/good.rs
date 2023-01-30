use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, PartialEq)]
pub struct GOOD {
    pub format: String,
    pub version: i64,
    pub source: String,
    pub artifacts: Option<Vec<Artifact>>,
}

#[derive(DeJson, SerJson, PartialEq)]
pub struct Artifact {
    #[nserde(rename = "setKey")]
    pub set_key: SetKey,
    #[nserde(rename = "slotKey")]
    pub slot_key: SlotKey,
    pub level: i16,
    pub rarity: i16,
    #[nserde(rename = "mainStatKey")]
    pub main_stat_key: StatKey,
    pub location: String,
    pub lock: bool,
    #[nserde(rename = "substats")]
    pub sub_stats: Vec<SubStat>,
}

#[derive(DeJson, SerJson, Eq, Hash, PartialEq)]
pub enum SlotKey {
    #[nserde(rename = "flower")]
    Flower,
    #[nserde(rename = "plume")]
    Plume,
    #[nserde(rename = "sands")]
    Sands,
    #[nserde(rename = "goblet")]
    Goblet,
    #[nserde(rename = "circlet")]
    Circlet,
}

#[repr(u8)]
#[derive(DeJson, SerJson, PartialEq)]
pub enum SetKey {
    Adventurer,
    LuckyDog,
    TravelingDoctor,
    ResolutionOfSojourner,
    TinyMiracle,
    Berserker,
    Instructor,
    TheExile,
    DefendersWill,
    BraveHeart,
    MartialArtist,
    Gambler,
    Scholar,
    PrayersForWisdom,
    PrayersForDestiny,
    PrayersForIllumination,
    PrayersToSpringtime,
    GladiatorsFinale,
    WanderersTroupe,
    NoblesseOblige,
    BloodstainedChivalry,
    MaidenBeloved,
    ViridescentVenerer,
    ArchaicPetra,
    RetracingBolide,
    Thundersoother,
    ThunderingFury,
    Lavawalker,
    CrimsonWitchOfFlames,
    BlizzardStrayer,
    HeartOfDepth,
    TenacityOfTheMillelith,
    PaleFlame,
    ShimenawasReminiscence,
    EmblemOfSeveredFate,
    HuskOfOpulentDreams,
    OceanHuedClam,
    VermillionHereafter,
    EchoesOfAnOffering,
    DeepwoodMemories,
    GildedDreams,
    DesertPavilionChronicle,
    FlowerOfParadiseLost,
}

#[derive(DeJson, SerJson, PartialEq)]
pub struct SubStat {
    pub key: StatKey,
    pub value: f64,
}

#[derive(DeJson, SerJson, PartialEq, Debug, Eq, Hash)]
pub enum StatKey {
    #[nserde(rename = "hp")]
    Hp,
    #[nserde(rename = "hp_")]
    HpPercent,
    #[nserde(rename = "atk")]
    Atk,
    #[nserde(rename = "atk_")]
    AtkPercent,
    #[nserde(rename = "def")]
    Def,
    #[nserde(rename = "def_")]
    DefPercent,
    #[nserde(rename = "eleMas")]
    ElementalMastery,
    #[nserde(rename = "enerRech_")]
    EnergyRecharge,
    #[nserde(rename = "heal_")]
    HealBonus,
    #[nserde(rename = "critRate_")]
    CritRate,
    #[nserde(rename = "critDMG_")]
    CritDmg,
    #[nserde(rename = "physical_dmg_")]
    PhysicalDmg,
    #[nserde(rename = "anemo_dmg_")]
    AnemoDmg,
    #[nserde(rename = "geo_dmg_")]
    GeoDmg,
    #[nserde(rename = "electro_dmg_")]
    ElectroDmg,
    #[nserde(rename = "hydro_dmg_")]
    HydroDmg,
    #[nserde(rename = "pyro_dmg_")]
    PyroDmg,
    #[nserde(rename = "cryo_dmg_")]
    CryoDmg,
    #[nserde(rename = "dendro_dmg_")]
    DendroDmg,
}
