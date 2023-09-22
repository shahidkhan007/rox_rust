use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::interpreter::Object;

#[derive(Clone)]
pub struct Env {
    enclosing: Box<Option<Env>>,
    values: HashMap<String, Object>,
}

#[derive(Debug, Clone)]
pub enum EnvError {
    VarNotFound(String),
    VarDefine(String),
    VarAssign(String),
}

impl Env {
    pub fn new(enclosing: Option<Env>) -> Env {
        Env {
            enclosing: Box::new(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, ident: String, value: Object) -> Result<(), EnvError> {
        self.values.insert(ident, value);
        Ok(())
    }

    pub fn assign(&mut self, ident: String, value: Object) -> Result<(), EnvError> {
        if self.values.contains_key(&ident[..]) {
            self.define(ident, value).unwrap();
        } else {
            match self.enclosing.as_mut() {
                Some(env) => {
                    env.define(ident, value).unwrap();
                }
                None => return Err(EnvError::VarAssign(format!("Undefined variable {ident}."))),
            }
        }

        Ok(())
    }

    pub fn get(&self, ident: String) -> Result<Object, EnvError> {
        match self.values.get(&ident[..]) {
            Some(val) => Ok(val.clone()),
            None => match *self.enclosing.clone() {
                Some(env) => env.get(ident),
                None => Err(EnvError::VarNotFound(format!(
                    "Cannot find the variable '{}' in the scope",
                    ident
                ))),
            },
        }
    }

    pub fn get_enclosing(&self) -> Option<Env> {
        *self.enclosing.clone()
    }
}
