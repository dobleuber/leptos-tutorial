use leptos::*;

fn main() {
  console_error_panic_hook::set_once();
  leptos::mount_to_body(|| view! { <App /> });
}

#[component]
fn ProgressBar(
  #[prop(default = 100)] // default value
  max: u16,
  #[prop(into)]
  progress: Signal<i32>
) -> impl IntoView {
  view! {
    <progress
      max=max
      value=progress
    />
  }
}

#[component]
fn StaticList(elements: i32) -> impl IntoView {
  view! {
    <ul>
      { (0..elements).map(|i| view! { <li>{i}</li> }).collect_view() }
    </ul>
  }
}

#[component]
fn DynamicList(initial_length: usize) -> impl IntoView {
  let mut next_counter_id = initial_length;

  let initial_counters = (0..initial_length)
    .map(|id| (id, create_signal(id + 1)))
    .collect::<Vec<_>>();

  let (counters, set_counters) = create_signal(initial_counters);

  let add_counter = move |_| {
    let sig = create_signal(next_counter_id + 1);
    set_counters.update(|counters| {
      counters.push((next_counter_id, sig));
    });

    next_counter_id += 1;
  };

  view! {
    <div>
      <button on:click=add_counter>
        "Add Counter"
      </button>
      <ul>
        <For
          each=counters
          key=|counter| counter.0
          children= move |(id, (count, set_count))| {
            view! {
              <li>
                <button
                  on:click=move |_| set_count.update(|n| *n += 1)
                >
                  "Increment: " {count}
                </button>
                <button
                  on:click=move |_| set_counters.update(|counters| {
                    counters.retain(|(i, _)| i != &id);
                  })
                >
                  "Remove"
                </button>
              </li>
            }
          }
        />
      </ul>
    </div>
  }
}

#[component]
fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let (x, set_x) = create_signal(0);

  // Derived signal
  let double_count = move || count() * 2;
  view! {
    <button
      on:click={move |_| set_count(0)}
    >
      "Reset to Zero: "
    </button>
    <button
      on:click={move |_| set_count.update(|n| *n += 1)}
      class:red=move || count() % 2 == 0
    >
      "Add 1"
    </button>
    <button
      on:click={move |_| set_x.update(|n| *n += 10)}
      style="position: absolute"
      style:left=move || format!("{}px", x() + 160)
      style:background-color=move || format!("rgb({}, 100, 100)", x() % 255)
      style=("--colums", x)
    >
      "Add columns"
    </button>

    <p>{ "current count: " } {double_count}</p>
    <StaticList elements=5 />
    <DynamicList initial_length=3 />
    <ProgressBar progress=count />
    <ProgressBar progress=Signal::derive(double_count) />
  }
}