use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use serde::Deserialize;

use crate::math;

const DEFAULT_LABEL: &str = "Mixed";

const DEFAULT_PURITY: f64 = 0.995;

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct RefrigerantName(String);

#[derive(Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(try_from = "HashMap<String, RefrigerantClassification>")]
pub struct ClassificationList(Vec<(String, RefrigerantClassification)>);

#[derive(Deserialize, PartialEq, Debug, Clone)]

pub struct RefrigerantMixture {
    identifier: RefrigerantName,
    components: HashMap<RefrigerantName, f64>,
    #[serde(default)]
    classifications: ClassificationList,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GCReading {
    components: HashMap<RefrigerantName, f64>,
}

#[derive(Debug, Clone)]
pub struct ClassificationResult {
    pub label: String,
    pub origin: RefrigerantName,
    pub purity: f64,
    pub components: HashMap<RefrigerantName, f64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RefrigerantClassification {
    purity: f64,
    #[serde(default)]
    constraints: Vec<ClassificationConstraint>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum ClassificationConstraint {
    MixedWith(HashMap<RefrigerantName, f64>),
    LowsLessThan(f64),
}

impl ClassificationConstraint {}

impl Display for RefrigerantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<HashMap<String, RefrigerantClassification>> for ClassificationList {
    type Error = String;

    fn try_from(value: HashMap<String, RefrigerantClassification>) -> Result<Self, Self::Error> {
        Ok(Self(value.into_iter().collect()))
    }
}

impl ClassificationList {
    pub fn get_classification(&self, purity: f64, pure_name: &String) -> String {
        if purity >= DEFAULT_PURITY {
            return pure_name.clone();
        }

        match self
            .0
            .binary_search_by(|(_, v)| v.purity.partial_cmp(&purity).expect("NaN???"))
        {
            Ok(i) => self.0[i].0.clone(),
            Err(i) if i > 0 => self.0[i - 1].0.clone(),
            _ => DEFAULT_LABEL.into(),
        }
    }
}

impl GCReading {
    pub fn new(components: HashMap<RefrigerantName, f64>) -> Self {
        Self { components }
    }

    pub fn get_component(&self, name: &RefrigerantName) -> Option<&f64> {
        self.components.get(name)
    }

    pub fn component_set(&self) -> HashSet<&RefrigerantName> {
        self.components.keys().collect()
    }

    pub fn components(&self) -> impl Iterator<Item = (&RefrigerantName, &f64)> {
        self.components.iter()
    }
}

impl TryFrom<String> for GCReading {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let data = value.split(",").map(|s| {
            let parts: Vec<&str> = s.trim().split(" ").collect();
            let name = RefrigerantName::new(&parts[0].into()).unwrap();
            let concentration = parts[1].to_string().trim().parse::<f64>().unwrap();
            (name, concentration)
        });

        let data: HashMap<RefrigerantName, f64> = HashMap::from_iter(data);

        Ok(GCReading::new(data))
    }
}

impl Display for ClassificationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Origin: {}, Classified Label: {}, Purity: {:.3}%, {} total components.",
            self.origin.0,
            self.label,
            self.purity * 100.0,
            self.components.len()
        )
    }
}

impl RefrigerantMixture {
    pub fn new(
        identifier: RefrigerantName,
        components: HashMap<RefrigerantName, f64>,
        classifications: ClassificationList,
    ) -> Self {
        Self {
            identifier,
            components,
            classifications,
        }
    }

    pub fn classify(&self, reading: &GCReading) -> Option<ClassificationResult> {
        let purity = math::find_concentration(reading, self);

        purity.map(|result| ClassificationResult {
            label: self
                .classifications
                .get_classification(result, &self.identifier.0),
            origin: self.identifier.clone(),
            purity: result,
            components: self
                .components
                .clone()
                .into_iter()
                .map(|(n, v)| (n, v * result))
                .collect(),
        })
    }

    pub fn classify_optimize(&self, reading: &GCReading) -> Option<ClassificationResult> {
        if !math::valid_comparison(reading, &self) {
            return None;
        }

        let result = math::optimize(reading, vec![(&self, 0.)]).0[0].0;

        Some(ClassificationResult {
            label: self
                .classifications
                .get_classification(result, &self.identifier.0),
            origin: self.identifier.clone(),
            purity: result,
            components: self
                .components
                .clone()
                .into_iter()
                .map(|(n, v)| (n, v * result))
                .collect(),
        })
    }

    pub fn get_component(&self, name: &RefrigerantName) -> Option<&f64> {
        self.components.get(name)
    }

    pub fn components(&self) -> impl Iterator<Item = (&RefrigerantName, &f64)> {
        self.components.iter()
    }

    pub fn identifier(&self) -> &RefrigerantName {
        &self.identifier
    }

    pub fn component_set(&self) -> HashSet<&RefrigerantName> {
        self.components.keys().collect()
    }
}

impl Hash for RefrigerantName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl RefrigerantName {
    pub fn new(name: &String) -> Option<Self> {
        Self::normalize(name).map(RefrigerantName)
    }

    fn normalize(name: &String) -> Option<String> {
        Some(name.to_uppercase().replace(" ", ""))
    }
}

impl AsRef<String> for RefrigerantName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for RefrigerantName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value).ok_or("Unable to parse refrigerant name".into())
    }
}
