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
                true 
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_login = ctx.link().callback(Msg::UserLoggedIn);

        html! {
            
            <div style="min-height: 100vh; background-color: #f0f2f5; margin: 0; padding: 0; width: 100%;">
                <main>
                    {
                        match &self.username {
                            None => html! {
                                <Login {on_login} />
                            },
                            Some(name) => html! {
                                <Chat username={name.clone()} />
                            }
                        }
                    }
                </main>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}