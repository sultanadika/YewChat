use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use web_sys::WebSocket;
use yew::prelude::*;

// FIXED: Defined properties so the component can accept incoming data parameters from its parent layout
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
    type Properties = ChatProps; // FIXED: Swapped from () to ChatProps

    fn create(ctx: &Context<Self>) -> Self {
        // Automatically connects straight out of the box to your designated target server port setup
        let ws = WebSocket::new("ws://127.0.0.1:8080").ok();
        
        let component = Self {
            ws,
            messages: vec![],
            text_input: NodeRef::default(),
        };

        // Configure standard browser WebSocket event listeners to pipe data seamlessly back into Yew's event loop
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
                // Incoming broadcast logs pushed down from the backend stream are appended straight into state
                self.messages.push(text);
                true // Re-render screen immediately to show the text block update
            }
            Msg::SendMessage => {
                if let Some(input) = self.text_input.cast::<HtmlInputElement>() {
                    let val = input.value();
                    if !val.trim().is_empty() {
                        if let Some(ref socket) = self.ws {
                            // FIXED: Extracted the parameter username string out of properties to attach to the string data payload
                            let user_identity = &ctx.props().username;
                            let formatted_message = format!("{}: {}", user_identity, val);
                            
                            // Fire the formatted string directly down the active socket pipeline
                            let _ = socket.send_with_str(&formatted_message);
                            input.set_value(""); // Clear out the layout entry element text field
                        }
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
    let on_click_send = ctx.link().callback(|_| Msg::SendMessage);

    // FIXED: Only dispatch a message to the Yew loop if the key is explicitly Enter
    let on_keydown = ctx.link().batch_callback(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            Some(Msg::SendMessage)
        } else {
            None // Tells Yew to completely ignore this key event and do nothing
        }
    });

    html! {
        <div class="chat-container" style="padding: 20px; font-family: sans-serif;">
            <h2>{"Sultan's WebChat Room"}</h2>
            
            <div class="message-log" style="border: 1px solid #ccc; height: 350px; overflow-y: scroll; padding: 15px; background-color: #fafafa; border-radius: 4px;">
                { for self.messages.iter().map(|msg| html! { <p style="margin: 6px 0; font-family: monospace; font-size: 14px; border-bottom: 1px dashed #eee; padding-bottom: 4px;">{ msg }</p> }) }
            </div>
            
            <div style="margin-top: 12px; display: flex; width: 100%; box-sizing: border-box;">
                <input 
                    ref={self.text_input.clone()} 
                    type="text" 
                    onkeydown={on_keydown} // Updated to use the clean keydown filter
                    placeholder="Type a message..." 
                    style="flex-grow: 1; padding: 10px; border: 1px solid #bbb; border-radius: 4px; font-size: 14px;"
                />
                <button 
                    onclick={on_click_send} 
                    style="width: 100px; padding: 10px; margin-left: 10px; background-color: #007bff; color: white; border: none; border-radius: 4px; font-weight: bold; cursor: pointer;"
                >
                    {"Send"}
                </button>
            </div>
        </div>
    }
}
}