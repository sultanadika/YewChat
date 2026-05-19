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
                        ctx.props().on_login.emit(username);
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default(); 
            Msg::SubmitLogin
        });

        html! {
            <div style="display: flex; justify-content: center; align-items: center; min-height: 80vh; background-color: #f0f2f5; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 20px;">
                
                
                <div style="width: 100%; max-width: 400px; background: white; padding: 30px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1); text-align: center;">
                    
                    <div style="margin-bottom: 24px;">
                        <h2 style="margin: 0 0 8px 0; color: #1a1a1a; font-size: 24px; font-weight: 700;">{"Welcome to YewChat"}</h2>
                        <p style="margin: 0; color: #6c757d; font-size: 14px;">{"Enter a username to join the chat workspace"}</p>
                    </div>

                    <form onsubmit={on_submit}>
                        <div style="margin-bottom: 20px; text-align: left;">
                            <label style="display: block; margin-bottom: 6px; font-size: 12px; font-weight: 600; color: #495057; text-transform: uppercase; letter-spacing: 0.5px;">
                                {"Username"}
                            </label>
                            <input 
                                ref={self.username_input.clone()} 
                                type="text" 
                                placeholder="e.g. Sultan" 
                                style="width: 100%; padding: 12px 16px; border: 1px solid #ced4da; border-radius: 8px; font-size: 14px; box-sizing: border-box; outline: none; transition: border-color 0.2s;"
                            />
                        </div>

                        <button 
                            type="submit" 
                            style="width: 100%; padding: 12px; background-color: #007bff; color: white; border: none; border-radius: 8px; font-size: 15px; font-weight: bold; cursor: pointer; transition: background-color 0.2s;"
                        >
                            {"Join Chat Room"}
                        </button>
                    </form>

                </div>
            </div>
        }
    }
}