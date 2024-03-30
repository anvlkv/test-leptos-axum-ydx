use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="text-indigo-100 text-4xl p-12">
            <div>
                <i class="fa-solid fa-spinner fa-spin-pulse"></i>
            </div>
        </div>
    }
}
