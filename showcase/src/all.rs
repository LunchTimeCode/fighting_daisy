use fighting_daisy::daisy::*;
use maud::{Markup, html};

pub fn btn() -> Markup {
    let btn_primary = btn::primary();
    let btn_secondary = btn::secondary();
    let btn_error = btn::error();
    let btn_success = btn::success();
    let btn_warning = btn::warning();
    let btn_info = btn::info();
    let btn_ghost = btn::ghost();
    let btn_outline = btn::outline();
    let btn_soft = btn::soft();
    let btn_link = btn::link();
    let btn_active = btn::active();
    let btn_disabled = btn::disabled();
    let btn_square = btn::square();
    let btn_circle = btn::circle();
    let btn_block = btn::block();
    let btn_wide = btn::wide();
    let btn_sm = btn::sm();
    let btn_md = btn::md();
    let btn_lg = btn::lg();
    let btn_xl = btn::xl();
    let btn_xs = btn::xs();

    html! {
            div .grid .gap-2 .grid-cols-1 .p-2 {
                 h1 { "Button" }
                div .grid .gap-2 .grid-cols-4 {
                    fieldset .fieldset {
                        button .(btn_primary) { "Primary" }
                         button .(btn_secondary) { "Secondary" }
                         button .(btn_error) { "Error" }
                         button .(btn_success) { "Success" }
                         button .(btn_warning) { "Warning" }
                         button .(btn_info) { "Info" }
                         button .(btn_ghost) { "Ghost" }
                         button .(btn_outline) { "Outline" }
                         button .(btn_soft) { "Soft" }
                         button .(btn_link) { "Link" }
                         button .(btn_active) { "Active" }
                         button .(btn_disabled) { "Disabled" }
                         button .(btn_square) { "Square" }
                         button .(btn_circle) { "Circle" }
                         button .(btn_block) { "Block" }
                         button .(btn_wide) { "Wide" }
                         button .(btn_sm) { "Small" }
                         button .(btn_md) { "Medium" }
                         button .(btn_lg) { "Large" }
                         button .(btn_xl) { "Extra Large" }
                         button .(btn_xs) { "Extra Small" }
                    }

                }



            }

    }
}
