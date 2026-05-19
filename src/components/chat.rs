use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use web_sys::WebSocket;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
    pub username: String,
}

pub struct Chat {
    ws: Option<WebSocket>,
    messages: Vec<String>,
    text_input: NodeRef,
}

pub enum Msg {
    MessageReceived(String),
    SendMessage,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ChatProps;

    fn create(ctx: &Context<Self>) -> Self {
        let ws = WebSocket::new("ws://127.0.0.1:8080").ok();
        
        let component = Self {
            ws,
            messages: vec![],
            text_input: NodeRef::default(),
        };

        if let Some(ref socket) = component.ws {
            let link = ctx.link().clone();
            let onmessage_callback = wasm_bindgen::prelude::Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                if let Some(text) = e.data().as_string() {
                    link.send_message(Msg::MessageReceived(text));
                }
            }) as Box<dyn FnMut(web_sys::MessageEvent)>);
            
            socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
        }

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MessageReceived(text) => {
                self.messages.push(text);
                true 
            }
            Msg::SendMessage => {
                if let Some(input) = self.text_input.cast::<HtmlInputElement>() {
                    let val = input.value();
                    if !val.trim().is_empty() {
                        if let Some(ref socket) = self.ws {
                            let user_identity = &ctx.props().username;
                            let formatted_message = format!("{}: {}", user_identity, val);
                            
                            let _ = socket.send_with_str(&formatted_message);
                            input.set_value(""); 
                        }
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click_send = ctx.link().callback(|_| Msg::SendMessage);

        let on_keydown = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Some(Msg::SendMessage)
            } else {
                None 
            }
        });

        html! {
            <div style="display: flex; justify-content: center; align-items: center; min-height: 80vh; background-color: #f0f2f5; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; padding: 20px;">
                
                
                <style>
                    {r#"
                        .chat-input:focus {
                            border-color: #007bff !important;
                        }
                        .chat-send-btn:hover {
                            background-color: #0056b3 !important;
                        }
                    "#}
                </style>

                // Main Chat Card Panel
                <div style="width: 100%; max-width: 600px; background: white; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1); display: flex; flex-direction: column; overflow: hidden;">
                    
                    // Chat Header Wrapper
                    <div style="background-color: #007bff; color: white; padding: 18px 20px; display: flex; justify-content: space-between; align-items: center;">
                        <h3 style="margin: 0; font-size: 18px; font-weight: 600;">{"Sultan's WebChat Room"}</h3>
                        <span style="font-size: 13px; background: rgba(255,255,255,0.2); padding: 4px 10px; border-radius: 20px;">
                            { format!("User: {}", ctx.props().username) }
                        </span>
                    </div>
                    
                    // Message Log Area
                    <div class="message-log" style="height: 400px; overflow-y: auto; padding: 20px; background-color: #f8f9fa; display: flex; flex-direction: column; gap: 10px;">
                        { 
                            for self.messages.iter().map(|msg| html! { 
                                <div style="background: white; padding: 10px 14px; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.05); border-left: 4px solid #007bff; word-break: break-word;">
                                    <p style="margin: 0; font-size: 14px; color: #333; line-height: 1.4;">{ msg }</p>
                                </div>
                            }) 
                        }
                    </div>
                    
                    // Input Action Bar
                    <div style="padding: 15px 20px; background: white; border-top: 1px solid #e9ecef; display: flex; width: 100%; box-sizing: border-box; align-items: center;">
                        <input 
                            ref={self.text_input.clone()} 
                            type="text" 
                            onkeydown={on_keydown}
                            placeholder="Type your message here..." 
                            class="chat-input" 
                            style="flex-grow: 1; padding: 12px 16px; border: 1px solid #ced4da; border-radius: 24px; font-size: 14px; outline: none; transition: border-color 0.2s;"
                        />
                        <button 
                            onclick={on_click_send} 
                            class="chat-send-btn" 
                            style="width: 90px; padding: 11px 0; margin-left: 12px; background-color: #007bff; color: white; border: none; border-radius: 24px; font-weight: bold; font-size: 14px; cursor: pointer; transition: background-color 0.2s;"
                        >
                            {"Send"}
                        </button>
                    </div>

                </div>
            </div>
        }
    }
}