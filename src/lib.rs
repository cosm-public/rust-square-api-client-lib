
pub mod api;
pub mod config;
pub mod http;
pub mod models;
mod square_client;

pub use square_client::SquareClient;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
