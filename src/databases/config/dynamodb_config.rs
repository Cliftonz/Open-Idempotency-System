// use std::ops::Deref;
// use crate::databases::config::{ConfigInitRequest, DbConfig, IConfig, IImporter};

// struct DynamoDBConfig {
//     table_name: String,
//     region: Option<String>,
//     db_config: DbConfig
// }

// impl Deref for DynamoDBConfig {
//     type Target = DbConfig;
//     fn deref(&self) -> &Self::Target {
//         &self.db_config
//     }
// }

// impl IConfig for DynamoDBConfig {
//     fn create( mut init_req: ConfigInitRequest) -> Box<dyn IConfig> {

//         let table_name = (*init_req.importer).get(String::from("TABLE_NAME"))
//             .expect("TABLE_NAME variable must be present with the Dynamodb configuration.").into_string().unwrap().into_string();

//         Box::new(DynamoDBConfig{
//             table_name,
//             region: match (*init_req.importer).get(String::from("REGION")){
//                 Some(val @ String) => val ,
//                 Err(_) => None
//             },
//             db_config: init_req.db_config,
//         })
//     }
// }
