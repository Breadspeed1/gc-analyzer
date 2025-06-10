use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    ptr::addr_of,
};

use serde::Deserialize;

use crate::math::{self, MixtureOptimization};

const DEFAULT_LABEL: &str = "Mixed";

const DEFAULT_PURITY: f64 = 0.995;

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct RefrigerantName(String);

#[derive(Deserialize, PartialEq, Debug, Default)]
#[serde(try_from = "HashMap<String, RefrigerantClassification>")]
pub struct ClassificationList<'a>(Vec<(String, RefrigerantClassification<'a>)>);

#[derive(Deserialize, PartialEq, Debug)]

pub struct RefrigerantMixture<'a> {
    identifier: RefrigerantName,
    components: HashMap<RefrigerantName, f64>,
    #[serde(default)]
    classifications: ClassificationList<'a>,
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

#[derive(Deserialize, Debug, Clone)]
#[serde(from = "RefrigerantName")]
enum RefrigerantRef<'a> {
    Unresolved(RefrigerantName),
    Resolved(&'a RefrigerantMixture<'a>),
}

#[derive(Debug, Deserialize)]
pub struct RefrigerantClassification<'a> {
    purity: f64,
    max_lows: Option<f64>,
    #[serde(default)]
    mixed_with: HashMap<RefrigerantRef<'a>, f64>,
}

impl<'a> Eq for RefrigerantClassification<'a> {}

impl<'a> PartialEq for RefrigerantClassification<'a> {
    fn eq(&self, other: &Self) -> bool {
        addr_of!(self) == addr_of!(other)
    }
}

impl<'a> PartialEq for RefrigerantRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unresolved(l0), Self::Unresolved(r0)) => l0 == r0,
            (Self::Resolved(l0), Self::Resolved(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<'a> Hash for RefrigerantRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<'a> Eq for RefrigerantRef<'a> {}

impl<'a> From<RefrigerantName> for RefrigerantRef<'a> {
    fn from(value: RefrigerantName) -> Self {
        Self::Unresolved(value)
    }
}

impl<'a> RefrigerantClassification<'a> {
    fn evaluate(
        &self,
        reading: &GCReading,
        origin: &RefrigerantMixture,
    ) -> math::OptimizationResult<'a> {
        if math::valid_comparison(reading, origin) {
            return Err("Not a valid comparison".into());
        }

        let max_low = math::find_max_low(reading, origin);

        if self.max_lows.is_some_and(|l| max_low > l) {
            return Err("Max lows not within bounds".into());
        }

        todo!()
    }
}

impl Display for RefrigerantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> TryFrom<HashMap<String, RefrigerantClassification<'a>>> for ClassificationList<'a> {
    type Error = String;

    fn try_from(
        value: HashMap<String, RefrigerantClassification<'a>>,
    ) -> Result<Self, Self::Error> {
        Ok(Self(value.into_iter().collect()))
    }
}

impl<'a> ClassificationList<'a> {
    pub fn get_classification(
        &self,
        reading: &GCReading,
        origin: &RefrigerantMixture<'a>,
    ) -> ClassificationResult {
        match self.0.iter().find_map(|(name, class)| {
            class
                .evaluate(reading, origin)
                .map(|(results, fin)| (results, fin, name))
                .ok()
        }) {
            Some((mixtures, _, name)) => ClassificationResult {
                label: name.clone(),
                origin: origin.identifier().clone(),
                purity: mixtures
                    .iter()
                    .find(|m| m.1.identifier() == origin.identifier())
                    .unwrap()
                    .0,
                components: mixtures
                    .iter()
                    .map(|o| (o.1.identifier().clone(), o.0))
                    .collect(),
            },
            None => ClassificationResult {
                label: DEFAULT_LABEL.into(),
                origin: origin.identifier().clone(),
                purity: 0.,
                components: HashMap::new(),
            },
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

impl<'a> RefrigerantMixture<'a> {
    pub fn new(
        identifier: RefrigerantName,
        components: HashMap<RefrigerantName, f64>,
        classifications: ClassificationList<'a>,
    ) -> Self {
        Self {
            identifier,
            components,
            classifications,
        }
    }

    pub fn classify(&self, reading: &GCReading) -> ClassificationResult {
        self.classifications.get_classification(reading, self)
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
