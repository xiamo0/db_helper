mod args_util;
mod database_type;
mod sqlx_util;
mod task;
mod util;

fn main() {
    let result = util::run();
    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
