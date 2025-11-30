/// Макрос создает структуру для документа CSV
///
///  # Пример
/// ```no_run
///
///  csv_data!(RowCsv, String, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);
///
#[macro_export]
macro_rules! csv_data {
    ($name: ident, $type:ty, $($field:ident),+) => {
        #[derive(Debug, Deserialize, Serialize)]
        pub(crate) struct $name {
            $($field: $type,)+
        }

        impl $name {
            /// Создает новый экземпляр структуры со значениями по умолчанию.
            ///
            /// # Примеры
            ///
            /// ```
            /// use your_crate::your_struct;
            ///
            /// let doc = your_struct::new();
            pub fn new() -> Self {
                Self {
                    $($field: "".to_string()),+
                }
            }
        }
    };
}


