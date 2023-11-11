mod scorer;

use yew::{html, Component, Context, Html, };

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
    Rack,
    Level1,
    Level2
}

pub struct GameAction {
    entity: Plyr,
    action: Msg
}

pub struct Rack {
    player1: u16,
    player2: u16,
    dead: u16,
    innings: u16,
}

pub struct App {
    rack: usize,
    totals: Rack,
    player1lvl: u8,
    player2lvl: u8,
    racks: Vec<Rack>
}

const MAX_RACKS: usize = 50;

impl Component for App {
    type Message = GameAction;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut racks = Vec::with_capacity(MAX_RACKS);
        for _i in 0..MAX_RACKS {
            racks.push(Rack{
                player1: 0,
                player2: 0,
                dead: 0,
                innings: 0,
            });
        }
        Self {
            rack: 0,
            racks,
            player1lvl: 2,
            player2lvl: 0,
            totals: Rack {
            player1: 0, player2: 0, dead: 0, innings: 0, }
        }


    }

    fn update(&mut self, _ctx: &Context<Self>, action: Self::Message) -> bool {
        let racks = &mut self.racks;
        let current_rack = &mut racks[self.rack];
        match action {
            GameAction{entity: Plyr::P1, action: Msg::Increment} => {
                if current_rack.player1 + current_rack.player2 + current_rack.dead < 10 {
                    current_rack.player1 += 1;
                    self.totals.player1 += 1;
                }
                return true;
            }
            GameAction{entity: Plyr::P1, action: Msg::Decrement} => {
                if current_rack.player1 > 0 {
                    current_rack.player1 -= 1;
                    self.totals.player1 -= 1;
                }
                return true;
            }
            GameAction{entity: Plyr::P2, action: Msg::Increment} => {
                if current_rack.player1 + current_rack.player2 + current_rack.dead < 10 {
                    current_rack.player2 += 1;
                    self.totals.player2 += 1;
                }
                return true;
            }
            GameAction{entity: Plyr::P2, action: Msg::Decrement} => {
                if current_rack.player2 > 0 {
                    current_rack.player2 -= 1;
                    self.totals.player2 -= 1;
                }
                return true;
            }
            GameAction{entity: Plyr::Dead, action: Msg::Increment} => {
                if current_rack.player1 + current_rack.player2 + current_rack.dead < 10 {
                    current_rack.dead += 1;
                    self.totals.dead += 1;
                }
                return true;
            }
            GameAction{entity: Plyr::Dead, action: Msg::Decrement} => {
                if current_rack.dead > 0 {
                    current_rack.dead -= 1;
                    self.totals.dead -= 1;
                }
                return true;
            }
            GameAction{entity: Plyr::Inning, action: Msg::Increment} => {
                current_rack.innings += 1;
                self.totals.innings += 1;
                return true;
            }
            GameAction{entity: Plyr::Inning, action: Msg::Decrement} => {
                if current_rack.innings > 0 {
                    current_rack.innings -= 1;
                    self.totals.innings -= 1;
                }
                return true;
            }
            GameAction{entity: Plyr::Rack, action: Msg::Increment} => {
                if current_rack.player1 + current_rack.player2 + current_rack.dead < 10 {
                    current_rack.dead = 10 - current_rack.player1 - current_rack.player2;
                    self.totals.dead += current_rack.dead;
                }
                if self.rack < MAX_RACKS-1 {
                    self.rack += 1;
                }
                return true;
            }
            GameAction{entity: Plyr::Rack, action: Msg::Decrement} => {
                if self.rack > 0 {
                    self.rack -= 1;
                }
                return true;
            }
            GameAction{entity: Plyr::Level1, .. } => {
                let tmp = self.player1lvl + 1;
                if tmp > 8 {
                    self.player1lvl = 0;
                } else {
                    self.player1lvl = tmp;
                }
                return true;
            }
            GameAction{entity: Plyr::Level2, .. } => {
                let tmp = self.player2lvl + 1;
                if tmp > 8 {
                    self.player2lvl = 0;
                } else {
                    self.player2lvl = tmp;
                }
                return true;
            }
            _ => { false }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="score_panel">
                    <table style="width:100%">
                        <tr>
                            <th/>
                            <th onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Level1, action: Msg::Increment})}>
                                {format!("Player 1 SL:{0}", self.player1lvl + 1)}
                            </th>
                            <th>{"Innings"}</th>
                            <th>{"Dead"}</th>
                            <th onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Level2, action: Msg::Increment})}>
                                {format!("Player 2 SL:{0}", self.player2lvl + 1)}
                            </th>
                        </tr>
                        <tr>
                            <td>{"Totals"}</td>
                            <td>{scorer::calc_score(self.totals.player1, self.player1lvl)}</td>
                            <td>{self.totals.innings}</td>
                            <td>{self.totals.dead}</td>
                            <td>{scorer::calc_score(self.totals.player2, self.player2lvl)}</td>
                        </tr>
                        <tr>
                          <td><button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Rack, action: Msg::Increment})}>{ "+ R" }</button></td>
                          <td><button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P1, action: Msg::Increment})}>{ "+ P1" }</button></td>
                          <td><button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Inning, action: Msg::Increment})}>{ "+ I" }</button></td>
                          <td><button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Dead, action: Msg::Increment})}>{ "+ D" }</button></td>
                          <td><button class="button" onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P2, action: Msg::Increment})}>{ "+ P2" }</button></td>
                        </tr>
                        <tr>
                            <td>{format!("Rack {0}", self.rack+1)}</td>
                            <td>{self.racks[self.rack].player1}</td>
                            <td>{self.racks[self.rack].innings}</td>
                            <td>{self.racks[self.rack].dead}</td>
                            <td>{self.racks[self.rack].player2}</td>
                        </tr>
                        <tr>
                          <td><button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Rack, action: Msg::Decrement})}>{ "- R" }</button></td>
                          <td><button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P1, action: Msg::Decrement})}>{ "- P1" }</button></td>
                          <td><button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Inning, action: Msg::Decrement})}>{ "- I" }</button></td>
                          <td><button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::Dead, action: Msg::Decrement})}>{ "- D" }</button></td>
                          <td><button onclick={ctx.link().callback(|_| GameAction{entity: Plyr::P2, action: Msg::Decrement})}>{ "- P2" }</button></td>
                        </tr>
                    </table>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}