use leptos::*;

use uuid::Uuid;

use crate::component::select::SelectUuidName;
use crate::component::template::{OptionError, OptionLoading};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, training_plan::model::TrainingPlanQuery};

#[server(endpoint = "training-plan-select")]
async fn get_training_plan_select() -> Result<Vec<SelectUuidName>, ServerFnError> {
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    Ok(TrainingPlanQuery::option_list_id(&pool).await?)
}

#[component]
pub fn TrainingPlanSelect(#[prop(optional, into)] selected: Uuid) -> impl IntoView {
    let resource = Resource::once(get_training_plan_select);
    let response = move || {
        resource.and_then(|data| {
            data.iter()
                .map(|option| {
                    let id = option.id;
                    view! {
                        <option value=option.id.to_string() selected=move || id == selected>
                            {&option.name}
                        </option>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <label class="block mb-4">
            <span class="block mb-1 text-sm font-bold">"Training Plan"</span>
            <select
                name="training_plan_id"
                class="block py-2 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            >
                <Transition fallback=OptionLoading>
                    <ErrorBoundary fallback=|_| {
                        view! { <OptionError/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </select>
        </label>
    }
}
