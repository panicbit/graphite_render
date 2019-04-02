use chrono::prelude::*;

pub type Response = Vec<TargetResponse>;

#[derive(Deserialize, Debug)]
pub struct TargetResponse {
    target: String,
    datapoints: Vec<Datapoint>,
}

#[derive(Deserialize, Debug)]
pub struct Datapoint {
    value: Option<f64>,
    #[serde(with="chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
}
