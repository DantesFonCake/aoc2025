use dioxus::prelude::*;
use reqwest;

const TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    let art = use_signal(|| 
" X X
X x X
 X X".to_string());
    rsx! {
        
        document::Stylesheet { href: TAILWIND }
        
        TaskInput {  
            ondataloaded: |data| None
        }
        //AsciiArt { art }

    }
}

#[component]
fn TaskInput(ondataloaded: Callback<Vec<u8>, Option<String>>) -> Element {
    let mut url_input = use_signal(|| "".to_string());
    let mut upload_inprogress = use_signal(|| false);
    let mut error_str = use_signal(|| Option::<String>::None);
    
    rsx! {
        div {
            class: "flex items-start",
            div {

                input {
                    r#type: "text",
                    name: "url",
                    value: url_input,
                    oninput: move |ev| url_input.set(ev.value()),
                    disabled: "{upload_inprogress}",
                },
                button {
                    r#type: "button",
                    onclick: move |_| async move {
                        upload_inprogress.set(true);
                        let response = match reqwest::get(&*url_input.read()).await  {
                            Ok(response) => response,
                            Err(err) => {
                                error!("Failed to request data: {err}");
                                error_str.set(Some(err.to_string()));
                                upload_inprogress.set(false);
                                return;
                            }
                        };
                        
                        let data = match response.bytes().await {
                            Ok(data) => data.to_vec(),
                            Err(err) => {
                                error!("Failed to load data: {err}");
                                error_str.set(Some(err.to_string()));
                                upload_inprogress.set(false);
                                return;
                            }
                        };
                        
                        let result = ondataloaded.call(data);
                        if let Some(err) = result {
                            error!("Failed to upload data: {err}");
                            error_str.set(Some(err));
                            upload_inprogress.set(false);
                        } else {
                            info!("Uploaded successfully");
                            upload_inprogress.set(false);
                        }
                    },
                    disabled: "{upload_inprogress}",
                    "Upload!"
                }
            },
            if let Some(err) = &*error_str.read(){
                span {  
                    class: "text-red-500",
                    "{err}"
                }
            }
        }
    }
}

#[component]
fn AsciiArt(art: ReadSignal<String>) -> Element {
    rsx! {
        div { 
            class: "flex items-center justify-center w-auto h-dvh rounded-xl p-4 m-4 bg-gray-300",
            pre { 
                class: "",
                "{art}" 
            }
        }
    }
}