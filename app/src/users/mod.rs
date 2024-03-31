mod edit;
mod list;

pub use edit::*;
pub use list::*;

pub fn user_name_short(user: &common::user::User) -> String {
    format!(
        "{} {}. {}",
        user.family_name,
        user.name.chars().next().unwrap_or_default(),
        user.patronym
            .as_ref()
            .map(|p| format!("{}.", p.chars().next().unwrap_or_default()))
            .unwrap_or_default()
    )
}
