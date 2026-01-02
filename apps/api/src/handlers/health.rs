use poem::handler;

#[handler]
pub fn health_check() -> String {
    String::from("Server is Healthy")
}