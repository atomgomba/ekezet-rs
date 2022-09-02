use crate::cfg::Link;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct IndexProps {
    links: Vec<Link>,
}

#[component]
pub fn IndexPage<G: Html>(cx: Scope, props: IndexProps) -> View<G> {
    view! { cx,
        html {
            head {
                HeaderFragment {}
            }
            body {
                nav {
                    ul(role="menubar") { MainMenu(links=props.links) }
                }

                div(id="background") {
                    h1(id="banner", role="banner", class="no-select") { "ekezet.com" }
                }
            }
        }
    }
}

#[component]
fn HeaderFragment<G: Html>(cx: Scope) -> View<G> {
    let css = include_str!("../target/style.css");

    view! { cx,
        meta(charset="utf-8")
        title { "ekezet.com" }
        meta(name="viewport", content="width=device-width, initial-scale=1")
        style(dangerously_set_inner_html=css.as_ref())
        BootstrapFragment {}
    }
}

#[component]
fn BootstrapFragment<G: Html>(cx: Scope) -> View<G> {
    let bootstrap = format!("\n{}", include_str!("bootstrap.js"));

    view! { cx,
        script(type="module", dangerously_set_inner_html=bootstrap.as_ref())
    }
}

#[component]
fn MainMenu<G: Html>(cx: Scope, props: IndexProps) -> View<G> {
    let links = props.links;

    View::new_fragment(
        links
            .iter()
            .enumerate()
            .map(|(index, link)| {
                let link = link.clone();
                let title = link.title.unwrap_or_default();
                let opacity = format!(
                    "opacity: {}",
                    1.0 - ((index as f32) * (0.75 / links.len() as f32))
                );

                view! { cx,
                    li(style=opacity, role="menuitem") {
                        a(href=link.uri, title=title) { (link.text) }
                    }
                }
            })
            .collect(),
    )
}
