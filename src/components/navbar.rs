use maud::{Markup, html};

use crate::daisy::*;

pub struct Navbar {
    pub title: String,
    pub items: Vec<Markup>,
}

impl Navbar {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Markup) -> &Self {
        self.items.push(item);
        self
    }

    pub fn render(&self) -> Markup {
        html! {
            div."navbar bg-base-100 shadow-sm" {
                div."flex-1" {
                    a .(btn::ghost()) ."text-xl" {
                        (self.title)
                    }
                }
                div."flex-none" {
                    ul."menu menu-horizontal px-1" {
                        @for item in &self.items {
                            li {
                                (item)
                            }
                        }
                    }
                }
            }
        }
    }
}
