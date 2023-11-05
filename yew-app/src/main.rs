use yew::{html, Component, Context, Html};
use std::cmp;

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub enum Plyr {
    Table,
    P1,
    P2,
    Dead,
    Inning,
    Rack
}

pub struct GameAction {
    entity: Plyr,
    action: Msg
}

pub struct Rack {
    player1: i64,
    player2: i64,
    dead: i64,
    innings: i64,
}

pub struct App {
    rack: usize,
    totals: Rack,
    racks: Vec<Rack>
}

impl Component for App {
    type Message = GameAction;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut racks = Vec::with_capacity(300);
        for i in 0..300 {
            racks.push(Rack{
                player1: 0,
                player2: 0,
                dead: 0,
                innings: 0,
            });
        }
        Self { rack: 0, racks, totals: Rack {
            player1: 0,
            player2: 0,
            dead: 0,
            innings: 0,
        } }


    }

    fn update(&mut self, _ctx: &Context<Self>, action: Self::Message) -> bool {
        let current_rack = &mut self.racks[self.rack];
        match action {
            GameAction{entity: Plyr::P1, action: Msg::Increment} => {
                current_rack.player1 += 1;
                return true;
            }
            GameAction{entity: Plyr::P1, action: Msg::Decrement} => {
                current_rack.player1 -= 1;
                return true;
            }
            GameAction{entity: Plyr::P2, action: Msg::Increment} => {
                current_rack.player2 += 1;
                return true;
            }
            GameAction{entity: Plyr::P2, action: Msg::Decrement} => {
                current_rack.player2 -= 1;
                return true;
            }
            GameAction{entity: Plyr::Dead, action: Msg::Increment} => {
                current_rack.dead += 1;
                return true;
            }
            GameAction{entity: Plyr::Dead, action: Msg::Decrement} => {
                current_rack.dead -= 1;
                return true;
            }
            GameAction{entity: Plyr::Inning, action: Msg::Increment} => {
                current_rack.innings += 1;
                return true;
            }
            GameAction{entity: Plyr::Inning, action: Msg::Decrement} => {
                current_rack.innings -= 1;
                return true;
            }
            GameAction{entity: Plyr::Rack, action: Msg::Increment} => {
                self.rack += 1;
                self.totals = update_totals(&self.racks);
                return true;
            }
            GameAction{entity: Plyr::Rack, action: Msg::Decrement} => {
                self.rack = cmp::max(0, self.rack -1);
                self.totals = update_totals(&self.racks);
                return true;
            }
            _ => { false }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="button_panel">
                    // A button to send the Increment message
                    <div>
                        <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P1, action: Msg::Increment})}>
                            { "+1 to P1" }
                        </button>

                        // A button to send the Decrement message
                        <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P1, action: Msg::Decrement})}>
                            { "-1 to P1" }
                        </button>
                    </div>

                    // A button to send the Increment message
                    <div>
                        <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P2, action: Msg::Increment})}>
                            { "+1 to P2" }
                        </button>
                        <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P2, action: Msg::Decrement})}>
                            { "-1 to P2" }
                        </button>
                    </div>

                    // A button to send the Increment message
                    <div>
                        <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Dead, action: Msg::Increment})}>
                            { "+1 to Dead" }
                        </button>
                        <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Dead, action: Msg::Decrement})}>
                            { "-1 to Dead" }
                        </button>
                    </div>

                    // A button to send the Increment message
                    <div>
                        <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Inning, action: Msg::Increment})}>
                            { "+1 to Inning" }
                        </button>
                        <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Inning, action: Msg::Decrement})}>
                            { "-1 to Inning" }
                        </button>
                    </div>

                    // A button to send the Increment message
                    <div>
                        <button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Rack, action: Msg::Increment})}>
                            { "+1 to Rack" }
                        </button>
                        <button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Rack, action: Msg::Decrement})}>
                            { "-1 to Rack" }
                        </button>
                    </div>

                </div>
                <div class="score_panel">
                    <table style="width:100%">
                        <tr>
                            <th/>
                            <th>{"Player 1"}</th>
                            <th>{"Innings"}</th>
                            <th>{"Dead"}</th>
                            <th>{"Player 2"}</th>
                        </tr>
                        <tr>
                            <td>{"Totals"}</td>
                            <td>{self.totals.player1}</td>
                            <td>{self.totals.innings}</td>
                            <td>{self.totals.dead}</td>
                            <td>{self.totals.player2}</td>
                        </tr>
                        <tr>
                            <td>{format!("Rack {0}", self.rack)}</td>
                            <td>{self.racks[self.rack].player1}</td>
                            <td>{self.racks[self.rack].innings}</td>
                            <td>{self.racks[self.rack].dead}</td>
                            <td>{self.racks[self.rack].player2}</td>
                        </tr>
                    </table>
                </div>
            </div>
        }
    }
}

fn update_totals(racks: &Vec<Rack>) -> Rack {
    let mut player1: i64 = 0;
    let mut player2: i64 = 0;
    let mut innings: i64 = 0;
    let mut dead: i64 = 0;
    for i in 0..racks.len() {
        player1 += racks[i].player1;
        player2 += racks[i].player2;
        innings += racks[i].innings;
        dead += racks[i].dead;
    }
    return Rack{ player1, player2, innings, dead }
}


fn main() {
    yew::Renderer::<App>::new().render();
}