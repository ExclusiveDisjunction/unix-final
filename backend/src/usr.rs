use std::{fmt::{Debug, Display}, hash::Hash, ops::Deref, str::FromStr};

use serde::{Serialize, Deserialize};
use sea_query::*;

use crate::{auth::JWT, db::DatabaseCallable, msg::{MessageBasis, RequestMessage, ResponseMessage}};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Username {
    val: String
}
impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.val as &dyn Display).fmt(f)
    }
}
impl Debug for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.val as &dyn Debug).fmt(f)
    }
}
impl Hash for Username {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state)
    }
}
impl PartialEq<str> for Username {
    fn eq(&self, other: &str) -> bool {
        self.val == other
    }
}
impl FromStr for Username {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Self {
                val: s.to_string()
            }
        )
    }
}
impl From<String> for Username {
    fn from(value: String) -> Self {
        Self {
            val: value
        }
    }
}
impl Deref for Username {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.val.deref()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
#[enum_def]
pub struct RawUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String
}
impl TryFrom<RawUser> for User {
    type Error = bcrypt::BcryptError;
    fn try_from(value: RawUser) -> Result<Self, Self::Error> {
        Ok(
            User {
                username: value.username.into(),
                first_name: value.first_name,
                last_name: value.last_name,
                hash_parts: value.password.parse()?
            }
        )
    }
}
impl From<User> for RawUser {
    fn from(value: User) -> Self {
        Self {
            username: value.username.val,
            first_name: value.first_name,
            last_name: value.last_name,
            password: value.hash_parts.to_string()
        }
    }
}
impl DatabaseCallable for RawUser {
    type Identity = RawUserIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            RawUserIden::Username,
            RawUserIden::FirstName,
            RawUserIden::LastName,
            RawUserIden::Password
        ]
    }
    fn table() -> Self::Identity {
        RawUserIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let username: &str = row.get(0);
        let first_name: &str = row.get(1);
        let last_name: &str = row.get(2);
        let password: &str = row.get(3);

        Self {
            username: username.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password: password.to_string()
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(RawUserIden::Username)
                    .text()
                    .primary_key()
            )
            .col(
                ColumnDef::new(RawUserIden::FirstName)
                    .text()
                    .not_null()
            )
            .col(
                ColumnDef::new(RawUserIden::LastName)
                    .text()
                    .not_null()
            )
            .col(
                ColumnDef::new(RawUserIden::Password)
                    .text()
                    .not_null()
            );
    }
}
impl RawUser {
    pub fn new(username: String, first_name: String, last_name: String, password: String) -> Self {
        Self {
            username,
            first_name,
            last_name,
            password
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct User {
    username: Username,
    first_name: String,
    last_name: String,
    hash_parts: bcrypt::HashParts
}
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.username)
    }
}
impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.first_name.hash(state);
        self.last_name.hash(state);
        self.username.hash(state);
        self.hash_parts.to_string().hash(state);
    }
}
impl User {
    pub fn new(username: Username, first_name: String, last_name: String,hash_parts: bcrypt::HashParts) -> Self {
        Self {
            username,
            first_name,
            last_name,
            hash_parts
        }
    }

    pub fn username(&self) -> &Username {
        &self.username
    }
    pub fn set_username(&mut self, new: Username) {
        self.username = new
    }
    pub fn get_names(&self) -> (&str, &str) {
        (&self.first_name, &self.last_name)
    }
    pub fn password(&self) -> String {
        self.hash_parts.to_string()   
    }
    pub fn compute_password(&mut self, new: String) -> bcrypt::BcryptResult<()> {
        self.hash_parts = bcrypt::hash_with_result(new, bcrypt::DEFAULT_COST)?;
        Ok( () )
    }
    pub fn salt(&self) -> String {
        self.hash_parts.get_salt()
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Hash)]
pub struct NetworkUser {
    jwt: String,
    username: String,
    first_name: String,
    last_name: String
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
struct SignInRequest {
    username: String,
    password: String
}
impl MessageBasis for SignInRequest { }
impl RequestMessage for SignInRequest { }

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    first_name: String,
    last_name: String,
    password: String
}
impl MessageBasis for CreateUserRequest { }
impl RequestMessage for CreateUserRequest { }

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
struct SignInResponse {
    ok: bool,
    message: String,
    jwt: Option<JWT>
}
impl MessageBasis for SignInResponse { }
impl ResponseMessage for SignInResponse { }