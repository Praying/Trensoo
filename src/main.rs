use diesel::prelude::*;
use dotenv::dotenv;

fn establish_connection()->MysqlConnection{
    dotenv().ok();
    std::env::set_var("DATABASE_URL","mysql://root:@localhost/taobaoke");
    let database_url = std::env::var(
        "DATABASE_URL"
    ).expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let conn=establish_connection();
}
