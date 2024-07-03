use leptos::*;
use leptos_router::*;

#[component]
pub fn ParentChildHome() -> impl IntoView {
    view! {
        <div>
            <h2>Parent child home</h2>
            <ul>
                <li><a href="/parent-child/write-signal">Write signal</a></li>
                <li><a href="/parent-child/callback">Callback</a></li>
                <li><a href="/parent-child/closure-instead-of-callback">Closure instead of Callback</a></li>
            </ul>
            <Outlet />
        </div>
    }
}