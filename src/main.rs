use yew::prelude::*;

mod components;
use components::chat::Chat;
use components::login::Login;

struct App {
    username: Option<String>,
}

enum Msg {
    UserLoggedIn(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { username: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserLoggedIn(name) => {
                self.username = Some(name);
                true // Re-render application layout to shift into the chat window
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_login = ctx.link().callback(Msg::UserLoggedIn);

        html! {
            <main>
                {
                    match &self.username {
                        // If no username exists in state, show the Login screen gate
                        None => html! {
                            <Login {on_login} />
                        },
                        
                        // FIXED: Captured the valid string reference and passed it as a parameter component property
                        Some(name) => html! {
                            <Chat username={name.clone()} />
                        }
                    }
                }
            </main>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}