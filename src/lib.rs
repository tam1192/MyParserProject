/// # MyParserProject
/// This repository will contain PARSER programs made for learning purposes.
///
/// # goal
/// It has the following goals
/// - [x] numerical expression
/// - [ ] HTTP parse
/// - [ ] BitMapPicture file parse
/// - [ ] Create new protocols
///
/// # My Rust Base Repositoly
/// It is implements the original rust development model that i advocate.
/// the contents are roughly as follows
/// - CI/CD  
/// Do CI/CD(Continuous Integration/Continuous Delivery) that adopt github actions.
/// - tests  
/// adopt test-driven development;
/// - original error  
/// Define your own error type that implements error trait.  
/// - Adopt general branching model
/// Adopt a general branching model during development
pub mod error;
pub mod number;
pub mod parser;
