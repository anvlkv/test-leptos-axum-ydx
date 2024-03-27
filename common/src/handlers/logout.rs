use leptos::*;

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::ctx::auth;

    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
