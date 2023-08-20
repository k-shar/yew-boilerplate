use yew::prelude::*;

pub enum Msg {
}

pub struct App {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p>{ "hello" }</p>
        }
    }
}

impl App {
}

fn main() {
    yew::Renderer::<App>::new().render();
}


