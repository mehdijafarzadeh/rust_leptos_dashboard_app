use leptos::{create_signal, event_target_value, view, IntoView, Show, WriteSignal};

pub fn AddPersonModal(set_if_show_modal: WriteSignal<bool>) -> impl IntoView{
    const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white
        mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000
        ease-in-out";

    const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded
        text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

    const ADD_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded text-white
        transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7]
        px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

    const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7]
        px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

    let(person_name, set_person_name) = create_signal(String::new());
    let (person_title, set_person_title) = create_signal(String::new());
    let (person_level, set_person_level) = create_signal(String::new());
    let (compensation, set_compensation) = create_signal(String::new());
    
    let(error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);

    // to close the modal
    let on_close = move |_| {
        set_if_show_modal(false);
    };
    
    let on_click = move|_| {};

    view! {
        <div class="flex flex-col w-full h-full z-50 mx-auto items-center align-center">
            <div class={move || {
                if if_error() { ERROR_STYLE }
                else { NO_ERROR_STYLE }
            }}>
                <Show when=move || { if_error() }>
                    <p class="text-white bg-red-500 rounded w-full h-12 px-5 py-3
                        transition-all duration-750 ease-in-out">
                        { error_message() }
                    </p>
                </Show>
                <p class="text-white pt-5">"Add New Employee"</p>
                <input type="text" placeholder="Name"
                    class=INPUT_STYLE
                    value=person_name
                    on:input=move |event| {
                        set_person_name(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Title"
                    class=INPUT_STYLE
                    value=person_title
                    on:input=move |event| {
                        set_person_title(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Level"
                    class=INPUT_STYLE
                    value=person_level
                    on:input=move |event| {
                        set_person_level(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Compensation"
                    class=INPUT_STYLE
                    value=compensation
                    on:input=move |event| {
                        set_compensation(event_target_value(&event));
                    }
                />
                <div class="flex flex-row w-full items-right justify-right">
                    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button on:click=on_click class=ADD_BUTTON_STYLE>
                        "Add"
                    </button>
                </div>
            </div>
        </div>
    }
    
}