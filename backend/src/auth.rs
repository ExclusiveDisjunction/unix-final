use std::{collections::{HashMap, HashSet}, fmt::Display, hash::Hash, ops::Deref, str::FromStr, sync::{Arc, Mutex, RwLock}};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

use crate::tool::lock::{MutexProvider, MutexProviderAccess, ProtectedAccess};

use crate::io::usr::{User, Username};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct JWT {
    val: String
}
impl PartialEq<str> for JWT {
    fn eq(&self, other: &str) -> bool {
        self.val == other
    }
}
impl Display for JWT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.val as &dyn Display).fmt(f)
    }
}
impl Hash for JWT {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state)
    }
}
impl FromStr for JWT {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Self {
            val: s.to_string()
            }
        )
    }
}
impl From<String> for JWT {
    fn from(value: String) -> Self {
        Self {
            val: value 
        }
    }
}
impl Deref for JWT {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

pub struct ExpiredTokens {
    data: Arc<RwLock<HashSet<JWT>>>
}
impl Default for ExpiredTokens{
    fn default() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashSet::new()))
        }
    }
}
impl ExpiredTokens {
    pub fn reset(&self) {
        {
            let mut guard = match self.data.write() {
                Ok(v) => v,
                Err(e) => e.into_inner()
            };

            *guard = HashSet::new();
        }

        self.data.clear_poison();
    }

    pub fn is_expired(&self, token: &JWT) -> bool {
        if let Ok(guard) = self.data.read() {
            guard.contains(token)
        }
        else {
            self.reset();
            false
        }
    }
    pub fn register_expired(&self, token: JWT) {
        if let Ok(mut guard) = self.data.write() {
            guard.insert(token);
        }
        else {
            self.reset();
        }
    }
}

lazy_static! {
    pub static ref EXPIRED: ExpiredTokens = ExpiredTokens::default();
}

#[derive(PartialEq, Debug)]
pub struct Session {
    target: User,
    jwt: Option<JWT>,
    is_new: bool
}
impl From<User> for Session {
    fn from(value: User) -> Self {
        Self {
            target: value,
            jwt: None,
            is_new: false
        }
    }
}
impl Session {
    pub fn new(user: User, jwt: Option<JWT>, is_new: bool) -> Self {
        Self {
            target: user,
            jwt,
            is_new
        }
    }

    pub fn get_auth(&self) -> Option<&JWT> {
        self.jwt.as_ref()
    }
    pub fn set_auth(&mut self, new: JWT) {
        self.expire_auth();
        self.jwt = Some(new);
    }
    pub fn expire_auth(&mut self) {
        if let Some(jwt) = self.jwt.take() {
            EXPIRED.register_expired(jwt);
        }
    }
    pub fn is_authenticated(&self) -> bool {
        self.jwt.is_some()
    }
}

#[derive(Debug, Default)]
pub struct UserSessions {
    data: HashMap<Username, Session>
}
impl UserSessions {
    pub fn fill(&mut self, data: Vec<User>) {
        let result = &mut self.data;
        for user in data {
            result.insert(user.username().clone(), user.into());
        }
    }

    pub fn register_authentication(&mut self, jwt: JWT, target: &Username) -> bool {
        if let Some(session) = self.data.get_mut(target) {
            session.set_auth(jwt);
            true
        }
        else {
            false
        }
    }
    pub fn register_new_user(&mut self, user: User, jwt: JWT) {
        if let Some(session) = self.data.get_mut(user.username()) {
            session.set_auth(jwt);
        }
        else {
            let username = user.username().clone();
            let session = Session::new(user, Some(jwt), true);
            self.data.insert(username, session);
        }
    }
    pub fn get_new_users(&self) -> Vec<&User> {
        self.data.iter().filter(|(_, x)| x.is_new).map(|(_, x)| &x.target).collect()
    }

    pub fn get_user_by_auth(&self, jwt: &JWT) -> Option<&Session> {
        for session in self.data.values() {
            if session.get_auth() == Some(jwt) {
                return Some(session)
            }
        }

        None
    }
    pub fn get_user_by_auth_mut(&mut self, jwt: &JWT) -> Option<&mut Session> {
        for session in self.data.values_mut() {
            if session.get_auth() == Some(jwt) {
                return Some(session)
            }
        }

        None
    }

    pub fn get_user_by_username(&self, username: &Username) -> Option<&Session> {
        self.data.get(username)
    }
    pub fn get_user_by_username_mut(&mut self, username: &Username) -> Option<&mut Session> {
        self.data.get_mut(username)
    }
}

pub struct SessionProvider {
    data: Arc<Mutex<UserSessions>>
}
impl Default for SessionProvider {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(UserSessions::default()))
        }
    }
}
impl MutexProvider for SessionProvider {
    type Data = UserSessions;
    fn access_raw(&self) -> ProtectedAccess<'_, Arc<Mutex<Self::Data>>> {
        ProtectedAccess::new(&self.data)
    }
}
impl MutexProviderAccess for SessionProvider { }

lazy_static! {
    pub static ref SESSIONS: SessionProvider = SessionProvider::default();
}