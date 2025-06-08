use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
pub struct RefrigerantName(String);

#[derive(Deserialize, Debug)]

pub struct RefrigerantComponent(pub RefrigerantName, pub f64);

#[derive(Deserialize, PartialEq, Debug)]

pub struct RefrigerantMixture {
    identifier: MixtureIdentifier,
    components: HashMap<RefrigerantName, f64>,
    price: f64,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum MixtureIdentifier {
    ID(u64),
    Name(RefrigerantName),
}

impl RefrigerantMixture {
    pub fn new(
        identifier: MixtureIdentifier,
        components: HashMap<RefrigerantName, f64>,
        price: f64,
    ) -> Self {
        Self {
            identifier,
            components,
            price,
        }
    }

    pub fn add_component(&mut self, name: RefrigerantName, concentration: f64) -> bool {
        self.components.insert(name, concentration).is_some()
    }

    pub fn components(&self) -> impl Iterator<Item = (&RefrigerantName, &f64)> {
        self.components.iter()
    }

    pub fn get_component(&self, name: &RefrigerantName) -> Option<&f64> {
        self.components.get(name)
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn identifier(&self) -> &MixtureIdentifier {
        &self.identifier
    }

    pub fn component_set(&self) -> HashSet<&RefrigerantName> {
        self.components.keys().collect()
    }
}

impl PartialEq for RefrigerantComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RefrigerantComponent {}

impl Hash for RefrigerantName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Hash for RefrigerantComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        //dont hash percent to avoid duplicating components in sets
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

impl RefrigerantComponent {
    pub fn name(&self) -> &RefrigerantName {
        &self.0
    }

    pub fn concentration(&self) -> f64 {
        self.1
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
