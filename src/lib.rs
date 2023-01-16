/*
INLINE.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub name: String,
    pub attributes: HashMap<String, String>
}

impl Style {
    pub fn new(name: String, attributes: HashMap<String, String>) -> Style {
        return Style {
            name: name,
            attributes: attributes
        }
    }
    pub fn to_css(&self) -> String {
        let mut result_list: Vec<String> = Vec::new();
        for (key, value) in &self.attributes {
            let attribute: String = format!("{}:{}", key, value);
            result_list.push(attribute);
        }
        let result: String = result_list.join(";");
        return result;
    }
}

pub fn test(){
    let paragraph_style: Style = Style::new(
        String::from("p"),
        HashMap::from(
            [
                ("text-align".to_string(), "center".to_string()),
                ("font-size".to_string(), "25px".to_string()),
                ("padding".to_string(), "0px".to_string()),
                ("margin".to_string(), "0px".to_string())
            ]
        )
    );
    println!("{}", paragraph_style.to_css());
}