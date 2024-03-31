use std::collections::HashMap;

use common::{models, moneys::Moneys, user::User, IdType};
use leptos::*;
use leptos_router::A;

use crate::users::user_name_short;

#[component]
pub fn ReportsSummary(#[prop(into)] reports: Signal<Vec<models::EntryWithUser>>) -> impl IntoView {
    let summary = Signal::derive(move || {
        let grouped = reports().into_iter().fold(
            HashMap::<IdType, (User, Moneys)>::new(),
            |mut acc, entry| {
                let ge = acc.entry(entry.user.id).or_insert((entry.user, Moneys(0)));
                ge.1 .0 += entry.revenue.0;

                acc
            },
        );

        let mut agg = grouped.values().cloned().collect::<Vec<_>>();

        agg.sort_by(|(a_user, _), (b_user, _)| {
            a_user
                .family_name
                .partial_cmp(&b_user.family_name)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        agg
    });

    view! {
        <table class="w-full">
            <thead class="border-solid border-b border-slate-500 font-bold text-left">
                <tr>
                    <th class="p-2 pl-8">{"Менеджер"}</th>
                    <th class="p-2">{"Выручка"}</th>
                    <th class="p-2 pr-8 text-right">
                        <i class="fa-solid fa-ellipsis-vertical"></i>
                    </th>
                </tr>
            </thead>
            <tbody>
                <For each=summary key=|(u, _)| u.id let:entry>
                    <tr class="border-solid border-b border-slate-500">
                        <td class="p-2 pl-8">
                            <A class="text-indigo-500" href={format!("/reports?user_id={}", entry.0.id)}>{user_name_short(&entry.0)}</A>
                        </td>
                        <td class="p-2">{format!("{}", entry.1)}</td>
                    </tr>
                </For>
            </tbody>
        </table>
    }
}
