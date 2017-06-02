#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub id: String,
    pub _rid: String,
    pub _ts: u64,
    pub _self: String,
    pub _etag: String,
    pub _colls: String,
    pub _users: String,
}
