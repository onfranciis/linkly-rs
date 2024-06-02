use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IURL {
    pub id: i32,
    pub long: String,
    pub short: String,
    pub date: String,
}

pub fn instantiate_table_query() -> &'static str {
    r#"
        CREATE TABLE IF NOT EXISTS url (
            id SERIAL PRIMARY KEY,
            short VARCHAR(10) NOT NULL UNIQUE,
            long VARCHAR(255) NOT NULL UNIQUE,
            date VARCHAR(255) NOT NULL
        )
        "#
}
