use csstype::CssUnitGroup;

#[derive(Clone, PartialEq)]
pub enum ConditionType {
    Keyword,
    Color,
    Unit(Vec<CssUnitGroup>),
}

impl ConditionType {
    fn as_condition(self) -> Condition {
        Condition {
            types_variant: vec![self],
        }
    }
}

pub struct Condition {
    pub types_variant: Vec<ConditionType>,
}

impl Condition {
    pub fn keyword() -> Condition {
        ConditionType::Keyword.as_condition()
    }
    pub fn color() -> Condition {
        ConditionType::Color.as_condition()
    }
    pub fn length_unit() -> Condition {
        ConditionType::Unit(vec![
            CssUnitGroup::Integer,
            CssUnitGroup::Number,
            CssUnitGroup::AbsoluteLength,
            CssUnitGroup::FontRelativeLength,
            CssUnitGroup::ViewportRelativeLength,
        ])
        .as_condition()
    }
    pub fn percentage_unit() -> Condition {
        ConditionType::Unit(vec![CssUnitGroup::Percentage]).as_condition()
    }
    pub fn or(self, cond: Condition) -> Self {
        Condition {
            types_variant: [self.types_variant, cond.types_variant].concat(),
        }
    }
}
