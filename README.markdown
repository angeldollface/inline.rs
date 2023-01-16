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
inline-rs = { git = "https://github.com/angeldollface/inline-rs", version = "1.0.0" }
```

To import *Inline.rs*'s API, put this line of code inside your Rust code:

```Rust
use inline_rs::Style;
```

To refer to *Inline.rs*'s detailed API, please click [here](https://github.com/angeldollface/inline.rs/blob/main/src/lib.rs).

### EXAMPLE

To style something using *Inline.rs* you would write the following:

```Rust
use inline_rs::Style;

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

## CHANGELOG

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE

- *Inline.rs :nail_care: :ribbon:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.