use std::{
    collections::HashMap,
    sync::OnceLock,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::user;

pub struct SubscriptionManager {
    subscriptions: HashMap<Uuid, Vec<String>>,
    reverse_subscriptions: HashMap<String, Vec<Uuid>>,
}

static INSTANCE: OnceLock<RwLock<SubscriptionManager>> = OnceLock::new();

impl SubscriptionManager {
    fn new() -> SubscriptionManager {
        todo!()
    }
    pub fn get_instance() -> &'static RwLock<SubscriptionManager> {
        INSTANCE.get_or_init(|| RwLock::new(SubscriptionManager::new()))
    }
    pub fn subscribe(&mut self , user_id : Uuid , channel : String) {
        
        // sae the user id in the subscriptions map
        // save the user id in the reverse subscriptions map 
        // that's it
       let channels = self.subscriptions.entry(user_id).or_insert_with(Vec::new);

       channels.push(channel.clone());
       
       let reverse_subscriptions = self.reverse_subscriptions.entry(channel).or_insert_with(Vec::new);
       reverse_subscriptions.push(user_id);




    }
    pub fn unsubscribe(&self) {
        // remove the user id from the subscriptions map
        // remove the userid frmo th reverse subscriptions map
        // done

    }
    pub fn broadcast(&self) {
        // for all the user id in the reversesubscription call user.emit

    }
    pub fn connection_left(&self) {

        // call unsubsribe on him
        


    }
}
