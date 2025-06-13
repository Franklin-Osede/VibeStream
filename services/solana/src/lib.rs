use vibestream_types::*;

pub mod client;
pub mod handlers;
pub mod service;

pub use service::SolanaService;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
} 