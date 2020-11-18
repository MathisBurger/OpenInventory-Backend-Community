pub(crate) struct user {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) token: String,
    pub(crate) root: bool,
    pub(crate) mail: String,
    pub(crate) displayname: String,
    pub(crate) register_date: String,
    pub(crate) status: String
}

pub(crate) struct table {
    pub(crate) name: String
}

pub(crate) struct param {
    pub(crate) param: String
}