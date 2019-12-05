mod read_json;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        super::read_json::untyped_example();
        assert_eq!(2 + 2, 4);
    }
}
