use leptos::*;

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::ctx::auth;

    let auth = auth()?;

    auth.remember_user(false);
    auth.logout_user();
    leptos_axum::redirect("/login");

    Ok(())
}
