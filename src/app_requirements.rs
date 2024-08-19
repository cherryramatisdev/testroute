use core::fmt;
use std::{str::FromStr, thread::sleep, time::Duration};

use inquire::{required, Select, Text};
use strum::VariantNames;
use strum_macros::{EnumString, VariantNames};

use crate::prompts;

#[derive(Debug, EnumString, VariantNames, Clone)]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl fmt::Display for HttpMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct ApplicationRequirements {
    pub path: String,
    pub http_method: HttpMethods,
    pub http_response_status: u16,
    pub http_response_path: Option<String>,
    pub delay: Option<usize>,
}

impl ApplicationRequirements {
    pub fn get_from_user() -> Self {
        let path = Text::new("What is the path of your route?")
            .with_validator(required!())
            .prompt()
            .unwrap();
        let http_method = Select::new(
            "What HTTP method should listen to?",
            HttpMethods::VARIANTS.to_vec(),
        )
        .prompt()
        .unwrap();
        let http_response_status = Select::new(
            "What should be the response HTTP status?",
            (100..599).map(|n| n.to_string()).collect(),
        )
        .prompt()
        .unwrap();
        let http_response =
            Text::new("What should be the response? (Write a file path or leave empty for none)")
                .with_autocomplete(prompts::file_completion::FilePathCompleter::default())
                .prompt()
                .unwrap();
        let delay = Text::new("There's any delay that you want on the route? (in seconds)")
            .prompt()
            .unwrap();

        Self {
            path,
            http_method: HttpMethods::from_str(http_method).unwrap(),
            http_response_status: http_response_status.parse::<u16>().unwrap(),
            http_response_path: if http_response.len() > 0 {
                Some(http_response)
            } else {
                None
            },
            delay: if delay.len() > 0 {
                Some(delay.parse::<usize>().unwrap())
            } else {
                None
            },
        }
    }

    pub fn try_sleep(&self) -> Option<()> {
        if let Some(delay) = self.delay {
            sleep(Duration::from_secs(delay.try_into().unwrap()));

            Some(())
        } else {
            None
        }
    }
}
