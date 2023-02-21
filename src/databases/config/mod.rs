
mod env_config;
mod cassandra_config;
mod dynamodb_config;
mod redis_config;

use env_config::EnvConfig;
use std::{env};
use std::time::Duration;
use log::{debug, info};


#[derive(Clone, Debug)]
enum DatabaseType {
    Dynamodb(String),
    Cassandra(String),
    Redis(String)
}

#[derive(Clone, Debug)]
pub struct DbConfig {
    pub url: String,
    pub database_type: DatabaseType,
    pub ttl: Duration
}

/// IImporter is a importer type used to import the corresponding variables for the underlying databases used.
pub trait IImporter {
    fn get(&self, key: String) -> std::result::Result<String, &'static str>;
}

struct ConfigInitRequest<'a> {
    importer_var: &'a dyn IImporter,
    db_conf: DbConfig,
}

/// A config type that returns the Configuration for the database.
pub trait IConfig {
    fn create( init_req: ConfigInitRequest) -> Box<dyn IConfig> where Self: Sized;
}

impl DbConfig {
    pub(crate) fn new() -> Box<dyn IConfig> {

        let mut importer = match env::var_os("CONFIG_TYPE") {
            _ => {
                info!("Setting or Defaulting to ENV variables.");
                EnvConfig{}
            }
        };

        let database_type: String = IImporter::get(&importer, String::from("DATABASE_TYPE") )
            .expect("DATABASE_TYPE environment variable must be present.").to_lowercase();
        info!("Database Configuration is for {}.", database_type);

        let url: String = IImporter::get(&importer, String::from("URL"))
            .expect("URL environment variable must be present.").to_lowercase();
        debug!("URL is {}.",url);

        let ttl = IImporter::get(&importer, String::from("TTL"))
            .unwrap_or(String::from("").parse().unwrap())
            .parse::<u64>().unwrap_or( 2 * 24 * 60 * 60);
        debug!("TTL is {}.",ttl);

        let db_type: DatabaseType = match database_type.as_str() {
            "dynamodb" => {
                DatabaseType::Redis(String::from("redis"))
            }
            "cassandra" =>{
                DatabaseType::Cassandra(String::from("cassandra"))
            }
            "redis" =>{
                DatabaseType::Cassandra(String::from("redis"))
            }
            _ => {
                panic!("Database type: '{}' is not a supported database.",database_type)
            }
        };

        let new_db_config = DbConfig{
            url,
            database_type: db_type,
            ttl: Duration::new(ttl,0),
        };

        let req =  ConfigInitRequest {
            importer_var:  &importer,
            db_conf: new_db_config,
        };

        match db_type {
            DatabaseType::Dynamodb(_) => {
                 //return dynamodb_config;
                 panic!("non")
            }
            DatabaseType::Cassandra(_) => {
               // return cassandra_config;
               panic!("non")
            }
            DatabaseType::Redis(_) => {
                //return redis_config;
                panic!("non")
            }
            _ => {
                panic!("Database type: '{}' is not a supported database.",database_type)
            }
        }
    }
}
