// use std::ops::Deref;
// use crate::databases::config::{ConfigInitRequest, DbConfig, IConfig, IImporter};

// struct CassandraConfig {
//     keyspace: String,
//     table_name: String,
//     db_config: DbConfig
// }

// impl Deref for CassandraConfig {
//     type Target = DbConfig;
//     fn deref(&self) -> &Self::Target {
//         &self.db_config
//     }
// }

// impl IConfig for CassandraConfig {
//     fn create(mut init_req: ConfigInitRequest) -> Box<dyn IConfig> {
        
//         let keyspace = (*init_req.importer).get(String::from("KEYSPACE"))
//             .expect("KEYSPACE environment variable must be present with the cassandra configuration.").into_string().unwrap().into_string();

//         let table_name = (*init_req.importer).get(String::from("TABLE_NAME"))
//             .expect("TABLE_NAME environment variable must be present with the cassandra configuration.").into_string().unwrap().into_string();

//         Box::new(CassandraConfig{
//             keyspace,
//             table_name,
//             db_config: init_req.db_config,
//         })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::databases::database::{DatabaseOption};
//     use super::*;

//     #[tokio::test]
//     pub async fn test_create_cassandra_config() {



//         let t = CassandraConfig::create();


//     }



// }