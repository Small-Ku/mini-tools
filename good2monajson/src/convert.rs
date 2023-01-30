use std::collections::HashMap;

use crate::{
    convert_maps::get_main_stat,
    good::{self, GOOD},
    monajson::{self, MonaJSON},
};

pub fn convert(good_json: GOOD) -> MonaJSON {
    let artifacts_map: HashMap<monajson::ArtifactTypeName, Vec<monajson::Artifact>> = good_json
        .artifacts
        .unwrap()
        .into_iter()
        .map(|artifact| convert_art(artifact))
        .fold(HashMap::new(), |mut map, artifact| {
            map.entry(artifact.position.clone())
                .or_insert_with(|| Vec::new())
                .push(artifact);
            map
        });
    MonaJSON {
        version: Some(String::from("1")),
        flower: artifacts_map
            .get(&monajson::ArtifactTypeName::Flower)
            .cloned(),
        feather: artifacts_map
            .get(&monajson::ArtifactTypeName::Feather)
            .cloned(),
        sand: artifacts_map
            .get(&monajson::ArtifactTypeName::Sand)
            .cloned(),
        cup: artifacts_map.get(&monajson::ArtifactTypeName::Cup).cloned(),
        head: artifacts_map
            .get(&monajson::ArtifactTypeName::Head)
            .cloned(),
    }
}

fn convert_art(good_art: good::Artifact) -> monajson::Artifact {
    monajson::Artifact {
        set_name: convert_set(&good_art.set_key),
        detail_name: String::from(""),
        position: convert_slot(&good_art.slot_key),
        main_tag: convert_tag_with_value(
            &good_art.main_stat_key,
            get_main_stat(&good_art.rarity, &good_art.level, &good_art.main_stat_key),
        ),
        normal_tags: good_art
            .sub_stats
            .into_iter()
            .map(|stat| convert_tag(stat))
            .collect(),
        omit: good_art.lock,
        level: good_art.level,
        star: good_art.rarity,
    }
}

fn convert_set(good_set_key: &good::SetKey) -> monajson::ArtifactSetName {
    match good_set_key {
        good::SetKey::Adventurer => monajson::ArtifactSetName::Adventurer,
        good::SetKey::LuckyDog => monajson::ArtifactSetName::LuckyDog,
        good::SetKey::TravelingDoctor => monajson::ArtifactSetName::TravelingDoctor,
        good::SetKey::ResolutionOfSojourner => monajson::ArtifactSetName::ResolutionOfSojourner,
        good::SetKey::TinyMiracle => monajson::ArtifactSetName::TinyMiracle,
        good::SetKey::Berserker => monajson::ArtifactSetName::Berserker,
        good::SetKey::Instructor => monajson::ArtifactSetName::Instructor,
        good::SetKey::TheExile => monajson::ArtifactSetName::Exile,
        good::SetKey::DefendersWill => monajson::ArtifactSetName::DefenderWill,
        good::SetKey::BraveHeart => monajson::ArtifactSetName::BraveHeart,
        good::SetKey::MartialArtist => monajson::ArtifactSetName::MartialArtist,
        good::SetKey::Gambler => monajson::ArtifactSetName::Gambler,
        good::SetKey::Scholar => monajson::ArtifactSetName::Scholar,
        good::SetKey::PrayersForWisdom => monajson::ArtifactSetName::PrayersForWisdom,
        good::SetKey::PrayersForDestiny => monajson::ArtifactSetName::PrayersForDestiny,
        good::SetKey::PrayersForIllumination => monajson::ArtifactSetName::PrayersForIllumination,
        good::SetKey::PrayersToSpringtime => monajson::ArtifactSetName::PrayersToSpringtime,
        good::SetKey::GladiatorsFinale => monajson::ArtifactSetName::GladiatorFinale,
        good::SetKey::WanderersTroupe => monajson::ArtifactSetName::WandererTroupe,
        good::SetKey::NoblesseOblige => monajson::ArtifactSetName::NoblesseOblige,
        good::SetKey::BloodstainedChivalry => monajson::ArtifactSetName::BloodstainedChivalry,
        good::SetKey::MaidenBeloved => monajson::ArtifactSetName::MaidenBeloved,
        good::SetKey::ViridescentVenerer => monajson::ArtifactSetName::ViridescentVenerer,
        good::SetKey::ArchaicPetra => monajson::ArtifactSetName::ArchaicPetra,
        good::SetKey::RetracingBolide => monajson::ArtifactSetName::RetracingBolide,
        good::SetKey::Thundersoother => monajson::ArtifactSetName::ThunderSmoother,
        good::SetKey::ThunderingFury => monajson::ArtifactSetName::ThunderingFury,
        good::SetKey::Lavawalker => monajson::ArtifactSetName::LavaWalker,
        good::SetKey::CrimsonWitchOfFlames => monajson::ArtifactSetName::CrimsonWitch,
        good::SetKey::BlizzardStrayer => monajson::ArtifactSetName::BlizzardStrayer,
        good::SetKey::HeartOfDepth => monajson::ArtifactSetName::HeartOfDepth,
        good::SetKey::TenacityOfTheMillelith => monajson::ArtifactSetName::TenacityOfTheMillelith,
        good::SetKey::PaleFlame => monajson::ArtifactSetName::PaleFlame,
        good::SetKey::ShimenawasReminiscence => monajson::ArtifactSetName::ShimenawaReminiscence,
        good::SetKey::EmblemOfSeveredFate => monajson::ArtifactSetName::EmblemOfSeveredFate,
        good::SetKey::HuskOfOpulentDreams => monajson::ArtifactSetName::HuskOfOpulentDreams,
        good::SetKey::OceanHuedClam => monajson::ArtifactSetName::OceanHuedClam,
        good::SetKey::VermillionHereafter => monajson::ArtifactSetName::VermillionHereafter,
        good::SetKey::EchoesOfAnOffering => monajson::ArtifactSetName::EchoesOfAnOffering,
        good::SetKey::DeepwoodMemories => monajson::ArtifactSetName::DeepwoodMemories,
        good::SetKey::GildedDreams => monajson::ArtifactSetName::GildedDreams,
        good::SetKey::DesertPavilionChronicle => monajson::ArtifactSetName::DesertPavilionChronicle,
        good::SetKey::FlowerOfParadiseLost => monajson::ArtifactSetName::FlowerOfParadiseLost,
    }
}

