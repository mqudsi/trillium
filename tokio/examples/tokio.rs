pub fn app() -> impl trillium::Handler {
    |conn: trillium::Conn| async move {
        let response = tokio::task::spawn(async {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            "successfully spawned a task"
        })
        .await
        .unwrap();
        conn.ok(response)
    }
}
pub fn main() {
    env_logger::init();
    trillium_tokio::run(app());
}
