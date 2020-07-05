use cucumber::{cucumber, steps, before, after};
#[path = "../src/ray.rs"] mod ray;
use std::collections::HashMap;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    foo: String,
    envArray: HashMap<String, ray::ArrayVect>
}

impl MyWorld {

    fn addToEnv(&mut self, varName: String, varValue: ray::ArrayVect) {
        self.envArray.insert(varName, varValue);
    }

    fn readFromEnv(&self, varName: String) -> std::option::Option<&ray::ArrayVect> {
        self.envArray.get(&varName)
    }

}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld { 
            foo: "a default string".to_string(),
            envArray:  HashMap::new()
        }
    }
}

mod example_steps {
    use cucumber::steps;
    use super::*;
    
    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given "I am trying out Cucumber" |world, step| {
            world.foo = "Some string".to_string();
            // Set up your context in given steps
        };

        when "I consider what I am doing" |world, step| {
            // Take actions
            let new_string = format!("{}.", &world.foo);
            world.foo = new_string;
        };

        then "I am interested in ATDD" |world, step| {
            // Check that the outcomes to be observed have occurred
            assert_eq!(world.foo, "Some string.");
        };

        then regex r"^we can (.*) rules with regex$" |world, matches, step| {
            // And access them as an array
            assert_eq!(matches[1], "implement");
        };

        then regex r"^we can also match (\d+) (.+) types$" (usize, String) |world, num, word, step| {
            // `num` will be of type usize, `word` of type String
            assert_eq!(num, 42);
            assert_eq!(word, "olika");
        };

        then "we can use data tables to provide more parameters" |world, step| {
            let table = step.table().unwrap().clone();

            assert_eq!(table.header, vec!["key", "value"]);

            let expected_keys = table.rows.iter().map(|row| row[0].to_owned()).collect::<Vec<_>>();
            let expected_values = table.rows.iter().map(|row| row[1].to_owned()).collect::<Vec<_>>();

            assert_eq!(expected_keys, vec!["a", "b"]);
            assert_eq!(expected_values, vec!["fizz", "buzz"]);
        };

        given regex r"^(.+) <- array3 (\d+), (\d+), (\d+)" (String, i32, i32, i32) |world, variableName, variableXValue, variableYValue, variableZValue, step| {
            let a = super::ray::ArrayVect::array3(variableXValue, variableYValue, variableZValue);
            // Set up your context in given steps
            world.addToEnv(variableName, a);
        };

        when regex r"^(.+) <- concat (.+),(.+)" (String, String, String) |world, variableName, operand1, operand2, step| {
            let a = world.readFromEnv(operand1).unwrap();
            let b = world.readFromEnv(operand2).unwrap();
            let c = a.concatRef(b);
            world.addToEnv(variableName, c);
        };

        then regex r"^(.+) = array6 (\d+), (\d+), (\d+), (\d+), (\d+), (\d+)" (String, i32, i32, i32, i32, i32, i32) |world, variableName, variableX1Value, variableY1Value, variableZ1Value, variableX2Value, variableY2Value, variableZ2Value, step| {
            let a = super::ray::ArrayVect::array6(variableX1Value, variableY1Value, variableZ1Value, variableX2Value, variableY2Value, variableZ2Value);
            let r = world.readFromEnv(variableName).unwrap();
            assert_eq!(a.elts, r.elts);
        };

    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |scenario| {

});

// A setup function to be called before everything else
fn setup() {
    
}

cucumber! {
    features: "./features", // Path to our feature files
    world: crate::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        example_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ], 
    after: &[
        an_after_fn // Optional; called after each scenario
    ] 
}