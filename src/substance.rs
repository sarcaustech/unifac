use crate::functional_group::FunctionalGroup;

#[derive(Debug, Clone)]
pub struct Substance {
    pub fraction: f64,
    pub functional_groups: Vec<FunctionalGroup>,
    pub gamma: Option<f64>,
}

impl Substance {
    pub fn from(fraction: f64, functional_groups: Vec<FunctionalGroup>) -> Substance {
        Substance {
            fraction,
            functional_groups,
            gamma: None,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
