mod task;
mod database_type;
mod sql_parser;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let result = sql_parser::run(&args);
    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}