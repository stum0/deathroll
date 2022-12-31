use nanoid::nanoid;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

pub struct Home {
    new_game: bool,
    input: NodeRef,
    input_pve: NodeRef,
    pub start_roll: Option<u32>,
    pub start_roll_pve: Option<u32>,
}

pub enum Msg {
    ShowNewGame,
    HideNewGame,
    Input(String),
    DoNothing,
    NewPvpGameCustom,
    NewPvpGame(u32),
    NewPveGame(u32),
    NewPveGameCustom,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            new_game: false,
            input: NodeRef::default(),
            input_pve: NodeRef::default(),
            start_roll: None,
            start_roll_pve: None,
        }
    }
    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let roll_emoji = '\u{1F3B2}';
        let skull = '\u{1F480}';

        let input_ref_pvp = self.input.clone();
        let input_ref_pve = self.input_pve.clone();

        let home = ctx.link().callback(move |_: MouseEvent| Msg::HideNewGame);

        let pvp = ctx
            .link()
            .callback(move |_: MouseEvent| Msg::NewPvpGameCustom);

        let pve = ctx
            .link()
            .callback(move |_: MouseEvent| Msg::NewPveGameCustom);

        let new_game = ctx.link().callback(move |_: MouseEvent| Msg::ShowNewGame);

        let oninput_pvp = ctx.link().batch_callback(move |_| {
            let input = input_ref_pvp.cast::<HtmlInputElement>();

            input.map(|input| Msg::Input(input.value()))
        });

        let start_game_enter_pvp = ctx.link().callback(move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                Msg::NewPvpGameCustom
            } else {
                Msg::DoNothing
            }
        });

        let oninput_pve = ctx.link().batch_callback(move |_| {
            let input = input_ref_pve.cast::<HtmlInputElement>();

            input.map(|input| Msg::Input(input.value()))
        });

        let start_game_enter_pve = ctx.link().callback(move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                Msg::NewPveGameCustom
            } else {
                Msg::DoNothing
            }
        });

        html! {
        <div>
           <header>
           <button onclick={home}>{"deathroll.gg "}{skull}{roll_emoji}</button>

           <button onclick={new_game}> {"play" }</button>
           if self.new_game {
                <div class="new-game">
                <h3>{"PvP \u{2694}\u{FE0F}"}</h3>
                {"1v1"}
                <br/>
                <button onclick={pvp_roll(100, ctx)}>{ "100" }</button>
                <button onclick={pvp_roll(1000, ctx)}>{ "1000" }</button>
                <button onclick={pvp_roll(10000, ctx)}>{ "10000" }</button>
                <button onclick={pvp_roll(100000, ctx)}>{ "100000" }</button>
                <br/>
                <button onclick={pvp_roll(1000000, ctx)}>{ "1000000" }</button>
                <button onclick={pvp_roll(10000000, ctx)}>{ "10000000" }</button>
                <button onclick={pvp_roll(100000000, ctx)}>{ "100000000" }</button>
                <br/>
                    <input
                    ref ={self.input.clone()}
                    placeholder="custom roll"
                    oninput={oninput_pvp}
                    onkeypress={start_game_enter_pvp}
                    type="text" maxlength="9" min="1" max="100000000" inputmode="numeric" pattern="[0-9]*"
                    title="Non-negative integral number"

                    /> <button onclick={pvp}>{ "custom game" }</button>
                <h3>{"PvE \u{1F916}"}</h3>
                {"vs AI"}
                <br/>
                <button onclick={pve_roll(100, ctx)}>{ "100" }</button>
                <button onclick={pve_roll(1000, ctx)}>{ "1000" }</button>
                <button onclick={pve_roll(10000, ctx)}>{ "10000" }</button>
                <button onclick={pve_roll(100000, ctx)}>{ "100000" }</button>
                <br/>
                <button onclick={pve_roll(1000000, ctx)}>{ "1000000" }</button>
                <button onclick={pve_roll(10000000, ctx)}>{ "10000000" }</button>
                <button onclick={pve_roll(100000000, ctx)}>{ "100000000" }</button>
                <br/>
                    <input
                    ref ={self.input_pve.clone()}
                    placeholder="custom roll"
                    oninput={oninput_pve}
                    onkeypress={start_game_enter_pve}
                    type="text" maxlength="9" min="1" max="100000000" inputmode="numeric" pattern="[0-9]*"
                    title="Non-negative integral number"

                    /> <button onclick={pve}>{ "custom game" }</button>

                </div>
            } else {
            {""}
            }
            </header>
            <br/>
            <p>{"deathrolling is a game made famous by World of Warcraft, where players deathroll for gold."}</p>
            <p>{"Check out this video for an example of the game in action: "}<a href="https://youtu.be/vshLQqwfnjc?t=1044">{"https://youtu.be/vshLQqwfnjc?t=1044"}</a></p>

            <h3>{"Rules"}</h3>
            <ol>
              <li>{"Players take turns rolling a die."}</li>
              <li>{"The first player selects a number, and then rolls the die. The number they roll becomes the maximum number for the next player's roll."}</li>
              <li>{"If a player rolls a 1, they lose the game."}</li>
            </ol>
            {"The code for this website can be found here: "}<a href="https://github.com/stum0/deathroll">{"https://github.com/stum0/deathroll"}</a>

           <footer>
           </footer>
        </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowNewGame => {
                if self.new_game == false {
                    self.new_game = true
                } else if self.new_game == true {
                    self.new_game = false
                }
            }
            Msg::HideNewGame => self.new_game = false,
            Msg::Input(msg) => {
                let start_roll: u32 = match msg.trim().parse::<u32>() {
                    Ok(parsed_input) => parsed_input,

                    Err(_) => 1,
                };

                self.start_roll = Some(start_roll);
            }
            Msg::NewPvpGameCustom => {
                if self.start_roll != Some(1) {
                    let navigator = ctx.link().navigator().unwrap();

                    let id = nanoid!(8);

                    let roll = self.start_roll;
                    match roll {
                        Some(roll) => navigator.push(&Route::PvP { id: id, roll: roll }),
                        None => {}
                    }
                } else {
                    //log::debug!("ERROR");
                }
            }
            Msg::NewPvpGame(num) => {
                let navigator = ctx.link().navigator().unwrap();

                let id = nanoid!(8);

                navigator.push(&Route::PvP { id: id, roll: num })
            }
            Msg::NewPveGame(num) => {
                let navigator = ctx.link().navigator().unwrap();

                navigator.push(&Route::PvE { roll: num })
            }
            Msg::NewPveGameCustom => {
                if self.start_roll != Some(1) {
                    let navigator = ctx.link().navigator().unwrap();

                    let roll = self.start_roll;
                    match roll {
                        Some(roll) => navigator.push(&Route::PvE { roll: roll }),
                        None => {}
                    }
                } else {
                    //log::debug!("ERROR");
                }
            }
            Msg::DoNothing => {
                //log::debug!("Do nothing");
            }
        }
        true
    }
}

fn pvp_roll(num: u32, ctx: &yew::Context<Home>) -> Callback<MouseEvent> {
    ctx.link()
        .callback(move |_: MouseEvent| Msg::NewPvpGame(num))
}

fn pve_roll(num: u32, ctx: &yew::Context<Home>) -> Callback<MouseEvent> {
    ctx.link()
        .callback(move |_: MouseEvent| Msg::NewPveGame(num))
}
