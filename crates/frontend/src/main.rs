use gloo::*;
use yew::prelude::*;

fn main() {
    if cfg!(target_family = "wasm") {
        utils::document().set_title("暴虐仙女的个人小站");
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
                <div class="container mx-auto max-w-full rounded bg-white py-3 shadow-md shadow-gray-200">
                    <div class="mx-96 rounded bg-gray-100 py-5">
                        <form></form>
                    </div>
                    <div class="mx-96 rounded bg-gray-950 py-5">
                        <form></form>
                    </div>
                </div>
            </>
        }
    }
}
