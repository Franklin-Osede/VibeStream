use vibestream_types::*;

pub mod zkp;
pub mod service;

pub use service::ZkService;

// Placeholder para funciones ZK
pub struct ZkProof {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
} 