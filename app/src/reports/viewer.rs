use common::{
    perms::{EDIT_OWNED, VIEW_ALL, VIEW_OWNED},
    user::User,
    IdType,
};
use leptos::*;
use leptos_router::{use_query, Params};

use crate::{
    calendar::Calendar, dropdown::Dropdown, loading::Loading, reports::ReportsList,
    users::user_name_short,
};

#[derive(Params, PartialEq)]
struct ReportQuery {
    user_id: IdType,
}

#[component]
pub fn ReportsViewer() -> impl IntoView {
    let params = use_query::<ReportQuery>();

    let list_users = create_server_action::<common::handlers::ListUsers>();

    let app_user = use_context::<Signal<User>>().unwrap();
    let admin_permissions_guard = Signal::derive(move || app_user().permissions.contains(VIEW_ALL));

    let users = create_local_resource(
        move || (list_users.version().get(), admin_permissions_guard()),
        move |(_, all)| common::handlers::list_users(all),
    );

    let rw_view_user = create_rw_signal(None);

    create_effect(move |_| {
        if !admin_permissions_guard() {
            rw_view_user.set(Some(app_user().id))
        } else if let Some(id) = params.with(|params| params.as_ref().map(|p| p.user_id).ok()) {
            rw_view_user.set(Some(id))
        } else if let Some(user) = users
            .get()
            .map(|r| r.ok().map(|d| d.first().cloned()))
            .flatten()
            .flatten()
        {
            rw_view_user.set(Some(user.id))
        }
    });

    view! {
        <Suspense fallback=Loading>
            <Show when={move|| admin_permissions_guard()}>
                <div class="w-full flex flex-col text-xl px-4 pt-8 pb-2 bg-slate-50 dark:bg-slate-700">
                    {
                        move || {
                            let options = Signal::derive(move || {
                                let data = users().map(|u| u.ok()).flatten().unwrap_or_default();

                                data.into_iter()
                                    .map(|u| (u.id, user_name_short(&u)))
                                    .collect::<Vec<_>>()
                            });

                            view!{<Dropdown
                                name="user"
                                label_text="Отчеты менеджера:"
                                options={options}
                                current_option=rw_view_user
                            />}
                        }
                    }
                </div>
            </Show>
            <ReportUserDates view_user=rw_view_user/>
        </Suspense>
    }
}

#[component]
fn ReportUserDates(#[prop(into)] view_user: Signal<Option<IdType>>) -> impl IntoView {
    let list_dates = create_server_action::<common::handlers::ListDates>();

    let user_dates = create_local_resource(
        move || (list_dates.version().get(), view_user()),
        move |(_, view_user)| common::handlers::list_dates(view_user),
    );

    let rw_month = create_rw_signal(None);
    let rw_year = create_rw_signal(None);

    create_effect(move |_| {
        if let Some((y, m)) = user_dates
            .get()
            .map(|r| {
                r.ok()
                    .map(|d| d.last().and_then(|(y, mm)| mm.last().map(|m| (*y, *m))))
            })
            .flatten()
            .flatten()
        {
            rw_year.set(Some(y));
            rw_month.set(Some(m));
        }
    });

    view! {
        <Suspense fallback=Loading>
            <div class="w-full flex flex-col text-xl px-4 pt-8 pb-2 bg-slate-50 dark:bg-slate-700">
                {move || {
                    let options = Signal::derive(move || user_dates().map(|r| r.ok()).flatten().unwrap_or_default());

                    view!{
                        <Calendar
                            rw_year=rw_year
                            rw_month=rw_month
                            options={options}
                        />
                    }
                }}
            </div>
            {move || {
                let year = Signal::derive(move || rw_year().unwrap_or_default());
                let month = Signal::derive(move || rw_month().unwrap_or_default());
                view!{
                    <ReportsList year=year month=month user=view_user />
                }
            }}
        </Suspense>
    }
}
