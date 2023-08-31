use leptos::*;

fn main()
{
    leptos::mount_to_body(|cx| view! {cx, <App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView
{
   view!
   {cx,
    
    <div class= "content">
        <div class= "photoFrame">
            <img src= "images/eu2.png/" class= "photo"></img>
            <div class= "photoOverlay"></div>
        </div>

        <div class= "text-container">
            <div class= "about-me"></div>
            <div class= "about-text">
                <h1>"Hola soy Romero chino"</h1>
            </div>
        </div>
    </div>
   }
}
