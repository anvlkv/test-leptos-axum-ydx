use common::{
    handlers::{NewReport, UpdateReport},
    models::{entry::month_range, Entry},
    moneys::Moneys,
    user::User,
    Datelike, IdType,
};
use leptos::*;
use leptos_router::{use_params, ActionForm, Params};

use crate::loading::Loading;

#[derive(Params, PartialEq)]
struct EditReporParams {
    id: Option<IdType>,
}

#[component]
pub fn EditReport() -> impl IntoView {
    let params = use_params::<EditReporParams>();

    let create_report = create_server_action::<NewReport>();
    let update_report = create_server_action::<UpdateReport>();

    let create_value = create_report.value();
    let has_create_error = move || create_value.with(|val| matches!(val, Some(Err(_))));
    let update_value = update_report.value();
    let has_update_error = move || update_value.with(|val| matches!(val, Some(Err(_))));

    let report_data = create_resource(
        move || params.with(|p| p.as_ref().map(|p| p.id).ok().flatten()),
        move |id: Option<IdType>| async move {
            match id {
                Some(id) => common::handlers::get_report(id).await.unwrap_or_default(),
                None => Entry::default(),
            }
        },
    );

    let current_user = use_context::<Signal<User>>().unwrap();

    let form_content = move || {
        view! {
            <hr class="my-2"/>
            <h3 class="text-lg mb-2">"Данные менеджера:"</h3>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Фамилия"
                    readonly
                    maxlength="250"
                    value=move || current_user().family_name
                    class="w-full text-base rounded px-4 py-2 !bg-transparent !text-inherit dark:!text-inherit border border-slate-300 dark:border-slate-700"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Фамилия:"</span>
            </label>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Имя"
                    readonly
                    maxlength="250"
                    value=move || current_user().name
                    class="w-full text-base rounded px-4 py-2 !bg-transparent !text-inherit dark:!text-inherit border border-slate-300 dark:border-slate-700"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Имя:"</span>
            </label>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Отчество"
                    readonly
                    maxlength="250"
                    value=move || current_user().patronym
                    class="w-full text-base rounded px-4 py-2 !bg-transparent !text-inherit dark:!text-inherit border border-slate-300 dark:border-slate-700"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Отчество:"</span>
            </label>
            <hr class="my-2"/>
            <h3 class="text-lg mb-2">"Данные отчета:"</h3>

            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="date"
                    placeholder="Дата"
                    name="date"
                    prop:value=move || report_data().unwrap_or_default().date.format("%Y-%m-%d").to_string()
                    min={move || {
                        let date = report_data().unwrap_or_default().date;
                        let (min_date, _) = month_range(date.year(), date.month());
                        min_date.format("%Y-%m-%d").to_string()
                    }}
                    max={move || {
                        let date = report_data().unwrap_or_default().date;
                        let (_, max_date) = month_range(date.year(), date.month());
                        max_date.format("%Y-%m-%d").to_string()
                    }}
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Дата:"</span>
            </label>

            <label class="w-full pb-8 flex flex-col-reverse">
                <textarea
                    prop:value={move || report_data().unwrap_or_default().address}
                    placeholder="Адрес точки"
                    name="address"
                    autocomplete="address"
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500">
                    // {move || report_data.get_untracked().unwrap_or_default().address}
                </textarea>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Адрес точки:"</span>
            </label>

            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="number"
                    step="0.01"
                    placeholder="Выручка"
                    name="revenue"
                    prop:value={move || {
                        let f_val:f64 = Moneys::from(report_data().unwrap_or_default().revenue).into();
                        f_val
                    }}
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Выручка, ₽:"</span>
            </label>
        }
    };

    let id_param =
        Signal::derive(move || params.with(|params| params.as_ref().map(|p| p.id).ok().flatten()));

    view! {
        <Transition fallback=Loading>
            {
                 move || match id_param() {
                    Some(id) => {
                        view! {
                            <ActionForm action=update_report
                                class="p-8 m-8 bg-slate-200 dark:bg-slate-800 rounded-lg"
                                attributes=vec![("autocomplete", Attribute::String("off".into()))]
                            >
                                <h1 class="text-2xl mb-12">"Редактирование отчета"</h1>
                                <input type="hidden" name="id" value=id.to_string()/>
                                {form_content}
                                <Show when=has_update_error>
                                    {move || {
                                        let err = format!("Ошибка: {}", update_value().unwrap().unwrap_err());
                                        view! {<p class="text-pink-600 pb-2">{err}</p>}
                                    }}
                                </Show>
                                <button type="submit" class="w-full mb-4 text-xl p-4 border border-solid border-slate-500 rounded">
                                    "Сохранить"
                                </button>
                            </ActionForm>
                        }.into_view()
                    }
                    None => {
                        view! {
                            <ActionForm action=create_report
                                class="p-8 m-8 bg-slate-200 dark:bg-slate-800 rounded-lg"
                                attributes=vec![("autocomplete", Attribute::String("off".into()))]
                            >
                                <h1 class="text-2xl mb-12">"Добавление нового отчета"</h1>
                                {form_content}
                                <Show when=has_create_error>
                                    {move || {
                                        let err = format!("Ошибка: {}", create_value().unwrap().unwrap_err());
                                        view! {<p class="text-pink-600 pb-2">{err}</p>}
                                    }}
                                </Show>
                                <button type="submit" class="w-full mb-4 text-xl p-4 border border-solid border-slate-500 rounded">
                                    "Добавить"
                                </button>
                            </ActionForm>
                        }.into_view()
                    }
                }
            }
        </Transition>
    }
}
