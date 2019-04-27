use crate::core::property::{register_property, util, Parameter, Property};

pub struct Instance;

impl Property for Instance {
    fn register(&self) {
        self.register_keyword(vec!["none", "left", "right", "both"]);
        register_property(Instance);
    }
    fn name(&self) -> &str {
        "clear"
    }

    fn verify(&self, parameters: &Vec<Parameter>) -> bool {
        util::when_size_1(parameters, |parameter| {
            util::when_keyword(parameter, |parameter| self.check_keyword(parameter))
        })
    }
}
