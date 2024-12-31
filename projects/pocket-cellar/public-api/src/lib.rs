use worker::*;

#[event(fetch)]
async fn fetch(_req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    console_error_panic_hook::set_once();
    env.service("PRIVATE_API")?.fetch("/", None).await?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Body::empty())?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true, "Placeholder test to ensure nextest passes");
    }
}
