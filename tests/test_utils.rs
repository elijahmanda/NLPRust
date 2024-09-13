#[cfg(test)]
mod tests {
    use nlp_rust::utils::sequences::missing_indexes;

    #[test]
    fn test_missing_indexes() {
        let spans = vec![(0, 5), (10, 15)];
        let missing = missing_indexes(spans, 20);
        assert_eq!(missing, vec![(5, 10), (15, 20)]);

        let spans = vec![(0, 3), (3, 6), (6, 10)];
        let missing = missing_indexes(spans, 10);
        assert!(missing.is_empty());
    }

    #[test]
    fn test_missing_indexes_edge_cases() {
        let spans = vec![];
        let missing = missing_indexes(spans, 10);
        assert_eq!(missing, vec![(0, 10)]);

        let spans = vec![(0, 10)];
        let missing = missing_indexes(spans, 10);
        assert!(missing.is_empty());
    }
}
