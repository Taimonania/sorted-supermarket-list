use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    let items = vec![
        Item {
            name: "Apple".to_string(),
            amount: "5".to_string(),
        },
        Item {
            name: "Banana".to_string(),
            amount: "10".to_string(),
        },
        Item {
            name: "Orange".to_string(),
            amount: "15".to_string(),
        },
    ];

    rsx! {
       div {
        class: "flex min-h-screen bg-white justify-center",
        div {
            class: "flex p-8 gap-4 flex-col w-1/2",
            p { class: "text-xl font-bold pb-2", "Einkaufsliste" }
            for item in items {
                div {
                    class: "flex space-x-4",
                    input { class: "text-lg w-1/6 rounded-md border-0 ring-gray-300", r#type: "text", value: item.amount }
                    span { class: "text-lg grow", {item.name} }
                }
            }
            }   
        }
    }
}

struct Item {
    name: String,
    amount: String,
}