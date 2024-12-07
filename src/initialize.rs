// Copyright Andrey Zelenskiy, 2024
use toml;

use crate::toml_parse::FromConfig;

/* ------------------------------------ */
/* Methods for structure initialization */
/* ------------------------------------ */

// Trait for argument structure with required initialization function
pub trait ArgsMethods: for <'a> FromConfig<'a> {
    type Target;

    // Initialize target structure from the parameters
    fn initialize_target(&self) -> Self::Target;
}

// Trait for initializing a structure from an argument structure
pub trait TargetFromArgs<T>
where 
    T: ArgsMethods<Target = Self>,
{
    // Initialize new Target from input parameters
    fn new(args: &T) -> Self where Self: Sized {
        args.initialize_target()
    }

    // Initialize Target from a config file
    fn from_config(
        config: &toml::Table, 
        config_name: &str,
        ) -> Self where Self: Sized {

        //Populate the parameters from the config
        let args = T::from_config(config, config_name);

        Self::new(&args)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use super::*;
    
    #[derive(Deserialize)]
    struct Args {
        x: u32,
        y: u32,
    }

    impl FromConfig<'_> for Args {}

    impl ArgsMethods for Args {
        type Target = TargetStruct;

        fn initialize_target(&self) -> Self::Target {
            Self::Target {
                x2: self.x * self.x,
                xy: self.x * self.y,
                y2: self.y * self.y,
            }
        }
    }

    struct TargetStruct {
        x2: u32,
        xy: u32,
        y2: u32,
    }

    impl TargetFromArgs<Args> for TargetStruct {}
    
    #[test]
    fn initialize_target() {
        let args = Args { x: 1, y: 2 };

        let target = TargetStruct::new(&args);

        println!(
            "x^2 = {0}, xy = {1}, y^2 = {2}.", 
            target.x2, 
            target.xy, 
            target.y2,
        );
    }
}
