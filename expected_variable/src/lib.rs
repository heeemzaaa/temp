
use convert_case::{Case, Casing};
use edit_distance::*;

pub fn expected_variables(source: &str, target:&str) -> Option<String> {
    if source.to_lowercase() == source.to_lowercase().to_case(Case::Camel) || source.to_lowercase() == source.to_lowercase().to_case(Case::Snake) {
        let steps = edit_distance(&source.to_lowercase(), &target.to_lowercase());
        let percentage = ((target.len() as f64 - steps as f64)*100)/target.len() as f64;

        if percentage > 50.0 {
            Some(format!("{}%", percentage.round()))
        } else {
            None
        }

    }
    None
}