#![allow(dead_code)]

use thiserror::Error;

#[derive(Debug, Error,PartialEq, Eq)]
pub enum UserBuilderError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}

#[derive(Debug,Clone)]
pub struct User {
    pub name: String,
    pub age: Option<u8>,
    pub email: String,
    pub activo : bool,
    pub rolee : UserRole
}

#[derive(Debug,Clone)]
pub enum UserRole {
    Admin,
    Regular,
    Guest
}

pub struct UserBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u8>,
    activo : bool,
    rolee : UserRole
}

impl UserBuilder {
  pub fn new() -> Self {
      UserBuilder {
          name: None,
          email: None,
          age: None,
          activo : false,
          rolee : UserRole::Guest
      }
  }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    pub fn activo(mut self, activo: bool) -> Self {
        self.activo = activo;
        self
    }

    pub fn rolee(mut self, rolee: UserRole) -> Self {
        self.rolee = rolee;
        self
    }

    pub fn build(self) -> Result<User,UserBuilderError>{
        Ok(User {
            name: self.name.ok_or(UserBuilderError::MissingField("name"))?,
            email: self.email.ok_or(UserBuilderError::MissingField("email"))?,
            age: self.age,
            activo : self.activo,
            rolee : self.rolee
        })

    }
}

