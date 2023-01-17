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

    // Styles for the main heading.
    let div_style: Style = Style::new(
        String::from("div"),
        HashMap::from(
            [
                ("margin", "0 auto"),
                ("display", "block"),
                ("width", "80%"),
                ("margin-top", "20px !important"),
                ("margin-bottom", "20px !important"),
                ("padding", "0px"),
                ("border", "none")
            ]
        )
    );

    // Styles for the main heading.
    let heading_style: Style = Style::new(
        String::from("h1"),
        HashMap::from(
            [
                ("margin-right", "0px"),
                ("margin-left", "0px"),
                ("text-align", "center"),
                ("margin-top", "20px"),
                ("margin-bottom", "20px"),
                ("padding", "0px"),
                ("font-size", "35px"),
                ("color", "#00000"),
                ("font-family", "OrangeSlice")
            ]
        )
    );

    // Styles for a paragraph.
    let description_style: Style = Style::new(
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

    // Styles for a link.
    let link_style: Style = Style::new(
        String::from("a"),
        HashMap::from(
            [
                ("margin", "0px"),
                ("padding", "0px"),
                ("color", "#000"),
                ("font-size", "25px"),
                ("transition-duration", "0.6s"),
                ("text-decoration", "underline"),
                ("font-family", "FiraCode-Regular")
            ]
        )
    );

    // Styles for hovering over a link.
    let link_hover_style: Style = Style::new(
        String::from("a:hover"),
        HashMap::from(
            [
                ("color", "#808080")
            ]
        )
    );

    // Returning the main div.
    return html!{
        <div style={div_style.to_css()}>
         <h1 style={heading_style.to_css()}>
          { "INLINE.RS" }
         </h1>
         <p style={description_style.to_css()}>
          {
            "
            I was wondering about how to style a Yew application from Rust itself. 
            I thought: What if we manufacture inline styles directly using a \'struct\' and a \'HashMap\'? 
            \'Inline.rs\' was born. Why? I could\'ve used Stylist but I want things to be tiny and easy. 
            \'Inline.rs\' IS tiny and easy. Here\'s a link to
            "
          }
            <a href="https://github.com/angeldollface/inline.rs">
             { " getting started." }
            </a>
         </p>
        </div>
    };
}

// Main entry point for 
// the Rust compiler.
fn main() {

    // We render the app.
    yew::Renderer::<App>::new().render();
}