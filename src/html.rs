use sycamore::prelude::{Html, view, View};

use crate::cfg::Link;

pub fn index_page<G: Html>(links: Vec<Link>) -> View<G> {
    let menu_items = create_menu(links);

    view! {
        html {
            head {
                (header_fragment())
            }
            body {
                nav {
                    ul(role="menubar") { (menu_items) }
                }

                div(id="background") {
                    h1(id="banner", role="banner", class="no-select") { "ekezet.com" }
                }
            }
        }
    }
}

fn header_fragment<G: Html>() -> View<G> {
    let css = include_str!("../target/style.css");

    view! {
        meta(charset="utf-8")
        title { "ekezet.com" }
        meta(name="viewport", content="width=device-width, initial-scale=1")
        style(dangerously_set_inner_html=css.as_ref())
        (bootstrap_fragment())
    }
}

fn bootstrap_fragment<G: Html>() -> View<G> {
    let bootstrap = format!("\n{}", include_str!("bootstrap.js"));

    view! {
        script(type="module", dangerously_set_inner_html=bootstrap.as_ref())
    }
}

fn create_menu<G: Html>(links: Vec<Link>) -> View<G> {
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

                view! {
                    li(style=opacity, role="menuitem") {
                        a(href=link.uri, title=title) { (link.text) }
                    }
                }
            })
            .collect(),
    )
}
