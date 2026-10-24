#[derive(::toasty::Model)]
#[key(id)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}
