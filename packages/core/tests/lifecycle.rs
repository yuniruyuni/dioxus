#![allow(unused, non_upper_case_globals)]

//! Tests for the lifecycle of components.
use dioxus::prelude::*;
use dioxus_core as dioxus;
use dioxus_core::DomEdit::*;
use dioxus_core::ScopeId;

use dioxus_core_macro::*;
use dioxus_html as dioxus_elements;
use std::sync::{Arc, Mutex};

mod test_logging;

const IS_LOGGING_ENABLED: bool = true;
type Shared<T> = Arc<Mutex<T>>;

#[test]
fn manual_diffing() {
    struct AppProps {
        value: Shared<&'static str>,
    }

    static App: Component<AppProps> = |cx| {
        let val = cx.props.value.lock().unwrap();
        cx.render(rsx! { div { "{val}" } })
    };

    let value = Arc::new(Mutex::new("Hello"));
    let mut dom = VirtualDom::new_with_props(
        App,
        AppProps {
            value: value.clone(),
        },
    );

    let _ = dom.rebuild();

    *value.lock().unwrap() = "goodbye";

    let edits = dom.rebuild();

    log::debug!("edits: {:?}", edits);
}

#[test]
fn events_generate() {
    static App: Component = |cx| {
        let count = cx.use_hook(|_| 0);

        let inner = match *count {
            0 => {
                rsx! {
                    div {
                        onclick: move |_| *count += 1,
                        div {
                            "nested"
                        }
                        "Click me!"
                    }
                }
            }
            _ => todo!(),
        };

        cx.render(inner)
    };

    let mut dom = VirtualDom::new(App);
    let mut channel = dom.get_scheduler_channel();
    assert!(dom.has_work());

    let edits = dom.rebuild();
    assert_eq!(
        edits.edits,
        [
            CreateElement {
                tag: "div",
                root: 1,
            },
            NewEventListener {
                event_name: "click",
                scope: ScopeId(0),
                root: 1,
            },
            CreateElement {
                tag: "div",
                root: 2,
            },
            CreateTextNode {
                text: "nested",
                root: 3,
            },
            AppendChildren { many: 1 },
            CreateTextNode {
                text: "Click me!",
                root: 4,
            },
            AppendChildren { many: 2 },
            AppendChildren { many: 1 },
        ]
    )
}

#[test]
fn components_generate() {
    static App: Component = |cx| {
        let render_phase = cx.use_hook(|_| 0);
        *render_phase += 1;

        cx.render(match *render_phase {
            1 => rsx!("Text0"),
            2 => rsx!(div {}),
            3 => rsx!("Text2"),
            4 => rsx!(Child {}),
            5 => rsx!({ None as Option<()> }),
            6 => rsx!("text 3"),
            7 => rsx!({ (0..2).map(|f| rsx!("text {f}")) }),
            8 => rsx!(Child {}),
            _ => todo!(),
        })
    };

    static Child: Component = |cx| {
        cx.render(rsx! {
            h1 {}
        })
    };

    let mut dom = VirtualDom::new(App);
    let edits = dom.rebuild();
    assert_eq!(
        edits.edits,
        [
            CreateTextNode {
                text: "Text0",
                root: 1,
            },
            AppendChildren { many: 1 },
        ]
    );

    let edits = dom.hard_diff(ScopeId(0));
    assert_eq!(
        edits.edits,
        [
            CreateElement {
                tag: "div",
                root: 2,
            },
            ReplaceWith { root: 1, m: 1 },
        ]
    );

    let edits = dom.hard_diff(ScopeId(0));
    assert_eq!(
        edits.edits,
        [
            CreateTextNode {
                text: "Text2",
                root: 3,
            },
            ReplaceWith { root: 2, m: 1 },
        ]
    );

    let edits = dom.hard_diff(ScopeId(0));
    assert_eq!(
        edits.edits,
        [
            CreateElement { tag: "h1", root: 4 },
            ReplaceWith { root: 3, m: 1 },
        ]
    );

    let edits = dom.hard_diff(ScopeId(0));
    assert_eq!(
        edits.edits,
        [CreatePlaceholder { root: 5 }, ReplaceWith { root: 4, m: 1 },]
    );

    let edits = dom.hard_diff(ScopeId(0));
    assert_eq!(
        edits.edits,
        [
            CreateTextNode {
                text: "text 3",
                root: 6,
            },
            ReplaceWith { root: 5, m: 1 },
        ]
    );

    let edits = dom.hard_diff(ScopeId(0));
    assert_eq!(
        edits.edits,
        [
            CreateTextNode {
                text: "text 0",
                root: 7,
            },
            CreateTextNode {
                text: "text 1",
                root: 8,
            },
            ReplaceWith { root: 6, m: 2 },
        ]
    );

    let edits = dom.hard_diff(ScopeId(0));
    assert_eq!(
        edits.edits,
        [
            CreateElement { tag: "h1", root: 9 },
            ReplaceWith { root: 7, m: 1 },
            Remove { root: 8 },
        ]
    );
}

#[test]
fn component_swap() {
    static App: Component = |cx| {
        let render_phase = cx.use_hook(|_| 0);
        *render_phase += 1;

        cx.render(match *render_phase {
            0 => rsx!(
                div {
                    NavBar {}
                    Dashboard {}
                }
            ),
            1 => rsx!(
                div {
                    NavBar {}
                    Results {}
                }
            ),
            2 => rsx!(
                div {
                    NavBar {}
                    Dashboard {}
                }
            ),
            3 => rsx!(
                div {
                    NavBar {}
                    Results {}
                }
            ),
            4 => rsx!(
                div {
                    NavBar {}
                    Dashboard {}
                }
            ),
            _ => rsx!("blah"),
        })
    };

    static NavBar: Component = |cx| {
        println!("running navbar");
        cx.render(rsx! {
            h1 {
                "NavBar"
                {(0..3).map(|f| rsx!(NavLink {}))}
            }
        })
    };

    static NavLink: Component = |cx| {
        println!("running navlink");
        cx.render(rsx! {
            h1 {
                "NavLink"
            }
        })
    };

    static Dashboard: Component = |cx| {
        println!("running dashboard");
        cx.render(rsx! {
            div {
                "dashboard"
            }
        })
    };

    static Results: Component = |cx| {
        println!("running results");
        cx.render(rsx! {
            div {
                "results"
            }
        })
    };

    let mut dom = VirtualDom::new(App);
    let edits = dom.rebuild();
    dbg!(&edits);

    let edits = dom.work_with_deadline(|| false);
    dbg!(&edits);
    let edits = dom.work_with_deadline(|| false);
    dbg!(&edits);
    let edits = dom.work_with_deadline(|| false);
    dbg!(&edits);
    let edits = dom.work_with_deadline(|| false);
    dbg!(&edits);
}
