pub fn create() {
    let conn = sqlite::open("actix.db").unwrap();

    conn.execute("
        CREATE TABLE IF NOT EXISTS shortened_links (
            uid INTEGER PRIMARY KEY AUTOINCREMENT,
            full TEXT NOT NULL,
            short TEXT NOT NULL
        )
    ").unwrap();
}