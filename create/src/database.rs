pub struct DatabaseName;

#[cfg(test)]
impl DatabaseName {
    pub fn get_database_name() -> String {
        "users_test".to_string()
    }
}

#[cfg(not(test))]
impl DatabaseName {
    pub fn get_database_name() -> String {
        "users".to_string()
    }
}