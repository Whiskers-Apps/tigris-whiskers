use std::process::exit;

use icons::get_icon_path;
use sniffer_rs::sniffer::Sniffer;
use tigris_rs::features::{
    actions::{CopyTextAction, ResultAction},
    api::{get_extension_request, send_search_results},
    search::get_search_query,
    search_results::SearchResult,
};
use whiskers_palette_rs::{
    color::Color,
    palette::{get_panther_palette, get_tiger_palette},
};

pub mod icons;

fn main() {
    let request = get_extension_request().get_results_request.unwrap();
    let search_query = get_search_query(&request.search_text);

    let search_text = if search_query.keyword.is_some() {
        &search_query.search_text
    } else {
        &request.search_text
    };

    let sniffer = Sniffer::new();

    if search_text.is_empty() {
        send_search_results(&vec![]);
        exit(0)
    }

    let mut color_type = String::new();

    if let Some(keyword) = &search_query.keyword {
        color_type = match keyword.as_str() {
            "hsl" => String::from("hsl"),
            "rgb" => String::from("rgb"),
            _ => String::from("hex"),
        }
    }

    let mut panther_results = get_panther_palette()
        .colors()
        .iter()
        .filter(|color| {
            let full_name = format!("Panther {}", &color.name);
            sniffer.matches(&full_name, search_text)
        })
        .map(|color| get_color_result(color, "Panther", &color_type))
        .collect::<Vec<SearchResult>>();

    let mut tiger_results = get_tiger_palette()
        .colors()
        .iter()
        .filter(|color| {
            let full_name = format!("Tiger {}", &color.name);
            sniffer.matches(&full_name, search_text)
        })
        .map(|color| get_color_result(color, "Tiger", &color_type))
        .collect::<Vec<SearchResult>>();

    let mut results = Vec::<SearchResult>::new();
    results.append(&mut panther_results);
    results.append(&mut tiger_results);

    send_search_results(&results);
    exit(0)
}

fn get_color_result(color: &Color, main_palette: &str, color_type: &str) -> SearchResult {
    let color_value = match color_type {
        "hsl" => &color.hsl.hsl,
        "rgb" => &color.rgb.rgb,
        _ => &color.hex,
    };

    SearchResult::new(&format!("{main_palette} {}", color.name))
        .set_description(&color_value)
        .set_icon_path(&get_icon_path(&format!("{main_palette}{}", color.name)))
        .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
            &color_value,
        )))
}
