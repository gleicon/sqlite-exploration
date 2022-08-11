use sqlite::Value;

fn main() {
    //let connection = sqlite::open(":memory:").unwrap();
    let connection = sqlite::open("tompero.sqlite").unwrap();

    connection
        .execute(
            "
        CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("SELECT * FROM users WHERE age > ?")
        .unwrap()
        .into_cursor()
        .bind(&[Value::Integer(50)])
        .unwrap();

    while let Some(Ok(row)) = cursor.next() {
        println!("name = {}", row.get::<String, _>(0));
        println!("age = {}", row.get::<i64, _>(1));
    }

    connection.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS posts USING fts5 (title, body);
       INSERT INTO posts(title, body) VALUES ('Learn SQlite FTS5','This tutorial teaches you how to perform full-text search in SQLite using FTS5');
       INSERT INTO posts(title, body) VALUES ('Advanced SQlite Full-text Search','Show you some advanced techniques in SQLite full-text searching');
        INSERT INTO posts(title, body) VALUES ('SQLite Tutorial','Help you learn SQLite quickly and effectively');",
    )
    .unwrap();
    println!("-------");

    let mut scursor = connection
        .prepare("SELECT * FROM posts WHERE posts MATCH ?")
        .unwrap()
        .into_cursor()
        .bind(&[Value::String("\"fts5\"".to_string())])
        .unwrap();
    println!("-------");

    while let Some(Ok(row)) = scursor.next() {
        println!("Title = {}", row.get::<String, _>(0));
        println!("Body = {}", row.get::<String, _>(1));
    }
    println!("-------");
}
