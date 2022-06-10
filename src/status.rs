#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[derive(Debug, Clone, Copy)]
pub enum Status {
    Running,
    Finished,
    Failed,
}
