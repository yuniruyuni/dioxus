/*
Tiny CRM: A port of the Yew CRM example to Dioxus.
*/
use dioxus::{events::FormEvent, prelude::*};

fn main() {
    dioxus::desktop::launch(app);
}
enum Scene {
    ClientsList,
    NewClientForm,
    Settings,
}

#[derive(Clone, Debug, Default)]
pub struct Client {
    pub first_name: String,
    pub last_name: String,
    pub description: String,
}

fn app(cx: Scope) -> Element {
    let scene = use_state(&cx, || Scene::ClientsList);
    let clients = use_ref(&cx, || vec![] as Vec<Client>);
    let firstname = use_state(&cx, String::new);
    let lastname = use_state(&cx, String::new);
    let description = use_state(&cx, String::new);

    cx.render(rsx!(
        body {
            margin_left: "35%",
            link {
                rel: "stylesheet",
                href: "https://unpkg.com/purecss@2.0.6/build/pure-min.css",
                integrity: "sha384-Uu6IeWbM+gzNVXJcM9XV3SohHtmWE+3VGi496jvgX1jyvDTXfdK+rfZc8C1Aehk5",
                crossorigin: "anonymous",
            }

            h1 {"Dioxus CRM Example"}

            match *scene {
                Scene::ClientsList => rsx!(
                    div { class: "crm",
                        h2 { margin_bottom: "10px", "List of clients" }
                        div { class: "clients", margin_left: "10px",
                            clients.read().iter().map(|client| rsx!(
                                div { class: "client", style: "margin-bottom: 50px",
                                    p { "First Name: {client.first_name}" }
                                    p { "Last Name: {client.last_name}" }
                                    p {"Description: {client.description}"}
                                })
                            )
                        }
                        button { class: "pure-button pure-button-primary", onclick: move |_| scene.set(Scene::NewClientForm), "Add New" }
                        button { class: "pure-button", onclick: move |_| scene.set(Scene::Settings), "Settings" }
                    }
                ),
                Scene::NewClientForm => rsx!(
                    div { class: "crm",
                        h2 { margin_bottom: "10px", "Add new client" }
                        form { class: "pure-form",
                            input {
                                class: "new-client firstname",
                                placeholder: "First name",
                                value: "{firstname}",
                                oninput: move |e| firstname.set(e.value.clone())
                            }
                            input {
                                class: "new-client lastname",
                                placeholder: "Last name",
                                value: "{lastname}",
                                oninput: move |e| lastname.set(e.value.clone())
                            }
                            textarea {
                                class: "new-client description",
                                placeholder: "Description",
                                value: "{description}",
                                oninput: move |e| description.set(e.value.clone())
                            }
                        }
                        button {
                            class: "pure-button pure-button-primary",
                            onclick: move |_| {
                                clients.write().push(Client {
                                    description: (*description).clone(),
                                    first_name: (*firstname).clone(),
                                    last_name: (*lastname).clone(),
                                });
                                description.set(String::new());
                                firstname.set(String::new());
                                lastname.set(String::new());
                            },
                            "Add New"
                        }
                        button { class: "pure-button", onclick: move |_| scene.set(Scene::ClientsList),
                            "Go Back"
                        }
                    }
                ),
                Scene::Settings => rsx!(
                    div {
                        h2 { margin_bottom: "10px", "Settings" }
                        button {
                            background: "rgb(202, 60, 60)",
                            class: "pure-button pure-button-primary",
                            onclick: move |_| clients.write().clear(),
                            "Remove all clients"
                        }
                        button {
                            class: "pure-button pure-button-primary",
                            onclick: move |_| scene.set(Scene::ClientsList),
                            "Go Back"
                        }
                    }
                )
            }
        }
    ))
}
