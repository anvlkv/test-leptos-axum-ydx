use leptos::*;

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use bcrypt::verify;

    use crate::ctx::{auth, pool};
    use crate::user::{User, UserPasshash};

    let pool = pool()?;
    let auth = auth()?;

    let (user, UserPasshash(expected_passhash)) =
        User::get_from_username_with_passhash(username, &pool)
            .await
            .ok_or_else(|| ServerFnError::new("Пользователь не найден."))?;

    match verify(password, &expected_passhash)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Проверьте введенные логин и пароль.".to_string(),
        )),
    }
}
