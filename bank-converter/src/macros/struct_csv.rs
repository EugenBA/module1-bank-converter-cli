
#[macro_export]
macro_rules! csv_data {
    ($name: ident, $type:ty, $($field:ident),+) => {
        #[derive(Debug, Deserialize, Serialize)]
        pub struct $name {
            $($field: $type,)+
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($field: "".to_string()),+
                }
            }
        }
    };
}


