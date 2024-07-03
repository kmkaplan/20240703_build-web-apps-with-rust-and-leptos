use leptos::*;

#[component]
pub fn Child(
    counter: ReadSignal<i16>,
    set_counter: WriteSignal<i16>
) ->impl IntoView {
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);
    view! {
            <div style="border: 1px solid black; margin: 4px">
                <h3>Child write signal</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=increment_counter>Parent +1</button>
                    <button type="button" on:click=decrement_counter>Parent -1</button>
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
        <div>
            <div style="border: 1px solid black; margin: 4px">
                <h3>Parent write signal</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button type="button" on:click=increment_counter>Parent +1</button>
                    <button type="button" on:click=decrement_counter>Parent -1</button>
                </div>
            </div>
            <Child counter=counter set_counter=set_counter />
        </div>
    }
}