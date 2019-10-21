#[derive(Serialize, Deserialize, Debug, Queryable, Responder)]
#[response(status = 500, content_type = "json")]
pub struct Error {
    pub message: String,
}
