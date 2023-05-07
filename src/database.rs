pub fn create() {
    let conn = sqlite::open("actix.db").unwrap();

    conn.execute("
        CREATE TABLE IF NOT EXISTS shortened_links (
            full TEXT NOT NULL,
            short TEXT NOT NULL,
            clicks INTEGER
        )
    ").unwrap();
}