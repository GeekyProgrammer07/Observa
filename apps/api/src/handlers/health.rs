use poem::handler;

#[handler]
pub fn health_check() -> String {
    String::from("Server is Healthy")
}
#[cfg(test)]
mod tests {
    use super::health_check;
    use poem::{get, test::TestClient, Route};

    #[tokio::test]
    async fn health_check_reports_healthy() {
        let app = Route::new().at("/", get(health_check));
        let cli = TestClient::new(app);

        let resp = cli.get("/").send().await;
        resp.assert_status_is_ok();
        resp.assert_text("Server is Healthy").await;
    }
}
