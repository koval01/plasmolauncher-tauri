use std::{collections::HashMap, borrow::Borrow};

use serde::{Serialize, Deserialize};

use crate::{external_api::mojang::models::general::string_or_string_vec::StringOrStringVec, task::tasks::launch_game::LaunchGameError};

use super::rules::RulesItem;

use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
    game: Vec<Argument>,
    jvm: Vec<Argument>,
}

impl Arguments {
    fn filter_rules_satisfied(vec: Vec<Argument>) -> Vec<Argument> {
        vec.into_iter().filter(|argument| {
            match argument {
                Argument::String(_) => true,
                Argument::ArgumentObject(object) => object.check_if_rules_satisfied(),
            }
        })
            .collect()
    }

    pub fn get_game_rules_satisfied(&self) -> Vec<Argument> {
        Self::filter_rules_satisfied(self.game.clone())
    }

    pub fn get_jvm_rules_satisfied(&self) -> Vec<Argument> {
        Self::filter_rules_satisfied(self.jvm.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Argument {
    String(String),
    ArgumentObject(ArgumentObject),
}

impl Argument {
    pub fn to_string_or_string_vec(self) -> StringOrStringVec {
        match self {
            Argument::String(string) => StringOrStringVec::String(string),
            Argument::ArgumentObject(object) => object.value
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArgumentObject {
    rules: Vec<RulesItem>,
    pub value: StringOrStringVec,
}

impl ArgumentObject {
    pub fn check_if_rules_satisfied(&self) -> bool {
        self.rules.iter().all(|rule| rule.check_if_satisfied())
    }
}

pub trait ParseArguments {
    fn parse_arguments(self, variables: HashMap<String, String>) -> Result<Vec<String>>;
}

fn check_if_varibale(string: impl Borrow<String>) -> bool {
    string.borrow().starts_with("${") && string.borrow().ends_with("}")
}

fn get_variable_key(string: String) -> String {
    string.replace("${", "").replace("}", "")
}

impl ParseArguments for Vec<Argument> {
    fn parse_arguments(self, variables: HashMap<String, String>) -> Result<Vec<String>> {
        let mut new_vec = Vec::new();

        let mut iter = self.into_iter()
            .flat_map(|argument| {
                match argument.to_string_or_string_vec() {
                    StringOrStringVec::String(string) => Vec::from([string]),
                    StringOrStringVec::StringVec(vec) => vec,
                }
            });
        
        while let Some(string) = iter.next_back() {
            if !check_if_varibale(&string) {
                new_vec.push(string);
                continue;
            }

            let Some(tag) = iter.next_back() else {
                Err(LaunchGameError::ArgumentParse("Varibale doesn't have a tag".into()))?
            };

            let Some(value) = variables.get(&get_variable_key(string)) else { continue; };
    
            new_vec.push(value.to_owned());
            new_vec.push(tag);
        }
        new_vec.reverse();
        Ok(new_vec)
    } 
}