use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct Login {
    username_input: NodeRef,
}

pub enum Msg {
    SubmitLogin,
}

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    /// Callback to pass the logged-in username back up to the parent component (App)
    pub on_login: Callback<String>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = LoginProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username_input: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitLogin => {
                if let Some(input) = self.username_input.cast::<HtmlInputElement>() {
                    let username = input.value();
                    if !username.trim().is_empty() {
                        // Pass the username up to the parent component
                        ctx.props().on_login.emit(username);
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default(); // Prevent the page from refreshing
            Msg::SubmitLogin
        });

        html! {
            <div class="login-container" style="max-width: 400px; margin: 100px auto; padding: 20px; border: 1px solid #ccc; border-radius: 8px; text-align: center;">
                <h2>{"Welcome to YewChat"}</h2>
                <p>{"Enter a username to join the chat room"}</p>
                <form onsubmit={on_submit}>
                    <input 
                        ref={self.username_input.clone()} 
                        type="text" 
                        placeholder="Username..." 
                        style="width: 100%; padding: 10px; margin-bottom: 15px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
                    />
                    <button 
                        type="submit" 
                        style="width: 100%; padding: 10px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                    >
                        {"Join Chat Room"}
                    </button>
                </form>
            </div>
        }
    }
}