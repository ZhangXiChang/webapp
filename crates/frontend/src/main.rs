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
        spawn_local(async move {
            let title = net::http::Request::get("http://127.0.0.1/webtitle")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            utils::document().set_title(title.as_str());
        });
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
            <div class="p-4">
                <div class="relative mx-auto flex max-w-full items-center rounded-md shadow-md">
                    <img class="absolute -left-3 h-24 w-24 rounded-full shadow-2xl"
                        src="http://q.qlogo.cn/headimg_dl?dst_uin=3066907854&spec=640&img_type=jpg"/>
                    <div class="flex flex-row space-x-1 py-5 pl-24">
                        <span class="text-2xl font-black text-violet-400">{utils::document().title()}</span>
                    </div>
                </div>
            </div>
            </>
        }
    }
}
