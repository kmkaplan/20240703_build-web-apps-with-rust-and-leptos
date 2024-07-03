use ev::MouseEvent;
use leptos::*;

#[component]
pub fn Child(
    counter: ReadSignal<i16>,
    #[prop(into)] on_increment: Callback<MouseEvent>,
    #[prop(into)] on_decrement: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
            <div style="border: 1px solid black; margin: 4px">
                <h3>Child callback</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=on_increment>Parent +1</button>
                    <button type="button" on:click=on_decrement>Parent -1</button>
                </div>
            </div>
    }
}
#[component]
pub fn Parent() -> impl IntoView {
    let (counter, set_counter) = create_signal::<i16>(0);
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <Child counter=counter on_increment=increment_counter on_decrement=decrement_counter/>
    }
}