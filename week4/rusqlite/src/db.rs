use sqlite::{Connection, Result, State};

#[derive(Debug)]
struct Person {
    id: i64,
    name: String,
    data: String,
}

pub fn run() -> Result<()> {
    let conn = Connection::open("my_database.db")?;

    create_table(&conn)?;
    insert_person(&conn, "Alice", "Some data")?;
    insert_person(&conn, "Bob", "More data")?;

    let persons = query_persons(&conn)?;
    for person in persons {
        println!("Found person {:?}", person);
    }

    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id    INTEGER PRIMARY KEY,
                  name  TEXT NOT NULL,
                  data  BLOB
                  )",
    )?;
    Ok(())
}

fn insert_person(conn: &Connection, name: &str, data: &str) -> Result<()> {
    let mut statement = conn.prepare("INSERT INTO person (name, data) VALUES (?1, ?2)")
        .unwrap();

    statement.bind((1, name));
    statement.bind((2, data));

    statement.next().expect("Error inserting person");
    Ok(())
}

fn query_persons(conn: &Connection) -> Result<Vec<Person>> {
    let query = "SELECT id, name, data FROM person";
    let mut stmt = conn.prepare(query).unwrap();

    let mut persons = Vec::new();
    while let Ok(State::Row) = stmt.next() {
        let id = stmt.read::<i64, _>(0).unwrap();
        let name = stmt.read::<String, _>(1).unwrap();
        let data = stmt.read::<String, _>(2).unwrap();
        println!("Found person: {} {} {}", id, name, data);
        persons.push(Person { id, name, data });
    }

    Ok(persons)
}
