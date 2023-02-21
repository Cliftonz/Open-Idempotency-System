use std::ops::Deref;
use crate::databases::config::{ConfigInitRequest, DbConfig, IConfig};

struct RedisConfig {
    db_config: DbConfig
}

impl Deref for RedisConfig {
    type Target = DbConfig;
    fn deref(&self) -> &Self::Target {
        &self.db_config
    }
}

impl IConfig for RedisConfig {
    fn create(mut init_req: ConfigInitRequest) -> Box<dyn IConfig> {

        Box::new(RedisConfig{
            db_config: init_req.db_conf,
        })

    }
}



