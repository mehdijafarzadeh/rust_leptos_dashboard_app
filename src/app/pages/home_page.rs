use leptos::{component, view, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
    <body
    class = "bg-gray-900 overflow-x-hide" >
        < div class = "w-full max-w-[64rem] mx-auto item-center justify-center align-center" >
        
           "Home Page is here!"
        < / div >
        < / body >
    }
}