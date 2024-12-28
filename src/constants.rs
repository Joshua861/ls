use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    data::{Data, ToData},
    expr::VariableMap,
};

pub fn constants() -> VariableMap {
    let mut map = HashMap::new();

    for (name, value) in [
        ("PI", Data::Number(Decimal::PI)),
        ("E", Data::Number(Decimal::E)),
        ("E_INVERSE", Data::Number(Decimal::E)),
        ("HALF_PI", Data::Number(Decimal::HALF_PI)),
        ("QUARTER_PI", Data::Number(Decimal::QUARTER_PI)),
    ] {
        map.insert(name.to_string(), value);
    }

    map
}
