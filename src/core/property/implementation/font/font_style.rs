use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, util, Property};

pub struct Instance;

impl Property for Instance {
    fn register(&self) {
        self.register_keyword(vec!["normal", "italic", "oblique"]);
        register_property(Instance);
    }
    fn name(&self) -> &str {
        "font-style"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            false
        }
    }
}