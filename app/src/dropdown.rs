use std::{fmt::Display, hash::Hash, str::FromStr};

use leptos::*;

#[component]
pub fn Dropdown<T>(
    #[prop(into)] current_option: RwSignal<Option<T>>,
    #[prop(into)] label_text: String,
    #[prop(into)] options: Signal<Vec<(T, String)>>,
    #[prop(into)] name: String,
) -> impl IntoView
where
    T: PartialEq + Eq + Clone + Copy + FromStr + Hash + Display + 'static,
{
    view! {
        <label class="w-full flex flex-col-reverse">
            <select on:change={move |ev| {
                let val: Option<T> = event_target_value(&ev).parse().ok();
                current_option.set(val)
            }} name={name.clone()} class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500">
                <For each={options} key={|val| val.0} let:data>
                    <option value={data.0.to_string()} selected={move || current_option.get().map(|id| id == data.0).unwrap_or(false)}>
                        {data.1}
                    </option>
                </For>
            </select>
            <span class="z-10 ml-3 text-base px-1 mr-auto -mb-3 bg-slate-50 dark:bg-slate-700 inline-block">{label_text.clone()}</span>
        </label>
    }
}
