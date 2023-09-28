use yew::prelude::*;

fn main() {
    if cfg!(target_family = "wasm") {
        yew::Renderer::<HTMLBody>::new().render();
    } else if cfg!(not(target_family = "wasm")) {
        println!("This is a front-end application, please compile to 'wasm'!");
    }
}

#[function_component]
fn HTMLBody() -> Html {
    html! {<RootLayout/>}
}
#[derive(Properties, PartialEq)]
pub struct RootLayoutProperties;
pub struct RootLayout;
impl yew::Component for RootLayout {
    type Message = ();
    type Properties = RootLayoutProperties;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <p>{"Hello world!"}</p>
            </>
        }
    }
}
