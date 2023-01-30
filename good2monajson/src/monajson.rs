use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, PartialEq)]
pub struct MonaJSON {
    pub version: Option<String>,
    pub flower: Option<Vec<Artifact>>,
    pub feather: Option<Vec<Artifact>>,
    pub sand: Option<Vec<Artifact>>,
    pub cup: Option<Vec<Artifact>>,
    pub head: Option<Vec<Artifact>>,
}

#[derive(DeJson, SerJson, PartialEq, Clone)]
pub struct Artifact {
    #[nserde(rename = "setName")]
    pub set_name: ArtifactSetName,
    #[nserde(rename = "detailName")]
    pub detail_name: String,
    pub position: ArtifactTypeName,
    #[nserde(rename = "mainTag")]
    pub main_tag: ArtTag,
    #[nserde(rename = "normalTags")]
    pub normal_tags: Vec<ArtTag>,
    pub omit: bool,
    pub level: i16,
    pub star: i16,
}

#[repr(u8)]
#[derive(DeJson, SerJson, PartialEq, Clone)]
pub enum ArtifactSetName {
    #[nserde(rename = "archaicPetra")]
    ArchaicPetra,
    #[nserde(rename = "heartOfDepth")]
    HeartOfDepth,
    #[nserde(rename = "blizzardStrayer")]
    BlizzardStrayer,
    #[nserde(rename = "retracingBolide")]
    RetracingBolide,
    #[nserde(rename = "noblesseOblige")]
    NoblesseOblige,
    #[nserde(rename = "gladiatorFinale")]
    GladiatorFinale,
    #[nserde(rename = "maidenBeloved")]
    MaidenBeloved,
    #[nserde(rename = "viridescentVenerer")]
    ViridescentVenerer,
    #[nserde(rename = "lavaWalker")]
    LavaWalker,
    #[nserde(rename = "crimsonWitch")]
    CrimsonWitch,
    #[nserde(rename = "thunderSmoother")]
    ThunderSmoother,
    #[nserde(rename = "thunderingFury")]
    ThunderingFury,
    #[nserde(rename = "bloodstainedChivalry")]
    BloodstainedChivalry,
    #[nserde(rename = "wandererTroupe")]
    WandererTroupe,
    #[nserde(rename = "scholar")]
    Scholar,
    #[nserde(rename = "gambler")]
    Gambler,
    #[nserde(rename = "tinyMiracle")]
    TinyMiracle,
    #[nserde(rename = "martialArtist")]
    MartialArtist,
    #[nserde(rename = "braveHeart")]
    BraveHeart,
    #[nserde(rename = "resolutionOfSojourner")]
    ResolutionOfSojourner,
    #[nserde(rename = "defenderWill")]
    DefenderWill,
    #[nserde(rename = "berserker")]
    Berserker,
    #[nserde(rename = "instructor")]
    Instructor,
    #[nserde(rename = "exile")]
    Exile,
    #[nserde(rename = "adventurer")]
    Adventurer,
    #[nserde(rename = "luckyDog")]
    LuckyDog,
    #[nserde(rename = "travelingDoctor")]
    TravelingDoctor,
    #[nserde(rename = "prayersForWisdom")]
    PrayersForWisdom,
    #[nserde(rename = "prayersToSpringtime")]
    PrayersToSpringtime,
    #[nserde(rename = "prayersForIllumination")]
    PrayersForIllumination,
    #[nserde(rename = "prayersForDestiny")]
    PrayersForDestiny,
    #[nserde(rename = "paleFlame")]
    PaleFlame,
    #[nserde(rename = "tenacityOfTheMillelith")]
    TenacityOfTheMillelith,
    #[nserde(rename = "emblemOfSeveredFate")]
    EmblemOfSeveredFate,
    #[nserde(rename = "shimenawaReminiscence")]
    ShimenawaReminiscence,
    #[nserde(rename = "huskOfOpulentDreams")]
    HuskOfOpulentDreams,
    #[nserde(rename = "oceanHuedClam")]
    OceanHuedClam,
    VermillionHereafter,
    EchoesOfAnOffering,
    DeepwoodMemories,
    GildedDreams,
    FlowerOfParadiseLost,
    DesertPavilionChronicle,
}

#[derive(DeJson, SerJson, Eq, Hash, PartialEq, Clone)]
pub enum ArtifactTypeName {
    #[nserde(rename = "flower")]
    Flower,
    #[nserde(rename = "feather")]
    Feather,
    #[nserde(rename = "sand")]
    Sand,
    #[nserde(rename = "cup")]
    Cup,
    #[nserde(rename = "head")]
    Head,
}

#[derive(DeJson, SerJson, PartialEq, Clone)]
pub struct ArtTag {
    pub name: SecondaryTagName,
    pub value: f64,
}

#[derive(DeJson, SerJson, PartialEq, Clone)]
pub enum SecondaryTagName {
    #[nserde(rename = "cureEffect")]
    CureEffect,
    #[nserde(rename = "lifeStatic")]
    LifeStatic,
    #[nserde(rename = "lifePercentage")]
    LifePercentage,
    #[nserde(rename = "attackStatic")]
    AttackStatic,
    #[nserde(rename = "attackPercentage")]
    AttackPercentage,
    #[nserde(rename = "defendStatic")]
    DefendStatic,
    #[nserde(rename = "defendPercentage")]
    DefendPercentage,
    #[nserde(rename = "critical")]
    Critical,
    #[nserde(rename = "eCritical")]
    ECritical,
    #[nserde(rename = "criticalDamage")]
    CriticalDamage,
    #[nserde(rename = "elementalMastery")]
    ElementalMastery,
    #[nserde(rename = "recharge")]
    Recharge,
    #[nserde(rename = "thunderBonus")]
    ThunderBonus,
    #[nserde(rename = "fireBonus")]
    FireBonus,
    #[nserde(rename = "waterBonus")]
    WaterBonus,
    #[nserde(rename = "iceBonus")]
    IceBonus,
    #[nserde(rename = "windBonus")]
    WindBonus,
    #[nserde(rename = "rockBonus")]
    RockBonus,
    #[nserde(rename = "dendroBonus")]
    DendroBonus,
    #[nserde(rename = "physicalBonus")]
    PhysicalBonus,
    #[nserde(rename = "bonus")]
    Bonus,
    #[nserde(rename = "aBonus")]
    ABonus,
    #[nserde(rename = "bBonus")]
    BBonus,
    #[nserde(rename = "eBonus")]
    EBonus,
    #[nserde(rename = "qBonus")]
    QBonus,
}
