#[derive(Clone, Debug, Serialize)]
pub(crate) struct Transaction {
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) amount: f32,
}