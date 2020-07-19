use cucumber::{after, before, cucumber, steps};
#[path = "../src/ray.rs"]
mod ray;
use std::collections::HashMap;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    foo: String,
    env_array: HashMap<String, ray::ArrayVect>,
    env_tuple: HashMap<String, ray::Tuple>,
}

impl MyWorld {
    fn add_to_env(&mut self, var_name: String, var_value: ray::ArrayVect) {
        self.env_array.insert(var_name, var_value);
    }

    fn read_from_env(&self, var_name: String) -> std::option::Option<&ray::ArrayVect> {
        self.env_array.get(&var_name)
    }

    fn add_to_env_tuple(&mut self, var_name: String, var_value: ray::Tuple) {
        self.env_tuple.insert(var_name, var_value);
    }

    fn read_from_env_tuple(&self, var_name: String) -> std::option::Option<&ray::Tuple> {
        self.env_tuple.get(&var_name)
    }
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            foo: "a default string".to_string(),
            env_array: HashMap::new(),
            env_tuple: HashMap::new(),
        }
    }
}

mod example_steps {
    use super::*;
    use cucumber::steps;

    fn float_value_from(sign: String, value: i32, dec: i32) -> f32 {
        format!("{}{}.{}", sign, value, dec).parse().unwrap()
    }

    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given "I am trying out Cucumber" |world, _step| {
            world.foo = "Some string".to_string();
            // Set up your context in given steps
        };

        when "I consider what I am doing" |world, _step| {
            // Take actions
            let new_string = format!("{}.", &world.foo);
            world.foo = new_string;
        };

        then "I am interested in ATDD" |world, _step| {
            // Check that the outcomes to be observed have occurred
            assert_eq!(world.foo, "Some string.");
        };

        then regex r"^we can (.*) rules with regex$" |_world, matches, _step| {
            // And access them as an array
            assert_eq!(matches[1], "implement");
        };

        then regex r"^we can also match (\d+) (.+) types$" (usize, String) |_world, num, word, _step| {
            // `num` will be of type usize, `word` of type String
            assert_eq!(num, 42);
            assert_eq!(word, "olika");
        };

        then "we can use data tables to provide more parameters" |_world, step| {
            let table = step.table().unwrap().clone();

            assert_eq!(table.header, vec!["key", "value"]);

            let expected_keys = table.rows.iter().map(|row| row[0].to_owned()).collect::<Vec<_>>();
            let expected_values = table.rows.iter().map(|row| row[1].to_owned()).collect::<Vec<_>>();

            assert_eq!(expected_keys, vec!["a", "b"]);
            assert_eq!(expected_values, vec!["fizz", "buzz"]);
        };

        given regex r"^(.+) <- array3 (\d+), (\d+), (\d+)" (String, i32, i32, i32) |world, variable_name, variable_x_value, variable_y_value, variable_z_value, _step| {
            let a = super::ray::ArrayVect::array3(variable_x_value, variable_y_value, variable_z_value);
            // Set up your context in given steps
            world.add_to_env(variable_name, a);
        };

        when regex r"^(.+) <- concat (.+),(.+)" (String, String, String) |world, variable_name, operand1, operand2, _step| {
            let a = world.read_from_env(operand1).unwrap();
            let b = world.read_from_env(operand2).unwrap();
            let c = a.concat_ref(b);
            world.add_to_env(variable_name, c);
        };

        then regex r"^(.+) = array6 (\d+), (\d+), (\d+), (\d+), (\d+), (\d+)" (String, i32, i32, i32, i32, i32, i32) |world, variable_name, variable_x1_value, variable_y1_value, variable_z1_value, variable_x2_value, variable_y2_value, variable_z2_value, _step| {
            let a = super::ray::ArrayVect::array6(variable_x1_value, variable_y1_value, variable_z1_value, variable_x2_value, variable_y2_value, variable_z2_value);
            let r = world.read_from_env(variable_name).unwrap();
            assert_eq!(a.elts, r.elts);
        };

