use dioxus::prelude::*;
use tracing::debug;

// Paths

// Asset constants for web resources
pub const DATABASE: &str = concat!("assets/", env!("CARGO_PKG_NAME"), ".db");
pub const FAVICON: Asset = asset!("assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("assets/main.css");
pub const HEADER_SVG: Asset = asset!("assets/header.svg");

fn main() {
  //{ Create the database }
  // let db = rusqlite::Connection::open(DATABASE).expect(
  //   "Failed to open
  // database"
  // );
  // db.execute(
  //   "CREATE TABLE app ( id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
  //   ()
  // )
  // .expect("Failed to create table");

  //{ Launch the application}
  dioxus::launch(App)
}

#[component]
fn App() -> Element {
  // A signal to store the Database connection
  // let con = use_signal(|| rusqlite::Connection::open(DATABASE).unwrap());

  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    Main {}
  }
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        //{ Open the database from the persisted file }
        let conn = rusqlite::Connection::open(DATABASE)
        .expect("Failed to open database");

        //{ Create the "todo" table if it doesn't already exist }
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).expect("Failed to create table");

        // Return the connection
        conn
    };
}

#[component]
pub fn Main() -> Element {
  let mut item: Signal<String> = use_signal(|| String::new());
  let mut items: Signal<Vec<String>> = use_signal(Vec::<String>::new);

  rsx! {
    div {
      div { class: "header",
        input {
          r#type: "text",
          class: "input",
          value: "{item}",
          oninput: move |event| {
              item.set(event.value());
          },
          onkeydown: move |event| {
              if event.code().to_string() == "Enter".to_string() {
                  debug!(" Item: {}", item);
                  items.write().push(item());
                  item.set(String::new());
                  debug!("Items: {:?}", items);
              }
          },
          placeholder: "Add your todo item here...",
        }
        button { class: "btn_add", "Add" }
        div {
          for item in items.iter() {
            div { class: "todo-item",
              label { {item.to_string()} }
              button { class: "btn_del", "Delete" }
            }
          }
        }
      }
    }
  }
}

#[server]
async fn save_todo_item(image: String) -> Result<(), ServerFnError> {
  DB.with(|f| f.execute("INSERT INTO todo (url) VALUES (?1)", &[&image]))?;
  Ok(())
}
