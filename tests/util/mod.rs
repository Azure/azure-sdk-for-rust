use std::env;

pub fn get_from_env(varname: &str) -> String {
    match env::var(varname) {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set {} env variable first!", varname);
        }
    }
}
