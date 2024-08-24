use core::fmt;
use std::{str::FromStr, thread::sleep, time::Duration};

use inquire::{required, Select, Text};
use strum::VariantNames;
use strum_macros::{EnumString, VariantNames};

use crate::{prompts, Args};

#[derive(Debug, EnumString, VariantNames, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApplicationRequirements {
    pub path: String,
    pub http_method: HttpMethods,
    pub http_response_status: u16,
    pub http_response_body: Option<String>,
    pub http_response_path: Option<String>,
    pub delay: Option<usize>,
}

impl ApplicationRequirements {
    pub fn get_from_user(args: Args) -> Self {
        let path = args.path.unwrap_or_else(|| {
            Text::new("What is the path of your route?")
                .with_validator(required!())
                .prompt()
                .unwrap()
        });

        let http_method = args.method.unwrap_or_else(|| {
            HttpMethods::from_str(
                Select::new(
                    "What HTTP method should listen to?",
                    HttpMethods::VARIANTS.to_vec(),
                )
                .prompt()
                .unwrap(),
            )
            .unwrap()
        });

        let http_response_status = args.status.unwrap_or_else(|| {
            Select::new(
                "What should be the response HTTP status?",
                (100..599).map(|n| n.to_string()).collect(),
            )
            .prompt()
            .unwrap()
            .parse::<u16>()
            .unwrap()
        });

        let http_response = args.response.unwrap_or_else(|| {
            Text::new("What should be the response? (Write a file path or leave empty for none)")
                .with_autocomplete(prompts::file_completion::FilePathCompleter::default())
                .prompt()
                .unwrap()
        });

        let delay = args.delay.unwrap_or_else(|| {
            Text::new("There's any delay that you want on the route? (in seconds)")
                .prompt()
                .unwrap()
        });

        Self {
            path,
            http_method,
            http_response_status,
            http_response_body: None,
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
