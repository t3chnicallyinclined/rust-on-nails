use crate::layout::Layout;
use db::User;
use dioxus::prelude::*;

struct Props {
    users: Vec<User>,
}

// Take a Vec<User> and create an HTML table.
pub fn users(users: Vec<User>) -> String {
    // Inner function to create our rsx! component
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {    // <-- Use our layout
                title: "Users Table",
                table {
                    thead {
                        tr {
                            th { "ID" }
                            th { "Email" }
                        }
                    }
                    tbody {
                        cx.props.users.iter().map(|user| rsx!(
                            tr {
                                td {
                                    strong {
                                        "{user.id}"
                                    }
                                }
                                td {
                                    "{user.email}"
                                }
                            }
                        ))
                    }
                }

                // 👇 this is our new form
                form {
                    action: "/sign_up",
                    method: "POST",
                    label { r#for: "user_email", "Email:" }
                    input { id: "user_email", name: "email", r#type: "email", required: "true" }
                    button { "Submit" }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(app, Props { users });
    let _ = app.rebuild();
    format!("<!DOCTYPE html><html lang='en'>{}</html>", dioxus_ssr::render(&app))
}