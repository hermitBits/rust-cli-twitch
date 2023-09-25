use serde::{Deserialize, Serialize};
use std::fs;

const DB_PATH: &str = "./data/data-base.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Channel {
    pub name: String,
    pub url: String,
    pub online: bool,
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    IOError(std::io::Error),
    JSONError(serde_json::Error),
    NotFound,
    AlreadyExists,
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::IOError(err) => write!(f, "IO Error: {}", err),
            RepositoryError::JSONError(err) => write!(f, "JSON Error: {}", err),
            RepositoryError::NotFound => write!(f, "Not Found"),
            RepositoryError::AlreadyExists => write!(f, "Already Exists"),
        }
    }
}

impl Error for RepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RepositoryError::IOError(err) => Some(err),
            RepositoryError::JSONError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for RepositoryError {
    fn from(err: std::io::Error) -> Self {
        RepositoryError::IOError(err)
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(err: serde_json::Error) -> Self {
        RepositoryError::JSONError(err)
    }
}

pub struct ChannelRepository {
    database: Vec<Channel>,
}

impl ChannelRepository {
    pub fn new() -> Result<Self, RepositoryError> {
        let database_string = fs::read_to_string(DB_PATH).map_err(RepositoryError::IOError)?;
        let database: Vec<Channel> =
            serde_json::from_str(&database_string).map_err(RepositoryError::JSONError)?;
        Ok(ChannelRepository { database })
    }

    pub fn add(&mut self, new_channel: Channel) -> Result<(), RepositoryError> {
        if self.get_by_name(&new_channel.name).is_some() || self.get_by_url(&new_channel.url).is_some() {
            return Err(RepositoryError::AlreadyExists);
        }

        self.database.push(new_channel);
        self.save_database()?;
        Ok(())
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Channel> {
        self.database.iter().find(|channel| channel.name == name)
    }

    pub fn get_by_url(&self, url: &str) -> Option<&Channel> {
        self.database.iter().find(|channel| channel.url == url)
    }

    pub fn delete_by_name(&mut self, name: &str) -> Result<(), RepositoryError> {
        if let Some(index) = self.database.iter().position(|channel| channel.name == name) {
            self.database.remove(index);
            self.save_database()?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    pub fn delete_by_url(&mut self, url: &str) -> Result<(), RepositoryError> {
        if let Some(index) = self.database.iter().position(|channel| channel.url == url) {
            self.database.remove(index);
            self.save_database()?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    fn save_database(&self) -> Result<(), RepositoryError> {
        let database_string = serde_json::to_string(&self.database).map_err(RepositoryError::JSONError)?;
        fs::write(DB_PATH, &database_string).map_err(RepositoryError::IOError)?;
        Ok(())
    }
}