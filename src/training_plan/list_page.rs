use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
    ListPageHeaderWithCreate,
};
use crate::training_plan::create_page::{TrainingPlanCreate, TrainingPlanCreateForm};
use crate::training_plan::model::TrainingPlanQuery;
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "training-plan-list", input = GetUrl)]
pub async fn get_training_plan_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<TrainingPlanQuery>, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let count = TrainingPlanQuery::count(&pool, &search).await?;
    let results = TrainingPlanQuery::filter(&pool, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn TrainingPlanListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();
    let action_create = Action::<TrainingPlanCreate, _>::server();
    provide_context(action_create);

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || {
            (
                search(),
                order(),
                size(),
                page(),
                action_bulk_delete.version().get(),
                action_create.version().get(),
            )
        },
        |(search, order, size, page, _, _)| get_training_plan_list(search, order, size, page),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            let count = &data.count;
            let results = &data.results;
            if *count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = results.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                results
                    .iter()
                    .map(|data| {
                        view! { <TrainingPlanListItem data checked_items/> }
                    })
                    .collect_view()
            }
        })
    };
    let count = move || {
        resource.with(|res| {
            res.as_ref()
                .and_then(|data| data.as_ref().ok().map(|res| res.count))
        })
    };

    view! {
        <Title text="Training Plans"/>
        <main class="grid grid-cols-4 gap-4 p-4 lg:grid-cols-12">

            <div class="col-span-4 p-4 bg-white border lg:col-span-8">
                <ListPageHeaderWithCreate
                    title="Training Plans"
                    create_href="/training-plans/create"
                >
                    <Transition>{count}</Transition>
                </ListPageHeaderWithCreate>

                <section>
                    <Form method="get" action="" class="flex flex-wrap gap-2">
                        <FilterInput name="search" value=Signal::derive(search)/>
                        <FilterInput name="order" value=Signal::derive(order)/>
                    </Form>
                </section>

                <section class="grid mb-4 grid-cols-checkbox-8">
                    <AutoListHeader all_items checked_items>
                        "Training Plan"
                        ""
                        "Workouts"
                        "Exercises"
                        "Sets"
                        "Reps"
                        "Duration Weeks"
                        "Created"
                    </AutoListHeader>
                    <Transition fallback=ListLoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </section>

                <section class="flex flex-wrap">
                    <div>
                        <BulkDeleteForm
                            table="training_plan"
                            action=action_bulk_delete
                            checked_items
                        />
                    </div>
                    <div class="flex-1">
                        <Form method="GET" action="" class="contents">
                            <input type="hidden" name="search" value=search/>
                            <input type="hidden" name="order" value=order/>
                            <input type="hidden" name="page" value=page/>
                            <Transition>
                                <Paginator count/>
                            </Transition>
                        </Form>
                    </div>
                </section>

            </div>

            <div class="col-span-4">
                <section class="p-4 bg-white border">
                    <h2 class="text-xl font-bold">"Create Training Plan"</h2>
                    <TrainingPlanCreateForm/>
                </section>
            </div>

        </main>
    }
}

#[component]
pub fn TrainingPlanListItem<'a>(
    data: &'a TrainingPlanQuery,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    // let updated_at = format_datetime(&data.updated_at);
    let name = data.name.clone();
    let slug = data.slug.clone();

    view! {
        <div class="contents group">
            <div class="flex justify-center items-center border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex col-span-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                <A class="flex-1 p-2 aria-[current=page]:bg-amber-200" exact=true href=slug>
                    {name}
                </A>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.workout_count}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.exercise_count}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.set_count}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.rep_count}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.duration_weeks}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 truncate">
                {created_at}
            </div>
        </div>
    }
}
