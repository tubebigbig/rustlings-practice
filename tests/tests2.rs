// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// I AM DONE

#[derive(Debug, PartialEq)]
enum STATUS {
    SUCCESS,
    FAIL,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: u8,
    y: u8,
}

#[cfg(test)]
mod tests {
    use Point;
    use STATUS;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(vec![1], vec![1]);
        assert_eq!(1, 1);
        assert_eq!("c", "c");
        assert_eq!("Bar", "Bar".to_owned());
        assert_eq!([1, 2, 3], [1, 2, 3]);
        assert_eq!(STATUS::SUCCESS, STATUS::SUCCESS);
        assert_eq!(Point { x: 10, y: 10 }, Point { x: 10, y: 10 });
    }
}
