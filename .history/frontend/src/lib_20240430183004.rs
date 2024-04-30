use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    greeting: String,
}

enum Msg {
    UpdateGreeting(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            greeting: "Hello, NEAR!".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateGreeting(new_greeting) => {
                self.greeting = new_greeting;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ &self.greeting }</h1>
                <button onclick=self.link.callback(|_| Msg::UpdateGreeting("Hello, Yew!".into()))>
                    { "Change Greeting" }
                </button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}