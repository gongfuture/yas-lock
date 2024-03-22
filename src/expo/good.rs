use anyhow::Result;
use std::convert::From;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::artifact::internal_artifact::{
    ArtifactSetName, ArtifactSlot, ArtifactStat, ArtifactStatName, CharacterKey, InternalArtifact,
};

impl ArtifactStatName {
    pub fn to_good(&self) -> String {
        let temp = match self {
            ArtifactStatName::HealingBonus => "heal_",
            ArtifactStatName::CriticalDamage => "critDMG_",
            ArtifactStatName::Critical => "critRate_",
            ArtifactStatName::Atk => "atk",
            ArtifactStatName::AtkPercentage => "atk_",
            ArtifactStatName::ElementalMastery => "eleMas",
            ArtifactStatName::Recharge => "enerRech_",
            ArtifactStatName::HpPercentage => "hp_",
            ArtifactStatName::Hp => "hp",
            ArtifactStatName::DefPercentage => "def_",
            ArtifactStatName::Def => "def",
            ArtifactStatName::ElectroBonus => "electro_dmg_",
            ArtifactStatName::PyroBonus => "pyro_dmg_",
            ArtifactStatName::HydroBonus => "hydro_dmg_",
            ArtifactStatName::CryoBonus => "cryo_dmg_",
            ArtifactStatName::AnemoBonus => "anemo_dmg_",
            ArtifactStatName::GeoBonus => "geo_dmg_",
            ArtifactStatName::PhysicalBonus => "physical_dmg_",
            ArtifactStatName::DendroBonus => "dendro_dmg_",
        };
        String::from(temp)
    }
}

impl ArtifactSetName {
    pub fn to_good(&self) -> String {
        return self.to_string();
    }
}

impl ArtifactSlot {
    pub fn to_good(&self) -> String {
        let temp = match self {
            ArtifactSlot::Flower => "flower",
            ArtifactSlot::Feather => "plume",
            ArtifactSlot::Sand => "sands",
            ArtifactSlot::Goblet => "goblet",
            ArtifactSlot::Head => "circlet",
        };
        String::from(temp)
    }
}

impl CharacterKey {
    pub fn to_good(&self) -> String {
        self.to_string()
    }
}

struct GoodArtifactStat<'a> {
    stat: &'a ArtifactStat,
}

impl<'a> Serialize for GoodArtifactStat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(2))?;
        root.serialize_entry("key", &self.stat.key.to_good())?;
        root.serialize_entry("value", &self.stat.value)?;
        root.end()
    }
}

struct GoodArtifact<'a> {
    artifact: &'a InternalArtifact,
}

impl<'a> Serialize for GoodArtifact<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(8))?;

        root.serialize_entry("setKey", &self.artifact.set_key.to_good())?;
        root.serialize_entry("slotKey", &self.artifact.slot_key.to_good())?;
        root.serialize_entry("level", &self.artifact.level)?;
        root.serialize_entry("rarity", &self.artifact.rarity)?;
        root.serialize_entry("lock", &self.artifact.lock)?;
        let location = match &self.artifact.location {
            Some(l) => l.to_good(),
            None => String::from(""),
        };
        root.serialize_entry("location", &location)?;
        root.serialize_entry("mainStatKey", &self.artifact.main_stat.key.to_good())?;
        let mut substats: Vec<GoodArtifactStat> = vec![];
        if let Some(ref s) = self.artifact.sub_stat_1 {
            substats.push(GoodArtifactStat { stat: s });
        }
        if let Some(ref s) = self.artifact.sub_stat_2 {
            substats.push(GoodArtifactStat { stat: s });
        }
        if let Some(ref s) = self.artifact.sub_stat_3 {
            substats.push(GoodArtifactStat { stat: s });
        }
        if let Some(ref s) = self.artifact.sub_stat_4 {
            substats.push(GoodArtifactStat { stat: s });
        }
        root.serialize_entry("substats", &substats)?;
        root.end()
    }
}

pub struct GoodFormat<'a> {
    format: String,
    version: u32,
    source: String,
    artifacts: Vec<GoodArtifact<'a>>,
}

impl<'a> Serialize for GoodFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(4))?;
        root.serialize_entry("format", &self.format)?;
        root.serialize_entry("version", &self.version)?;
        root.serialize_entry("source", &self.source)?;
        root.serialize_entry("artifacts", &self.artifacts)?;
        root.end()
    }
}

impl<'a> GoodFormat<'a> {
    pub fn new(results: &'a Vec<InternalArtifact>) -> GoodFormat {
        let artifacts: Vec<GoodArtifact<'a>> = results
            .into_iter()
            .map(|artifact| GoodArtifact { artifact })
            .collect();

        GoodFormat {
            format: String::from("GOOD"),
            version: 1,
            source: String::from("yas-lock"),
            artifacts,
        }
    }
}
