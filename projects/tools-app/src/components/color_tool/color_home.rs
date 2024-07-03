use leptos::*;
use crate::repositories::color_repository::{all_colors, AppendColor, RemoveColor};
use crate::components::color_tool::{color_form::ColorForm, color_list::ColorList};
#[component]
pub fn ColorHome() -> impl IntoView {
    let append_color = create_server_action::<AppendColor>();
    let remove_color = create_server_action::<RemoveColor>();
    let color_resource = create_resource(
        move || (append_color.version().get(), remove_color.version().get()),
        |_| all_colors(),
    );
    view! {
        <div>
            <header>
                <h2>ColorTool</h2>
                <ColorList color_resource=color_resource remove_color=remove_color />
                <ColorForm append_color=append_color /> 
            </header>
        </div>
    }
}