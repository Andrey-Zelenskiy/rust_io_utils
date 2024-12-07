// Copyright Andrey Zelenskiy, 2024
use std::fs;
use std::ops::{
    Add,
    AddAssign,
};

/* -------------------------- */
/* Methods for summary output */
/* -------------------------- */

// Structure for storing the information about user-defined types in the string
// format
pub struct Summary {
    // Information about the parameters used to define the object
    pub parameters: Vec<Option<String>>,
    // Data that defines the current state of the object
    pub state: Vec<Option<String>>,
}

// Method to check that the numbers of parameters and state summary items are
// the same
impl Summary {
    fn check_size(&self) {
        if self.parameters.len() != self.state.len() {
            panic!("Number of summary items for parameters ({0}) and \
                   the states ({1}) are not the same.",
                   self.parameters.len(),
                   self.state.len(),
                   );
        }
    }
}

// Concatenation of summaries
impl Add for Summary {
    type Output = Summary;

    fn add(self, mut other: Summary) -> Summary {
        
        let mut parameters = self.parameters.clone();
        parameters.append(&mut other.parameters);

        let mut state = self.state.clone();
        state.append(&mut other.state);

        Summary {
            parameters,
            state,
        }
    }
}

impl AddAssign for Summary {
    fn add_assign(&mut self, other: Self) {

        self.parameters.append(&mut other.parameters.clone());
        self.state.append(&mut other.state.clone());
    }
}

// Trait for outputing the data stored in the user-defined types
pub trait Summarize {
    // Generate the Summary structure 
    fn get_summary(&self) -> Summary;

    // Print the parameters of the object
    fn print_parameters(&self) {
    
        let summary = self.get_summary();
        summary.check_size();

        println!();

        for parameters in summary.parameters.iter().flatten() {
            println!("{}", parameters);
        }
    }
    
    // Print the state of the object
    fn print_state(&self) {
    
        let summary = self.get_summary();
        summary.check_size();

        println!();

        for state in summary.state.iter().flatten() {
            println!("{}", state);
        }
    }
    
    // Print the full summary of the data
    fn print_summary(&self) {
    
        let summary = self.get_summary();
        summary.check_size();

        println!();

        for i in 0_usize..summary.parameters.len() {
            if let Some(parameters_summary) = &summary.parameters[i] {
                println!("{}", parameters_summary);
            }

            if let Some(state_summary) = &summary.state[i] {
                println!("{}", state_summary);
            }
        }
    }
    
    // Save the parameters of the object to text file
    fn save_parameters(&self, filename: &str) {
    
        let summary = self.get_summary();
        summary.check_size();

        for parameters in summary.parameters.iter().flatten() {
            fs::write(filename, parameters)
                .expect("Unable to write parameters to file {filename}.");
        }
    }
    
    // Save the state of the object to text file
    fn save_state(&self, filename: &str) {
    
        let summary = self.get_summary();
        summary.check_size();

        for state in summary.state.iter().flatten() {
            fs::write(filename, state)
                .expect("Unable to write the state to file {filename}.");
        }
    }
    
    // Save the full summary of the data to text file
    fn save_summary(&self, filename: &str) {
    
        let summary = self.get_summary();
        summary.check_size();

        for i in 0_usize..summary.parameters.len() {
            if let Some(parameters_summary) = &summary.parameters[i] {
                fs::write(filename, parameters_summary)
                    .expect("Unable to write the summary to file {filename}.");
            }

            if let Some(state_summary) = &summary.state[i] {
                fs::write(filename, state_summary)
                    .expect("Unable to write the summary to file {filename}.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_summaries() {
        let summary1 = Summary {
            parameters: vec![
                Some(format!("Parameters of the first structure.\n\n")),
            ],
            state: vec![
                Some(format!("State of the first structure.\n\n")),
            ],
        };
        
        let summary2 = Summary {
            parameters: vec![
                Some(format!("Parameters of the second structure.\n\n")),
            ],
            state: vec![
                Some(format!("State of the second structure.\n\n")),
            ],
        };

        let _ = summary1 + summary2;
    }
}
