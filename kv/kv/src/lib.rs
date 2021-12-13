mod pb;
mod error;
mod storage;
mod service;


pub use error::KvError;
pub use pb::abi::*;
pub use storage::*;
pub use service::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
