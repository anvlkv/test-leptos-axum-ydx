use common::{
    handlers::{get_user, ListReports, NewReport, UpdateReport},
    models::{entry::month_range, Entry},
    perms::MANAGE_USERS,
    user::{self, User},
    Datelike, IdType,
};
use leptos::*;
use leptos_router::{use_params, ActionForm, Params, A};

use crate::{
    calendar::Calendar,
    loading::Loading,
    users::{user_name_short, UserDropdown},
};

#[component]
pub fn Reports() -> impl IntoView {
    let list_reports = create_server_action::<ListReports>();

    let rw_dates_year = create_rw_signal(None);
    let rw_dates_month = create_rw_signal(None);

    let view_year = rw_dates_year.read_only();
    let view_month = rw_dates_month.read_only();

    let (view_user, set_view_user) = create_signal(None);

    let app_user = use_context::<Signal<user::User>>().unwrap();

    let user = create_resource(
        move || view_user().unwrap_or_else(|| app_user().id),
        |id| async move { common::handlers::get_user(id).await.unwrap_or_default() },
    );

    let reports = create_resource(
        move || {
            (
                list_reports.version().get(),
                view_year(),
                view_month(),
                view_user(),
            )
        },
        move |(_, year, month, user_id)| async move {
            match (year, month) {
                (Some(y), Some(m)) => Some(common::handlers::list_reports(y, m, user_id).await),
                _ => None,
            }
        },
    );

    let current_user = Signal::derive(move || view_user().unwrap_or_else(|| app_user().id));

    let title = move || {
        if app_user().permissions.contains(MANAGE_USERS) {
            view! {
                <UserDropdown
                    change={move|user_id| {
                        set_view_user(Some(user_id))
                    }}
                    label_text="Отчеты менеджера:"
                    current_user={current_user}
                />
            }
            .into_view()
        } else {
            view! {<span class="block">{format!(
                "Отчеты менеджера: {}",
                user_name_short(&user().unwrap_or_default())
            )}</span>}
            .into_view()
        }
    };

    let table = move || {
        match reports().flatten() {
        Some(Ok(reports)) => {
            view!{
                    <table class="w-full">
                        <thead class="border-solid border-b border-slate-500 font-bold text-left">
                            <tr>
                                <th class="p-2 pl-8">{"Дата"}</th>
                                <th class="p-2">{"Адрес"}</th>
                                <th class="p-2">{"Выручка"}</th>
                                <th class="p-2 pr-8 text-right">
                                    <i class="fa-solid fa-ellipsis-vertical"></i>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <For each=move || reports.clone() key=|u| u.id let:report>
                                <tr class="border-solid border-b border-slate-500">
                                    <td class="p-2 pl-8">{report.date.format("%d.%m.%Y").to_string()}</td>
                                    <td class="p-2">{report.address}</td>
                                    <td class="p-2">{format!("{:.2}₽", report.revenue.0 as f64 / 100.0)}</td>
                                    <td class="p-2 pr-6 text-right">
                                        <Show when=|| true>
                                            <A href=format!("{}",report.id) class="px-2 py-1 border border-solid border-slate-500 rounded-sm">
                                                <i title="Редактировать" class="fa-solid fa-pen-to-square"></i>
                                            </A>
                                        </Show>
                                    </td>
                                </tr>
                            </For>
                        </tbody>
                    </table>
                }.into_view()
        }
        Some(Err(err)) => {
            let err = format!("Ошибка: {}", err);
            view! {
                <p class="text-pink-600 pb-2">{err}</p>
            }
            .into_view()
        }
        None => {
            view! {
                <Loading/>
            }
            .into_view()
        }
    }
    };

    view! {
        <Transition fallback=Loading>
            <h2 class="w-full flex justify-between text-xl px-4 pt-8 pb-2 bg-slate-50 dark:bg-slate-700">
                {title}
            </h2>
            <Calendar year=rw_dates_year month=rw_dates_month />
            {table}
        </Transition>
    }
}

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

    let form_content = view! {
        <Transition fallback=Loading>
            <hr class="my-2"/>
            <h3 class="text-lg mb-2">"Данные менеджера:"</h3>
            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="text"
                    placeholder="Фамилия"
                    readonly
                    maxlength="250"
                    name="family_name"
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
                    name="name"
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
                    name="patronym"
                    value=move || current_user().patronym
                    class="w-full text-base rounded px-4 py-2 !bg-transparent !text-inherit dark:!text-inherit border border-slate-300 dark:border-slate-700"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Отчество:"</span>
            </label>
            <hr class="my-2"/>
            <h3 class="text-lg mb-2">"Данные отчета:"</h3>

            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="date"
                    step="0.01"
                    placeholder="Дата"
                    name="date"
                    value=move || report_data().unwrap_or_default().date.format("%Y-%m-%d").to_string()
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
                    placeholder="Адрес точки"
                    name="address"
                    autocomplete="address"
                    value=move || report_data().unwrap_or_default().address
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Адрес точки:"</span>
            </label>

            <label class="w-full pb-8 flex flex-col-reverse">
                <input
                    type="number"
                    // step="0.01"
                    placeholder="Выручка"
                    name="revenue"
                    value=move || report_data().unwrap_or_default().revenue.0 as f64 / 100.0
                    class="w-full text-xl rounded p-4 !bg-transparent !text-inherit dark:!text-inherit border border-slate-500"/>
                <span class="z-10 ml-3 px-1 mr-auto -mb-3 bg-slate-200 dark:bg-slate-800 inline-block">"Выручка, ₽:"</span>
            </label>
        </Transition>
    };

    params.with(|params| { match params.as_ref().map(|p| p.id).ok().flatten() {
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
    }})
}
