use leptos::*;

#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    view! { cx,
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        // .into() converts `ReadSignal` to `Signal`
        <ProgressBar progress=count/>
        // use `Signal::derive()` to wrap a derived signal
        <ProgressBar progress=Signal::derive(cx, double_count)/>
    }
}
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}
