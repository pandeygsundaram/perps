// subscription_manager.rs

// Maintains the relationship between Redis channels and WebSocket connections. It handles subscribe() and unsubscribe(), 
// tracks which connections belong to each channel, and 
// receives Redis Pub/Sub messages. When Redis sends a message, 
// it finds every subscribed connection and sends the message through that
// connection's WebSocket sender. When a connection disappears, it removes it from all subscriptions.