use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use std::collections::HashMap;
use crate::met_api::Location;

pub enum Resolution {
    ThreeHourly,
    Daily,
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreeHourly => write!(f, "3hourly"),
            Self::Daily => write!(f, "daily"),
        }
    }
}

// <?xml version="1.0" encoding="ISO-8859-1"?>
// <SiteRep>
// <Wx>
// <Param name="F" units="C">Feels Like Temperature</Param>
// <Param name="G" units="mph">Wind Gust</Param>
// <Param name="H" units="%">Screen Relative Humidity</Param>
// <Param name="T" units="C">Temperature</Param>
// <Param name="V" units="">Visibility</Param>
// <Param name="D" units="compass">Wind Direction</Param>
// <Param name="S" units="mph">Wind Speed</Param>
// <Param name="U" units="">Max UV Index</Param>
// <Param name="W" units="">Weather Type</Param>
// <Param name="Pp" units="%">Precipitation Probability</Param>
// </Wx>
// <DV dataDate="2012-11-19T14:00:00Z" type="Forecast">
// 16. <Location i="310069" lat="50.7179" lon="-3.5327" name="EXETER" country="ENGLAND" continent="EUROPE">
// <Period type="Day" value="2012-11-19Z">
// <Rep D="SSE" F="8" G="29" H="80" Pp="16" S="13" T="11" V="VG" W="7" U="1">540</Rep>
// <Rep D="S" F="9" G="34" H="88" Pp="50" S="16" T="12" V="VG" W="10" U="1">720</Rep>
// <Rep D="S" F="9" G="29" H="94" Pp="51" S="11" T="12" V="GO" W="10" U="1">900</Rep>
// <Rep D="S" F="10" G="25" H="96" Pp="52" S="9" T="12" V="GO" W="12" U="0">1080</Rep>
// <Rep D="SSW" F="11" G="20" H="97" Pp="14" S="7" T="12" V="GO" W="7" U="0">1260</Rep>
// </Period>
// <Period type="Day" value="2012-11-20Z">
// <Rep D="SSE" F="11" G="16" H="95" Pp="16" S="7" T="12" V="VG" W="7" U="0">0</Rep>
// <Rep D="S" F="11" G="31" H="96" Pp="96" S="13" T="13" V="MO" W="15" U="0">180</Rep>
// <Rep D="S" F="10" G="43" H="92" Pp="97" S="18" T="14" V="GO" W="15" U="0">360</Rep>
// <Rep D="S" F="10" G="45" H="92" Pp="94" S="18" T="13" V="MO" W="15" U="1">540</Rep>
// <Rep D="SSW" F="12" G="29" H="93" Pp="65" S="11" T="14" V="GO" W="12" U="1">720</Rep>
// <Rep D="SSW" F="12" G="18" H="90" Pp="20" S="7" T="13" V="VG" W="7" U="1">900</Rep>
// <Rep D="SSW" F="11" G="11" H="90" Pp="15" S="4" T="12" V="VG" W="7" U="0">1080</Rep>
// <Rep D="SW" F="10" G="13" H="88" Pp="14" S="7" T="11" V="VG" W="7" U="0">1260</Rep>
// </Period>
// <Period type="Day" value="2012-11-21Z">
// <Rep D="S" F="10" G="9" H="91" Pp="11" S="4" T="11" V="VG" W="7" U="0">0</Rep>
// <Rep D="SSW" F="10" G="9" H="91" Pp="10" S="4" T="10" V="VG" W="7" U="0">180</Rep>
// <Rep D="SSW" F="9" G="7" H="87" Pp="6" S="4" T="9" V="VG" W="2" U="0">360</Rep>
// <Rep D="WSW" F="8" G="7" H="83" Pp="4" S="4" T="9" V="VG" W="3" U="1">540</Rep>
// <Rep D="SW" F="10" G="18" H="71" Pp="2" S="9" T="12" V="EX" W="1" U="1">720</Rep>
// <Rep D="SSW" F="10" G="13" H="72" Pp="1" S="7" T="11" V="EX" W="1" U="1">900</Rep>
// <Rep D="S" F="8" G="18" H="80" Pp="2" S="9" T="10" V="EX" W="2" U="0">1080</Rep>
// <Rep D="S" F="9" G="22" H="84" Pp="2" S="11" T="11" V="EX" W="2" U="0">1260</Rep>
// </Period>
// <Period type="Day" value="2012-11-22Z">
// <Rep D="S" F="9" G="25" H="86" Pp="2" S="13" T="11" V="VG" W="2" U="0">0</Rep>
// <Rep D="S" F="9" G="27" H="86" Pp="4" S="16" T="12" V="VG" W="2" U="0">180</Rep>
// <Rep D="S" F="8" G="31" H="86" Pp="4" S="18" T="12" V="VG" W="0" U="0">360</Rep>
// <Rep D="S" F="8" G="36" H="82" Pp="9" S="20" T="12" V="VG" W="3" U="1">540</Rep>
// <Rep D="S" F="9" G="40" H="74" Pp="9" S="22" T="13" V="VG" W="3" U="1">720</Rep>
// <Rep D="S" F="8" G="38" H="75" Pp="20" S="20" T="12" V="VG" W="3" U="1">900</Rep>
// <Rep D="S" F="8" G="36" H="85" Pp="58" S="20" T="11" V="GO" W="12" U="0">1080</Rep>
// <Rep D="S" F="7" G="18" H="88" Pp="61" S="11" T="9" V="GO" W="12" U="0">1260</Rep>
// </Period>
// <Period type="Day" value="2012-11-23Z">
// <Rep D="SSW" F="7" G="16" H="91" Pp="61" S="11" T="9" V="GO" W="12" U="0">0</Rep>
// <Rep D="SSW" F="6" G="16" H="93" Pp="55" S="11" T="8" V="GO" W="12" U="0">180</Rep>
// <Rep D="SSW" F="5" G="13" H="93" Pp="39" S="9" T="7" V="GO" W="9" U="0">360</Rep>
// <Rep D="WSW" F="6" G="11" H="89" Pp="18" S="7" T="8" V="VG" W="1" U="1">540</Rep>
// <Rep D="NW" F="8" G="16" H="82" Pp="11" S="9" T="10" V="VG" W="3" U="1">720</Rep>
// <Rep D="WNW" F="7" G="13" H="81" Pp="16" S="9" T="9" V="VG" W="3" U="1">900</Rep>
// <Rep D="WNW" F="4" G="11" H="89" Pp="15" S="9" T="7" V="VG" W="0" U="0">1080</Rep>
// <Rep D="WNW" F="3" G="11" H="91" Pp="15" S="9" T="6" V="VG" W="0" U="0">1260</Rep>
// </Period>
// </Location>
// </DV>
// </SiteRep>

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForecastResponse {
    pub site_rep: SiteRep,
}

#[derive(Debug, Deserialize)]
pub struct SiteRep {
    #[serde(rename = "Wx")]
    pub wx: Params,
    #[serde(rename = "DV")]
    pub dv: Dv,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(rename = "Param")]
    pub param: Vec<Param>,
}

#[derive(Debug, Deserialize)]
pub struct Param {
    pub name: String,
    pub units: String,
    #[serde(rename = "$")]
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dv {
    #[serde(rename = "dataDate")]
    pub data_date: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "Location")]
    pub location: ForecastLocation,
}

#[derive(Debug, Deserialize)]
pub struct ForecastLocation {
    pub i: String,
    pub lat: String,
    pub lon: String,
    pub name: String,
    pub country: String,
    pub continent: String,
    #[serde(rename = "Period")]
    pub period: Vec<ForecastPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastPeriod {
    #[serde(rename = "type")]
    pub period_type: String,
    pub value: String,
    #[serde(rename = "Rep")]
    pub rep: Vec<Rep>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rep {
    pub pp: String,
    pub s: String,
    pub d: String,
    pub w: String,
    pub h: String,
    pub t: String,
    #[serde(rename = "$")]
    pub dollar: String,
    pub g: String,
    pub u: String,
    pub v: String,
    pub f: String,
}
