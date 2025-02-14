use async_trait::async_trait;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::error::Error as StdError;

mod events;
mod handle;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Priority {
    HIGHEST,
    HIGH,
    NORMAL,
    LOW,
    LOWEST,
}

#[async_trait]
pub trait EventBeforeListener {
    fn priority() -> Priority { Priority::NORMAL }
    type Event;
    type Error = ();
    async fn handle(&self, event: &mut Self::Event, cancelled: &mut bool) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait EventAfterListener {
    fn priority() -> Priority { Priority::NORMAL }
    type Event;
    async fn handle(&self, event: &Self::Event);
}

#[async_trait]
pub trait EventMutBeforeListener {
    fn priority() -> Priority { Priority::NORMAL }
    type Event;
    async fn handle(&mut self, event: &mut Self::Event, cancelled: &mut bool);
}

#[async_trait]
pub trait EventMutAfterListener {
    fn priority() -> Priority { Priority::NORMAL }
    type Event;
    async fn handle(&mut self, event: &Self::Event);
}

pub struct EventBus {
    before_listeners: (
        HashMap<TypeId, Vec<Arc<dyn EventBeforeListener<Event = (dyn Send + Sync)> + Send + Sync>>>,
        HashMap<
            TypeId,
            Vec<Arc<dyn EventMutBeforeListener<Event = (dyn Send + Sync)> + Send + Sync>>,
        >,
    ),
    after_listeners: (
        HashMap<TypeId, Vec<Arc<dyn EventAfterListener<Event = (dyn Send + Sync)> + Send + Sync>>>,
        HashMap<
            TypeId,
            Vec<Arc<dyn EventMutAfterListener<Event = (dyn Send + Sync)> + Send + Sync>>,
        >,
    ),
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            before_listeners: (HashMap::new(), HashMap::new()),
            after_listeners: (HashMap::new(), HashMap::new()),
        }
    }

    pub fn register_before<E, L>(&mut self, listener: L, priority: Priority)
    where
        E: Send + Sync,
        L: EventBeforeListener<Event = E> + Send + Sync,
    {
        let x = L::handle;
        
        x
        // self.before_listeners
        //     .0
        //     .entry(TypeId::of::<E>())
        //     .or_insert_with(Vec::new)
        //     .push(Arc::new(listener));
    }

    pub fn register_after<E, L>(&mut self, listener: L, priority: Priority)
    where
        E: Send + Sync + 'static,
        L: EventAfterListener<Event = E> + Send + Sync + 'static,
    {
        self.after_listeners.0
            .entry(TypeId::of::<E>())
            .or_insert_with(Vec::new)
            .push(Arc::new(listener));
    }

    pub async fn dispatch_before<E: Send + Sync>(&self, event: &mut E) -> bool {
        if let Some(listeners) = self.before_listeners.get(&TypeId::of::<E>()) {
            for listener in listeners {
                let listener = listener
                    .downcast_ref::<dyn EventBeforeListener<EventBefore = E>>()
                    .unwrap();
                listener.handle(event, &mut true).await;
            }
        }
        true
    }

    pub async fn dispatch_after<E: Send + Sync + Sized + Any>(&self, event: &E) {
        if let Some(listeners) = self.after_listeners.get(&TypeId::of::<E>()) {
            for listener in &listeners {
                listener.handle(event).await;
            }
        }
    }
}
