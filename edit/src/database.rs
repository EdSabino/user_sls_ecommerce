pub struct DatabaseName;

#[cfg(not(test))]
impl DatabaseName {
    pub fn get_database_name() -> String {
        println!("cai aqui");
        "users".to_string()
    }
}

#[cfg(test)]
impl DatabaseName {
    pub fn get_database_name() -> String {
        println!("que isso");
        "users_test".to_string()
    }
}