fn convert_slot(good_slot: &good::SlotKey) -> monajson::ArtifactTypeName {
    match good_slot {
        good::SlotKey::Flower => monajson::ArtifactTypeName::Flower,
        good::SlotKey::Plume => monajson::ArtifactTypeName::Feather,
        good::SlotKey::Sands => monajson::ArtifactTypeName::Sand,
        good::SlotKey::Goblet => monajson::ArtifactTypeName::Cup,
        good::SlotKey::Circlet => monajson::ArtifactTypeName::Head,
    }
}

fn convert_tag_name(good_stat_key: &good::StatKey) -> monajson::SecondaryTagName {
    match good_stat_key {
        good::StatKey::Hp => monajson::SecondaryTagName::LifeStatic,
        good::StatKey::HpPercent => monajson::SecondaryTagName::LifePercentage,
        good::StatKey::Atk => monajson::SecondaryTagName::AttackStatic,
        good::StatKey::AtkPercent => monajson::SecondaryTagName::AttackPercentage,
        good::StatKey::Def => monajson::SecondaryTagName::DefendStatic,
        good::StatKey::DefPercent => monajson::SecondaryTagName::DefendPercentage,
        good::StatKey::ElementalMastery => monajson::SecondaryTagName::ElementalMastery,
        good::StatKey::EnergyRecharge => monajson::SecondaryTagName::Recharge,
        good::StatKey::HealBonus => monajson::SecondaryTagName::CureEffect,
        good::StatKey::CritRate => monajson::SecondaryTagName::Critical,
        good::StatKey::CritDmg => monajson::SecondaryTagName::CriticalDamage,
        good::StatKey::PhysicalDmg => monajson::SecondaryTagName::PhysicalBonus,
        good::StatKey::AnemoDmg => monajson::SecondaryTagName::WindBonus,
        good::StatKey::GeoDmg => monajson::SecondaryTagName::RockBonus,
        good::StatKey::ElectroDmg => monajson::SecondaryTagName::ThunderBonus,
        good::StatKey::HydroDmg => monajson::SecondaryTagName::WaterBonus,
        good::StatKey::PyroDmg => monajson::SecondaryTagName::FireBonus,
        good::StatKey::CryoDmg => monajson::SecondaryTagName::IceBonus,
        good::StatKey::DendroDmg => monajson::SecondaryTagName::DendroBonus,
    }
}

fn convert_tag_with_value(good_stat_key: &good::StatKey, value: f64) -> monajson::ArtTag {
    monajson::ArtTag {
        name: convert_tag_name(good_stat_key),
        value: value,
    }
}

fn convert_tag(good_sub_stat: good::SubStat) -> monajson::ArtTag {
    monajson::ArtTag {
        name: convert_tag_name(&good_sub_stat.key),
        value: match good_sub_stat.key {
            good::StatKey::HpPercent
            | good::StatKey::AtkPercent
            | good::StatKey::DefPercent
            | good::StatKey::EnergyRecharge
            | good::StatKey::HealBonus
            | good::StatKey::CritRate
            | good::StatKey::CritDmg
            | good::StatKey::PhysicalDmg
            | good::StatKey::AnemoDmg
            | good::StatKey::GeoDmg
            | good::StatKey::ElectroDmg
            | good::StatKey::HydroDmg
            | good::StatKey::PyroDmg
            | good::StatKey::CryoDmg
            | good::StatKey::DendroDmg => good_sub_stat.value / 100.,
            _ => good_sub_stat.value,
        },
    }
}
