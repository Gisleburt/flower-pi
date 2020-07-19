use crate::met_api::forecast::{ForecastLocation, Dv, ForecastPeriod, Rep};
use std::convert::{TryFrom, TryInto};
use std::num::ParseIntError;

pub struct SaneLocation {
    pub id: u32,
    pub name: String,
    pub latitude: String,
    pub longitude: String,
}

impl TryFrom<ForecastLocation> for SaneLocation {
    type Error = ParseIntError;

    fn try_from(forecast_location: ForecastLocation) -> Result<Self, Self::Error> {
        Ok(Self {
            id: forecast_location.i.parse()?,
            name: forecast_location.name,
            latitude: forecast_location.lat,
            longitude: forecast_location.lon,
        })
    }
}

pub struct SaneForecastUnit {
    feels_like_temperature: String,
    wind_gust: String,
    screen_relative_humidity: String,
    temperature: String,
    visibility: String,
    wind_direction: String,
    wind_speed: String,
    max_uv_index: String,
    weather_type: String,
    precipitation_probability: String,
}

impl From<Rep> for SaneForecastUnit {
    fn from(rep: Rep) -> Self {
        Self {
            feels_like_temperature: rep.f,
            wind_gust: rep.g,
            screen_relative_humidity: rep.h,
            temperature: rep.t,
            visibility: rep.v,
            wind_direction: rep.d,
            wind_speed: rep.s,
            max_uv_index: rep.u,
            weather_type: rep.w,
            precipitation_probability: rep.pp,
        }
    }
}

