use std::{
    collections::{HashMap, HashSet}, sync::OnceLock,
};

use tokio::sync::RwLock;
use uuid::Uuid;


pub struct SubscriptionManager {
    subscriptions: HashMap<Uuid, HashSet<String>>,
    reverse_subscriptions: HashMap<String, HashSet<Uuid>>,
}

static INSTANCE: OnceLock<RwLock<SubscriptionManager>> = OnceLock::new();

impl SubscriptionManager {
    fn new() -> SubscriptionManager {
        SubscriptionManager{
            subscriptions: HashMap::new(),
            reverse_subscriptions: HashMap::new()
        }
        
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
    pub fn unsubscribe(&mut self , user_id : Uuid , channel : String) {
        // remove the user id from the subscriptions map
        // remove the userid frmo th reverse subscriptions map



        // when i am doing unsubscribe 
        // then subscription[userid].remove(channel)

        // if subscription[userid].size()==0 
        // subscriptions.remove(userid)
        
        


        let Some( users_channel) = self.subscriptions.get_mut(&user_id) else {
            // user not found
            eprint!("User not found");
            return;
        }; 
        users_channel.remove(&channel);
        if users_channel.is_empty(){
            self.subscriptions.remove(&user_id);
        } 

        // reveresesubscription[channel].remove(userid)
        // if reveresesubscription[channel].isempty()
        // reveresesubscriptions.remove(channel)


        // now pop out user if frmo there
        if let Some(user_data)=  self.reverse_subscriptions.get_mut(&channel)  {
            
            user_data.remove(&user_id);

            if user_data.is_empty(){
                self.reverse_subscriptions.remove(&channel);
            }                
        } else {
            eprintln!("Either channel not found , yeah channel not found");
        };


        

        println!("Properly ub subscribed the user");


        // done

    }

    pub  fn broadcast(&self , channel : String  ) -> Result<HashSet<Uuid> , String> {
        // for all the user id in the reversesubscription call user.emit

        // get all the users in that channel
        let Some(channel_subs) = self.reverse_subscriptions.get(&channel).cloned() else {

            eprintln!("Wrong channel name or channel not found");
            return Err("channel not found".to_string());
        } ;


        println!("Called the broadcast successfully");
        Ok(channel_subs)


    }
    pub async fn connection_left(&mut self , user_id : Uuid) {

        // call unsubsribe on him
        let Some(channels) =  self.subscriptions.remove(&user_id) else {
            eprintln!("User Not Found");
            return; 
        };
        for i in channels{
            let  Some(user_data) =self.reverse_subscriptions.get_mut(&i) else {
                eprintln!("{i} not found in the reveresesubscription " );
                continue;
            };
            user_data.remove(&user_id);

            // if channel is no more pointing to any users remove the channel
            if user_data.is_empty(){
                self.reverse_subscriptions.remove(&i);
            }
        }



        // maybe a way to call the delete method on user from the usermanager
        // here delete him from the usermanager map as well 


    }
}
