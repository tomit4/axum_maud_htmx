use maud::{html, Markup, DOCTYPE};

pub fn index(name: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title {"Hello, World!"}
                link rel="stylesheet" href="/static/css/style.css" {}
                script defer src="static/js/index.js" {}
            }
            body {
                h1 {"Hello, World!"}
                p {"Hi, " (name) "!"}
            }
        }
    }
}
