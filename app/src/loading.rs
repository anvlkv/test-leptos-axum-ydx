use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="text-indigo-100 text-4xl">
            <i class="fa-solid fa-spinner fa-spin-pulse"></i>
        </div>
    }
}
