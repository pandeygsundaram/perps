use std::{collections::HashMap, sync::OnceLock};
use tokio::sync::{RwLock, mpsc::UnboundedSender};

use crate::types::SenderChannelObject;
use uuid::Uuid;

pub struct UserManager {
    users: HashMap<Uuid, User>,
}

impl UserManager {
    fn new() -> UserManager {
        UserManager {
            users: HashMap::new(),
        }
    }

    pub fn get_instance() -> &'static RwLock<UserManager> {
        INSTANCE.get_or_init(|| RwLock::new(UserManager::new()))
    }

    pub fn add_user(&mut self, tx: UnboundedSender<SenderChannelObject>) -> Uuid {
        let id = Uuid::new_v4();
        self.users.insert(id, User::new(id, tx));
        id
    }

    pub fn remove_user(&mut self, id: Uuid) {
        self.users.remove(&id);
    }

    pub fn emit(&self, id: &Uuid, message: SenderChannelObject) {
        // get the user from self
        // then call the send

        let user = self.users.get(&id);
        match user {
            Some(user) => {
                let _ = user.tx.send(message);
            }
            None => {
                // don't send ig
                // we need to log it though
                eprintln!("User {} not found in UserManager", id);
            }
        }
    }
}

// now this enforces that thorughout the code there is going to be only a single instance for the usermanagerstate!!!
// there can't be any more than one!
static INSTANCE: OnceLock<RwLock<UserManager>> = OnceLock::new();

// the onelock ensures that in the memory that value exsits only once!!
// it can't be created again

// rw lock will be on the manager instance
// rather than the hashmap
// because we don't want to expose the hashmap
// instead the

struct User {
    pub id: Uuid,
    pub tx: UnboundedSender<SenderChannelObject>,
}

impl User {
    fn new(id: Uuid, tx: UnboundedSender<SenderChannelObject>) -> User {
        User { id, tx }
    }
}
