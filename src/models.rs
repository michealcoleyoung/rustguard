struct PassWordEntry {
    site: String,
    email: String,
    username: String,
    password: String,
}

pub struct Vault {
    pub entries: Vec<PasswordEntry>,
}
