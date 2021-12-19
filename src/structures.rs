    use crate::constants;

    #[derive(Debug)]
    pub(crate) struct Row {
        pub(crate) id: i32,
        pub(crate) username: String,
        pub(crate) email: String,
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub(crate) enum TransactionStatus {
        Success,
        Full,
        Error
    }

    struct Page {
        rows: Vec<Row>,
        current_row_idx: usize
    }

    impl Page {
        fn insert_row(&mut self, row: Row) {
            self.rows.insert(self.current_row_idx, row);
            self.current_row_idx += 1;
        }

        fn full(&self) -> bool {
            return self.current_row_idx == constants::N_ROWS - 1
        }
    }


    pub(crate) struct Table {
        pages: Vec<Page>,
        current_page_idx: usize
    }

    impl Table {
        pub(crate) fn insert_row(&mut self, row: Row) -> TransactionStatus {
            if self.table_is_full() {
                return TransactionStatus::Full
            }

            if self.page_is_full() {
                self.create_new_page()
            }

            self.pages[self.current_page_idx].insert_row(row);
            return TransactionStatus::Success
        }

        pub(crate) fn select_row(&self, n: usize) -> Option<&Row> {
            let n_page = n / constants::N_ROWS;
            let n_row = n % constants::N_ROWS;
            // You've to check the current idx of the page
            if self.pages[n_page].current_row_idx <= n_row {
                return None
            }
            return Some(&self.pages[n_page].rows[n_row]);
        }

        fn table_is_full(&self) -> bool {
            return self.current_page_idx == constants::N_PAGES - 1 && self.page_is_full()
        }

        fn page_is_full(&self) -> bool {
            return self.pages[self.current_page_idx].full()
        }

        fn create_new_page(&mut self) {
            self.current_page_idx += 1;
            self.pages.insert(self.current_page_idx, Page {
                rows: Vec::with_capacity(constants::N_ROWS),
                current_row_idx: 0
            });
        }
    }

    pub(crate) fn create_table() -> Table {
        let mut t = Table {
            pages: Vec::with_capacity(constants::N_PAGES),
            current_page_idx: 0
        };
        t.pages.insert(0,Page {
            rows: Vec::with_capacity(constants::N_ROWS),
            current_row_idx: 0
        });
        return t;
    }