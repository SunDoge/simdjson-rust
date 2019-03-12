pub mod parsed_json;
pub mod json_parser;
pub mod error;
pub mod parsed_json_iterator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
