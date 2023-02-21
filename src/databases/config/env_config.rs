use std::env;
use log::{debug, error};
use crate::databases::config::IImporter;

pub struct EnvConfig where Self: Sized {}

impl IImporter for EnvConfig {
    fn get(&self, key: String) -> std::result::Result<String, &'static str>{
        match env::var_os(key) {
            Some(val) =>{
                debug!("Key: {}, Value: {}", key, val.into_string().expect("Unable to decode to string") );
                Ok(val.into_string().unwrap())
            }
            None => {
                error!("{key} is not defined in the environment.");
                Err("{key} is not defined in the environment.")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use std::*;

    use crate::databases::config::IImporter;
    use crate::databases::config::env_config::EnvConfig;

    #[tokio::test]
    pub async fn test_get_active_env_variable() {

        env::set_var("KEY", "VALUE");

        let struct_val = EnvConfig{};

        assert_eq!(EnvConfig::get(&struct_val, String::from("KEY")).expect("msg"),"VALUE");

        env::set_var("KEY", " ");

    }
}
