

cfg_if::cfg_if!{
    if #[cfg(feature = "ssr")] {
        use crate::app::models::Person;
        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
        use once_cell::sync::Lazy;
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::Surreal;
        use surrealdb::opt::auth::Root;

        pub async fn open_db_connection(){
            DB.connect::<Ws>("127.0.0.1:8000").await;

            let  _ = DB.signin(Root{
                username: "root",
                password: "root",
            })
            .await;
            DB.use_ns("surreal").use_db("person").await;

        }
        pub async fn get_all_person() -> Option<Vec<Person>>{
            open_db_connection().await;
            let get_all_person = DB.query("SELECT * FROM person ORDER BY joined_date DESC;").await;
            DB.invalidate().await;
            match get_all_person{
                Ok(mut res) => {
                    let found = res.take(0);
                    match found {
                        Ok(found_person) => Some(found_person),
                        Err(_) => None,
                    }
                },
                Err(_) => None,
            }
        }
        pub async fn add_person(new_person:Person) -> Option<Person> {
            open_db_connection().await;
            let result = DB.create(("person", new_person.uuid.clone()))
            .content(new_person)
            .await;

            let _ = DB.invalidate().await;
            match result {
                Ok(created_person) => created_person,
                Err(_) => None,
            }
        }
    }
}