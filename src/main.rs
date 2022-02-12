use web_sys::{HtmlInputElement, MouseEvent};
use yew::{events::Event, html, Component, Context, Html, TargetCast};

mod chips;
use chips::Chips;

pub enum Msg {
    UpdateChipsStack(String),
    Increment,
    Decrement,
}

pub struct App {
    chips: u64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { chips: 1 }
    }

    /// Handle the `Msg` of the app
    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateChipsStack(amount) => {
                let n = match amount.parse::<u64>() {
                    Ok(n) => n,
                    _ => panic!("Failed to parse form value to int"),
                };
                self.chips = n;
                true
            }
            Msg::Increment => {
                self.chips += 1;
                true
            }
            Msg::Decrement => {
                self.chips -= 1;
                true
            }
        }
    }

    /// Render the app base, linking the input `onchange` to the `UpdateChipsStack` Message.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onchange = link.batch_callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::UpdateChipsStack(input.value()))
        });

        let increment = ctx.link().callback(|_: MouseEvent| Msg::Increment);
        let decrement = ctx.link().callback(|_: MouseEvent| Msg::Decrement);

        html! {
            <div>
                <div class="controls">
                    <label for="amount">{"Amount"}</label>
                    <input id="amount" {onchange} value={self.chips.to_string()} />
                    <button onclick={increment}>{"+"}</button>
                    <button onclick={decrement}>{"-"}</button>
                </div>
                <Chips amount={self.chips}/>
            </div>
        }
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<App>();
}
