use std::{
    collections::{HashMap, HashSet}, sync::OnceLock,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{types::SenderChannelObject,  user_manager::UserManager};

pub struct SubscriptionManager {
    subscriptions: HashMap<Uuid, HashSet<String>>,
    reverse_subscriptions: HashMap<String, HashSet<Uuid>>,
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
       let channels = self.subscriptions.entry(user_id).or_insert_with(HashSet::new );

       channels.insert(channel.clone());
       
       let reverse_subscriptions = self.reverse_subscriptions.entry(channel).or_insert_with(HashSet::new);
       reverse_subscriptions.insert(user_id);




    }
    pub fn unsubscribe(&mut self , user_id : Uuid) {
        // remove the user id from the subscriptions map
        // remove the userid frmo th reverse subscriptions map
        
        let Some( users_channel) = self.subscriptions.remove(&user_id) else {
            // user not found
            eprint!("User not found");


            return;
        };

        for i in users_channel{
            // now pop out user if frmo there
            if let Some(user_data)=  self.reverse_subscriptions.get_mut(&i)  {
                
                user_data.remove(&user_id);

                if user_data.is_empty(){
                    user_data.remove(&user_id);
                }                
            };
        }
        println!("Properly ub subscribed the user");


        // done

    }

    pub async fn broadcast(&self , channel : String , message : SenderChannelObject ) {
        // for all the user id in the reversesubscription call user.emit

        // get all the users in that channel
        let Some(channel_subs) = self.reverse_subscriptions.get(&channel) else {

            eprintln!("Wrong channel name or channel not found");
            return;
        } ;

        // get the usermanager instance
        let user_handler = & UserManager::get_instance().read().await;


        // call the emit method on them
        for id in channel_subs{
            user_handler.emit(id, message.clone());

        }


        println!("Called the broadcast successfully");


    }
    pub async fn connection_left(&mut self , user_id : Uuid) {

        // call unsubsribe on him
        self.unsubscribe(user_id);

        // maybe a way to call the delete method on user from the usermanager
        // here delete him from the usermanager map as well 
        let user_handler = &mut UserManager::get_instance().write().await;
        user_handler.remove_user(user_id);

    }
}
