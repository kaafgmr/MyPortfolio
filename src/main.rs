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
        <div class = "content-row">
            <div class= "photoFrame">
                <img src= "images/eu2.png/" class= "photo"></img>
                <div class= "photoOverlay"></div>
            </div>

            <div class= "text-container">
                <div class= "about-me"></div>
                <div class= "blur-filter"></div>
                <div class= "about-text">
                    <h1>"Lorem ipsum dolor sit amet, qui minim labore."</h1>
                    <p>
                        "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis."
                    </p>
                </div>
            </div>
        </div>
    </div>
   }
}
