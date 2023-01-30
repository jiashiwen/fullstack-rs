use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
fn main() {
    println!("diesel connect mysql security");
    let database_url ="mysql://rust_test:Git785230root@mysql-internet-cn-north-1-1221449f8fb94332.rds.jdcloud.com:3306/rust_test";
    let conn = MysqlConnection::establish(database_url).unwrap();
}
