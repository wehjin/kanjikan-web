use yew::prelude::*;

const BASE_URL: &'static str = "https://topaz-utopian-khaan.glitch.me";

struct StudyGroup {
	start: usize,
	end: usize,
}

impl StudyGroup {
	pub fn url(&self) -> String {
		format!("{}?start={}&end={}", BASE_URL, self.start, self.end)
	}
	pub fn label(&self) -> String {
		format!("Chapters {}-{}", self.start, self.end)
	}
}

#[function_component(App)]
fn app() -> Html {
	let groups = vec![
		StudyGroup { start: 1, end: 5 },
		StudyGroup { start: 5, end: 10 },
		StudyGroup { start: 11, end: 15 },
	];
	let groups_html = groups.into_iter().map(|group| {
		let (url, label) = (group.url(), group.label());
		html! {<li><a href={url}>{label}</a></li>}
	}).collect::<Html>();
	html! {
		<>
		<h1>{ "Kanjikan Web" }</h1>
		<ul>{groups_html}</ul>
		</>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
