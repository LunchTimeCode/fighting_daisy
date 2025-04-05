use maud::{Markup, html};
use rocket::{Route, response::content};

use crate::all;

pub fn page(markup: Markup) -> Markup {
    html! {
       html  data-theme="dim" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content="Jobs";
                ({frontend::resources()})
                ({title("Handball")})
            }

            body {
                (markup)
        }
       }
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}

pub mod frontend {
    use maud::{Markup, PreEscaped, html};

    const DAISY: &str = r#"<link  href="/_assets/daisy.css" rel="stylesheet" type="text/css">"#;
    const TAIL: &str = r#"<script src="/_assets/tail.js"></script>"#;
    const DAISY_THEMES: &str =
        r#"<link  href="/_assets/themes.css" rel="stylesheet" type="text/css">"#;

    pub fn resources() -> Markup {
        html! {
        (PreEscaped(TAIL))
        (PreEscaped(DAISY))
        (PreEscaped(DAISY_THEMES))
        }
    }
}

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page(body_m()).into_string())
}

fn body_m() -> Markup {
    let navbar = fighting_daisy::components::navbar::Navbar::new("Showcase")
        .add_item(html! {
            button .(fighting_daisy::daisy::btn::primary()) { "Test" }
        })
        .render();

    html! {
    body {
    header {
        (navbar)
    }
    (all::btn())


        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body])
}
