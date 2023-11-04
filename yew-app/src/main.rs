use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub enum Plyr {
    Table,
    P1,
    P2,
    Dead
}

pub struct GameAction {
    entity: Plyr,
    action: Msg
}

pub struct App {
    table: i64, // This will store the counter value
    player1: i64, // This will store the counter value
    player2: i64, // This will store the counter value
    dead: i64, // This will store the counter value
}

impl Component for App {
    type Message = GameAction;
    // type Player = Plyr;
    // type PlayerAction = GameAction;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { table: 9, player1: 0, player2: 0, dead: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, action: Self::Message) -> bool {
        match action {
            GameAction{entity: Plyr::Table, action: Msg::Increment} => {
                self.table += 1;
                return true;
            }
            GameAction{entity: Plyr::Table, action: Msg::Decrement} => {
                self.table -= 1;
                return true;
            }
            GameAction{entity: Plyr::P1, action: Msg::Increment} => {
                self.player1 += 1;
                self.table -=1;
                return true;
            }
            GameAction{entity: Plyr::P1, action: Msg::Decrement} => {
                self.player1 -= 1;
                self.table += 1;
                return true;
            }
            GameAction{entity: Plyr::P2, action: Msg::Increment} => {
                self.player2 += 1;
                self.table -= 1;
                return true;
            }
            GameAction{entity: Plyr::P2, action: Msg::Decrement} => {
                self.player2 -= 1;
                self.table += 1;
                return true;
            }
            GameAction{entity: Plyr::Dead, action: Msg::Increment} => {
                self.dead += 1;
                self.table -= 1;
                return true;
            }
            GameAction{entity: Plyr::Dead, action: Msg::Decrement} => {
                self.dead -= 1;
                self.table += 1;
                return true;
            }
            _ => { false }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="panel">
                    // // A button to send the Increment message
                    // <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Table, action: Msg::Increment})}>
                    //     { "+1 to table" }
                    // </button>

                    // // A button to send the Decrement message
                    // <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Table, action: Msg::Decrement})}>
                    //     { "-1 to table" }
                    // </button>

                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P1, action: Msg::Increment})}>
                        { "+1 to P1" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P1, action: Msg::Decrement})}>
                        { "-1 to P1" }
                    </button>

                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P2, action: Msg::Increment})}>
                        { "+1 to P2" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P2, action: Msg::Decrement})}>
                        { "-1 to P2" }
                    </button>

                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Dead, action: Msg::Increment})}>
                        { "+1 to Dead" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Dead, action: Msg::Decrement})}>
                        { "-1 to Dead" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                   { self.table }
                </p>
                <p class="counter">
                { self.player1 }
             </p>
             <p class="counter">
             { self.player2 }
          </p>
          <p class="counter">
          { self.dead }
       </p>


            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}