        given regex r"^(.+) <- tuple (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, variable_w_sign, variable_w_value, variable_w_dec_value, _step| {
            let a = super::ray::Tuple(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value), float_value_from(variable_w_sign, variable_w_value, variable_w_dec_value));
            // Set up your context in given steps
            world.add_to_env_tuple(variable_name, a);
        };

        then regex r"^(a).x = (-?)(\d+).(\d+)" (String, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, _step| {
            let a = float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value);
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert_eq!(a, r.x());
        };

        then regex r"^(a).y = (-?)(\d+).(\d+)" (String, String, i32, i32) |world, variable_name, variable_y_sign, variable_y_value, variable_y_dec_value, _step| {
            let a = float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value);
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert_eq!(a, r.y());
        };

        then regex r"^(a).z = (-?)(\d+).(\d+)" (String, String, i32, i32) |world, variable_name, variable_z_sign, variable_z_value, variable_z_dec_value, _step| {
            let a = float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value);
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert_eq!(a, r.z());
        };

        then regex r"^(a).w = (-?)(\d+).(\d+)" (String, String, i32, i32) |world, variable_name, variable_w_sign, variable_w_value, variable_w_dec_value, _step| {
            let a = float_value_from(variable_w_sign, variable_w_value, variable_w_dec_value);
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert_eq!(a, r.w());
        };

        then regex r"(.+) is a point" (String) |world, variable_name, _step| {
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert!(r.is_point());
        };

        then regex r"(.+) is not a point" (String) |world, variable_name, _step| {
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert!(!r.is_point());
        };

        then regex r"(.+) is a vector" (String) |world, variable_name, _step| {
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert!(r.is_vector());
        };

        then regex r"(.+) is not a vector" (String) |world, variable_name, _step| {
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert!(!r.is_vector());
        };

        given regex r"^(.+) <- point (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, _step| {
            let a = super::ray::Tuple::point3(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value));
            // Set up your context in given steps
            world.add_to_env_tuple(variable_name, a);
        };

        then regex r"^(.+) = tuple (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, variable_w_sign, variable_w_value, variable_w_dec_value, _step| {
            let a = super::ray::Tuple(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value), float_value_from(variable_w_sign, variable_w_value, variable_w_dec_value));
            let r = world.read_from_env_tuple(variable_name).unwrap();
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, *r);
        };

        given regex r"^(.+) <- vector (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, _step| {
            let a = super::ray::Tuple::vector3(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value));
            // Set up your context in given steps
            world.add_to_env_tuple(variable_name, a);
        };

        then regex r"^([^\+]+) \+ ([^\+]+) == tuple (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_name2, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, variable_w_sign, variable_w_value, variable_w_dec_value, _step| {
            let a = super::ray::Tuple(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value), float_value_from(variable_w_sign, variable_w_value, variable_w_dec_value));
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r2 = world.read_from_env_tuple(variable_name2).unwrap();
            let r = r1.add(r2);
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, r);
        };

        then regex r"^([^\-]+) \- ([^\-]+) == vector (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_name2, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, _step| {
            let a = super::ray::Tuple::vector3(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value));
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r2 = world.read_from_env_tuple(variable_name2).unwrap();
            let r = r1.sub(r2);
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, r);
        };

        then regex r"^([^\-]+) \- ([^\-]+) == point (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_name2, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, _step| {
            let a = super::ray::Tuple::point3(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value));
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r2 = world.read_from_env_tuple(variable_name2).unwrap();
            let r = r1.sub(r2);
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, r);
        };

        then regex r"^neg (.+) == tuple (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, variable_w_sign, variable_w_value, variable_w_dec_value, _step| {
            let a = super::ray::Tuple(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value), float_value_from(variable_w_sign, variable_w_value, variable_w_dec_value));
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r = r1.neg();
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, r);
        };

        then regex r"^([^\*]+) \* (-?)(\d+).(\d+) == tuple (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, scalar_sign, scalar_value, scalar_dec, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, variable_w_sign, variable_w_value, variable_w_dec_value, _step| {
            let a = super::ray::Tuple(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value), float_value_from(variable_w_sign, variable_w_value, variable_w_dec_value));
            let scale = float_value_from(scalar_sign, scalar_value, scalar_dec);
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r = r1.scale(scale);
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, r);
        };

        then regex r"^(.+) / (-?)(\d+).(\d+) == tuple (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, scalar_sign, scalar_value, scalar_dec, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, variable_w_sign, variable_w_value, variable_w_dec_value, _step| {
            let a = super::ray::Tuple(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value), float_value_from(variable_w_sign, variable_w_value, variable_w_dec_value));
            let scale = float_value_from(scalar_sign, scalar_value, scalar_dec);
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r = r1.divide(scale);
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, r);
        };

        then regex r"^magnitude (.+) == (\d+).(\d+)" (String, i32, i32) |world, variable_name, variable_x_value, variable_x_dec_value, _step| {
            let a = float_value_from(String::from(""), variable_x_value, variable_x_dec_value);
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r = r1.magnitude();
            assert!(a == r || super::ray::eqv_float(a, r));
        };

        then regex r"^magnitude (.+) == sqrt (\d+).(\d+)" (String, i32, i32) |world, variable_name, variable_x_value, variable_x_dec_value, _step| {
            let a = float_value_from(String::from(""), variable_x_value, variable_x_dec_value);
            let sqrt = a.sqrt();
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r = r1.magnitude();
            assert_eq!(sqrt, r);
        };

        then regex r"^normalize (.+) == vector (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, _step| {
            let a = super::ray::Tuple::vector3(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value));
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r = r1.normalize();
            assert_eq!(a.x(), r.x());
            assert_eq!(a.y(), r.y());
            assert_eq!(a.z(), r.z());
            assert_eq!(a.w(), r.w());
            assert_eq!(a, r);
        };

        then regex r"^normalize (.+) == approximately vector (-?)(\d+).(\d+), (-?)(\d+).(\d+), (-?)(\d+).(\d+)" (String, String, i32, i32, String, i32, i32, String, i32, i32) |world, variable_name, variable_x_sign, variable_x_value, variable_x_dec_value, variable_y_sign, variable_y_value, variable_y_dec_value, variable_z_sign, variable_z_value, variable_z_dec_value, _step| {
            let a = super::ray::Tuple::vector3(float_value_from(variable_x_sign, variable_x_value, variable_x_dec_value), float_value_from(variable_y_sign, variable_y_value, variable_y_dec_value), float_value_from(variable_z_sign, variable_z_value, variable_z_dec_value));
            let r1 = world.read_from_env_tuple(variable_name).unwrap();
            let r = r1.normalize();
            let approx = r.approximately(a);
            assert!(approx);
        };

        when regex r"^(.+) <- normalize (.+)" (String, String) | world, variable_name, variable_to_normalize, _step | {
            let r1 = world.read_from_env_tuple(variable_to_normalize).unwrap();
            let a = r1.normalize();
            // Set up your context in given steps
            world.add_to_env_tuple(variable_name, a);
        };

    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

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
