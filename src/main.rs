fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("Test");
    }

    #[test]
    fn read_env() {
        use dotenv::dotenv;

        dotenv().ok();

        let user = std::env::var("USER").expect("USER must be set.");

        println!("User = {}", user);
    }

    #[test]
    fn read_postgres_env() {
        use dotenv::dotenv;

        dotenv().ok();

        let user_res = std::env::var("DATABASE_URL");

        println!("User = {:?}", user_res);
    }
}
