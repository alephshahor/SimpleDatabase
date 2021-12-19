#[cfg(test)]
mod tests {
    use crate::structures;
    use crate::constants;

    #[test]
    fn table_insert_row() {
        let r = structures::Row {
            id: 0,
            username: String::from("John"),
            email: String::from("John@email.com")
        };
        let mut t = structures::create_table();
        let status = t.insert_row(r);
        assert_eq!(status, structures::TransactionStatus::Success);
    }

    fn create_row(idx: i32) -> structures::Row {
        return structures::Row {
            id: idx as i32,
            username: String::from("John"),
            email: String::from("John@email.com")
        };
    }

    #[test]
    fn page_reaches_limit() {
        let mut t = structures::create_table();
        for i in 0 .. constants::N_PAGES {
            for j in 0 .. constants::N_ROWS - 2 {
                let r = create_row(j as i32);
                let status = t.insert_row(r);
                assert_eq!(status, structures::TransactionStatus::Success);
            }
            let r = create_row((constants::N_ROWS - 1) as i32);
            let status = t.insert_row(r);
            assert_eq!(status, structures::TransactionStatus::Success);
        }
        let r = create_row((constants::N_ROWS - 1) as i32);
        let status = t.insert_row(r);
        assert_eq!(status, structures::TransactionStatus::Full);
    }

    #[test]
    fn table_select_row() {
        let mut t = structures::create_table();
        let status = t.insert_row(
            create_row((0) as i32)
        );
        let mut r = t.select_row(0);
        match r {
            Some(r) => {
                assert_eq!(r.id, 0);
                assert_eq!(r.username, "John");
                assert_eq!(r.email, "John@email.com");
            },
            None => ()
        }
        r = t.select_row(1);
    }
}