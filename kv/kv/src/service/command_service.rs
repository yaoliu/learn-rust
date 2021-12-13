use http::HeaderMap;
use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(value)) => value.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(error) => error.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into()
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, self.key, self.value) {
                Ok(Some(value)) => value.into(),
                Ok(None) => Value::default().into(),
                Err(err) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.table, &self.key) {
            Ok(Some(value)) => value.into(),
            Err(error) => error.into(),
            _ => {}
        }
    }
}