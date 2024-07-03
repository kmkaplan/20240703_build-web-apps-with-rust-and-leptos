use ev::MouseEvent;
use leptos::*;

#[component]
pub fn Child<I, D>(
    counter: ReadSignal<i16>,
    on_increment: I,
    on_decrement: D,
) -> impl IntoView 
where I: Fn(MouseEvent) + 'static,
    D: Fn(MouseEvent) + 'static,
{
    view! {
            <div style="border: 1px solid black; margin: 4px">
                <h3>Child closure instead of callback</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=on_increment>Child +1</button>
                    <button type="button" on:click=on_decrement>Child -1</button>
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
        <p>Parent Child Closure instead of Callback</p>
        <Child counter=counter on_increment=increment_counter on_decrement=decrement_counter/>
    }
}