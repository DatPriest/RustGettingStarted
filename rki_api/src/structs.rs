use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct RkiAttributes {
    pub attributes: RkiData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RkiWrapper {
    pub features: Vec<RkiAttributes>,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RkiData {
    pub adm_unit_id: i16,
    pub anz_fall_erkrankung: i32,
    pub anz_fall_meldung: i32,
    pub anz_fall_neu: i32,
    pub anz_fall_vortag: i32,
    pub bundesland_id: i8,
    pub datum: i64,
    pub kum_fall: i32,
    pub object_id: i32,
}

impl Serialize for RkiData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RkiData", 3)?;
        state.serialize_field("AdmUnitId", &self.adm_unit_id)?;
        state.serialize_field("AnzFallErkrankung", &self.anz_fall_erkrankung)?;
        state.serialize_field("AnzFallMeldung", &self.anz_fall_meldung)?;
        state.serialize_field("AnzFallNeu", &self.anz_fall_neu)?;
        state.serialize_field("AnzFallVortag", &self.anz_fall_vortag)?;
        state.serialize_field("BundeslandId", &self.bundesland_id)?;
        state.serialize_field("Datum", &self.datum)?;
        state.serialize_field("KumFall", &self.kum_fall)?;
        state.serialize_field("ObjectId", &self.object_id)?;
        state.end()
    }
}

impl RkiData {
    fn new(
        adm_unit_id: i16,
        anz_fall_erkrankung: i32,
        anz_fall_meldung: i32,
        anz_fall_neu: i32,
        anz_fall_vortag: i32,
        bundesland_id: i8,
        datum: i64,
        kum_fall: i32,
        object_id: i32,
    ) -> Self {
        RkiData {
            adm_unit_id,
            anz_fall_erkrankung,
            anz_fall_meldung,
            anz_fall_neu,
            anz_fall_vortag,
            bundesland_id,
            datum,
            kum_fall,
            object_id,
        }
    }

    pub fn convert_to_class(dat: &Value) -> RkiData {
        RkiData::new(
            dat["AdmUnitId"].to_string().parse::<i16>().unwrap(),
            dat["AnzFallErkrankung"].to_string().parse::<i32>().unwrap(),
            dat["AnzFallMeldung"].to_string().parse::<i32>().unwrap(),
            dat["AnzFallNeu"].to_string().parse::<i32>().unwrap(),
            dat["AnzFallVortag"].to_string().parse::<i32>().unwrap(),
            dat["BundeslandId"].to_string().parse::<i8>().unwrap(),
            dat["Datum"].to_string().parse::<i64>().unwrap(),
            dat["KumFall"].to_string().parse::<i32>().unwrap(),
            dat["ObjectId"].to_string().parse::<i32>().unwrap(),
        )
    }

    pub fn as_prometheus_string(&self) -> String {
        let mut result: String = String::new();
        result += "rkidata{";
        result += "AdmUnitId=\"";
        result += &self.adm_unit_id.to_string();
        result += "\" ";
        result += &self.anz_fall_erkrankung.to_string();
        result += &self.anz_fall_meldung.to_string();
        result += &self.anz_fall_neu.to_string();
        result += &self.anz_fall_vortag.to_string();
        result += &self.bundesland_id.to_string();
        result += &self.datum.to_string();
        result += &self.kum_fall.to_string();
        result += &self.object_id.to_string();
        result += "}\n";
        result
    }
}
