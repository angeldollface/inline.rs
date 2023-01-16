/*
INLINE.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// Importing "HashMap" from the
// standard library.
use std::collections::HashMap;

// We make a blueprint for a struct
// that has a name (The element's name.)
// and a "HashMap" (The element's rules.). 
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub name: String,
    pub attributes: HashMap<String, String>
}

impl Style {

    // Function to create a new "instance" of the struct.
    pub fn new(name: String, attributes: HashMap<String, &str>) -> Style {
        let mut hash_result: HashMap<String, String> = HashMap::new();
        for (key, value) in attributes {
            hash_result.insert(key, value.to_owned());
        }
        return Style {
            name: name,
            attributes: hash_result
        }
    }

    // Converts the data to a usable CSS string.
    pub fn to_css(&self) -> String {
        let mut result_list: Vec<String> = Vec::new();
        for (key, value) in &self.attributes {
            let attribute: String = format!(
                "{}:{};", 
                key, 
                value
            );
            result_list.push(attribute);
        }
        let result: String = result_list.join("");
        return result;
    }

    // String representation of the struct's data.
    pub fn to_string(&self) -> String {
        let mut result_list: Vec<String> = Vec::new();
        for (key, value) in &self.attributes {
            let attribute: String = format!(" {}:{};", key, value);
            result_list.push(attribute);
        }
        let result: String = format!(
            "{} {{\n{}\n}}", 
            &self.name, 
            result_list.join("\n")
        );
        return result;
    }
}

// Test method to see that everything works properly.
pub fn test(){

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