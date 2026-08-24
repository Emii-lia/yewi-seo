use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[seo(
	meta(
		title = "Dioxus | Yewi",
		description = "Yewi-seo example on Dioxus",
		keywords("dioxus", "yewi", "yewi-seo", "seo"),
	),
	open_graph(
		title = "Dioxus | Yewi",
		description = "Yewi-seo example on Dioxus",
		url = "https://yewi.fiaro.app",
		site_name = "Yewi",
		locale = "en_US",
		image = "https://yewi.fiaro.app/images/og.png",
	),
	link(
		canonical = "https://yewi.fiaro.app",
	),
	twitter(
		card = "summary_large_image",
		site = "@Emii_lia",
		creator = "@Emii_lia",
		title = "Dioxus | Yewi",
		description = "Yewi-seo example on Dioxus",
		image = "https://yewi.fiaro.app/images/og.png",
	)
)]
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
