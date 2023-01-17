# INLINE.RS :nail_care: :ribbon:

***Inline CSS in Rust! :nail_care: :ribbon:***

![GitHub CI](https://github.com/angeldollface/inline.rs/actions/workflows/rust.yml/badge.svg)

## ABOUT

I was wondering about how to style a Yew application from Rust itself. I thought: What if we manufacture inline styles directly using a `struct` and a `HashMap`? **Inline.rs** was born. Why? I could've used **Stylist** but I want things to be tiny and easy. **Inline.rs** *is* tiny and easy.

## USAGE

### SETUP AND API

To use *Inline.rs* in your projects, add this to your `Cargo.toml`:

```TOML
[dependencies]
inline-rs = { git = "https://github.com/angeldollface/inline-rs",branch = "main" }
```

To import *Inline.rs*'s API, put this line of code inside your Rust code:

```Rust
use inline_rs::Style;
```

To refer to *Inline.rs*'s detailed API, please click [here](https://github.com/angeldollface/inline.rs/blob/main/src/lib.rs).

### EXAMPLES

To style something using *Inline.rs* you would write the following:

```Rust
/*
INLINE.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// Imports "Inline.rs".
use inline_rs::Style;

// Imports "HashMap".
use std::collections::HashMap;

fn main(){
    // Defining one variable for no padding and no margin.
    let my_fancy_padding: String = String::from("0px");

    // Standardizing our font size!
    let my_std_font_size: String = String::from("25px");

    // Sample style rules for a paragraph element.
    let paragraph_style: Style = Style::new(
        String::from("p"),
        HashMap::from(
            [
                ("text-align".to_string(), "center"),
                ("font-size".to_string(),  &my_std_font_size),
                ("padding".to_string(), &my_fancy_padding),
                ("margin".to_string(), &my_fancy_padding)
            ]
        )
    );

    // Printing the generated CSS.
    println!("{}", paragraph_style.to_css());

    // Printing the string representation.
    println!("{}", paragraph_style.to_string());
}
```

This will yield this for inline styles: 

```text
margin:0px;padding:0px;text-align:center;font-size:25px;
```

and this for a string representation: 

```CSS
p {
 margin:0px;
 padding:0px;
 text-align:center;
 font-size:25px;
}
```

To use *Inline.rs* with Yew, here's a small example:

```Rust
/*
INLINE.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// Importing Yew's APIs.
use yew::prelude::*;

// Importing Inline.rs.
use inline_rs::Style;

// Importing the "HashMap" implementation
// from the standard library.
use std::collections::HashMap;

// Main app component.
#[function_component]
pub fn App() -> Html {

    // Styles for a paragraph.
    let paragraph_style: Style = Style::new(
        String::from("p"),
        HashMap::from(
            [
                ("margin-right", "0px"),
                ("margin-left", "0px"),
                ("text-align", "left"),
                ("margin-top", "20px"),
                ("margin-bottom", "20px"),
                ("padding", "0px"),
                ("font-size", "25px"),
                ("color", "#00000"),
                ("font-family", "FiraCode-Regular")
            ]
        )
    );

    // Returning the paragraph element.
    return html!{
        <>
         <p style={paragraph_style.to_css()}>{"Whatever."}</p>
        </>
    };
}

// Main entry point for 
// the Rust compiler.
fn main() {

    // We render the app.
    yew::Renderer::<App>::new().render();
}
```

To view a more elaborate example, click [here](example/src/main.rs).

To try the more elaborate example on your own machine, follow these steps:

- 1.) Install `trunk` from [crates.io](https://crates.io/crates/trunk):

```bash
cargo install trunk
```

- 2.) Clone this project's source code:

```bash
git clone https://github.com/angeldollface/inline.rs.git
```

- 3.) Change directory into the source code's root directory:

```bash
cd inline.rs/example
```

- 4.) Serve the app locally (This will serve the app locally on [`http://127.0.0.1:8080/inline-rs/`](http://127.0.0.1:8080/inline-rs/).):

```bash
trunk --config trunk.toml serve --release
```

- 5.) If you want to build the app into a bundle to deploy to a server, run the command below. This will produce a directory called `dist` with the bundle inside it.

```bash
trunk --config trunk.toml build --release
```

- 5.) Enjoy! :heart_on_fire:

## CHANGELOG

### Version 1.0.0

- Initial release.
- Upload to GitHub.

### Version 1.1.0

- Added an example.
- Updated documentation.
- Changed paramater types for the `new` function.

## NOTE

- *Inline.rs :nail_care: :ribbon:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
