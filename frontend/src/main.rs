use gloo::*;
use wasm_bindgen_futures::*;
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
            <div class="p-4">
                <div class="relative mx-auto flex max-w-full items-center rounded-md shadow-md">
                    <img class="absolute -left-3 h-24 w-24 rounded-full shadow-2xl"
                        src="/api/get_app_logo"/>
                    <div class="flex flex-row space-x-1 py-5 pl-24">
                        <span id="title" class="text-2xl font-black text-violet-400"></span>
                    </div>
                </div>
            </div>
            </>
        }
    }

    fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
        if first_render {
            spawn_local(async move {
                let title = net::http::Request::get("/api/get_app_name")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                utils::document().set_title(title.as_str());
                utils::document()
                    .query_selector("#title")
                    .unwrap()
                    .unwrap()
                    .set_inner_html(title.as_str());
            });
        }
    }
}
