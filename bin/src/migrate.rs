use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use dbs::mysqlx::migrator::*;

pub fn migrate(tables: &Vec<String>){
    let conf = config::load_config().unwrap();

    let mysql_url = format!("mysql://{}:{}@{}:{}/{}",
        conf.mysql.username,
        conf.mysql.password,
        conf.mysql.host,
        conf.mysql.port,
        conf.mysql.database
    );

    // 使用 actix_rt 来运行异步代码
    actix_rt::System::new().block_on(async {

        let db: DatabaseConnection = Database::connect(mysql_url).await.unwrap();
        let schema_manager = SchemaManager::new(&db);
        let data = Migrator::migrations();
        for item in data{

            if tables.len() > 0{
                // 判断是否是指定数据表
                for table in tables{
                    if item.name().contains(table.as_str()){
                        item.up(&schema_manager).await.unwrap();
                        println!("Up table {}", item.name());
                        break;
                    }
                }
            }else{
                item.up(&schema_manager).await.unwrap();
                println!("Up table {}", item.name());
            }

        }
    });

}