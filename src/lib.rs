use isbn::{Isbn, Isbn10, Isbn13};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct IsbnInfo {
    valid: bool,
    isbn_type: Option<String>,
    raw: Option<String>,
    hyphenated: Option<String>,
    registration_group: Option<String>,
    isbn10: Option<String>,
    isbn13: Option<String>,
    error: Option<String>,
}

#[wasm_bindgen]
pub fn parse_isbn(input: &str) -> JsValue {
    let info = match input.parse::<Isbn>() {
        Ok(isbn) => {
            let (isbn_type, raw, isbn10, isbn13) = match &isbn {
                Isbn::_10(i10) => {
                    let i13: Isbn13 = (*i10).into();
                    (
                        "ISBN-10".to_string(),
                        i10.to_string(),
                        Some(i10.to_string()),
                        Some(i13.to_string()),
                    )
                }
                Isbn::_13(i13) => {
                    let i10 = Isbn10::try_from(*i13).ok().map(|i| i.to_string());
                    (
                        "ISBN-13".to_string(),
                        i13.to_string(),
                        i10,
                        Some(i13.to_string()),
                    )
                }
            };

            let hyphenated = isbn.hyphenate().ok().map(|h| h.to_string());
            let registration_group = isbn.registration_group().ok().map(|s| s.to_string());

            IsbnInfo {
                valid: true,
                isbn_type: Some(isbn_type),
                raw: Some(raw),
                hyphenated,
                registration_group,
                isbn10,
                isbn13,
                error: None,
            }
        }
        Err(e) => IsbnInfo {
            valid: false,
            isbn_type: None,
            raw: None,
            hyphenated: None,
            registration_group: None,
            isbn10: None,
            isbn13: None,
            error: Some(e.to_string()),
        },
    };

    serde_wasm_bindgen::to_value(&info).unwrap()
}
