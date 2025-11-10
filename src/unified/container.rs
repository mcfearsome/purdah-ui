//! State container for managing both TEA models and Flux stores.
//!
//! The container provides a unified interface for registering and accessing
//! state from both architectural patterns.

use super::dispatcher::UnifiedDispatcher;
use crate::tea::{TeaModel, Message};
use crate::flux::{FluxStore, Action};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Container that holds both TEA models and Flux stores.
///
/// The state container manages the lifecycle of all state objects in the application,
/// regardless of which pattern they use.
pub struct StateContainer {
    /// TEA models, stored by their type ID.
    tea_models: Arc<RwLock<HashMap<TypeId, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,

    /// Flux stores, stored by their type ID.
    flux_stores: Arc<RwLock<HashMap<TypeId, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,

    /// Shared dispatcher for both patterns.
    dispatcher: Arc<UnifiedDispatcher>,
}

impl StateContainer {
    /// Creates a new state container with the given dispatcher.
    pub fn new(dispatcher: Arc<UnifiedDispatcher>) -> Self {
        Self {
            tea_models: Arc::new(RwLock::new(HashMap::new())),
            flux_stores: Arc::new(RwLock::new(HashMap::new())),
            dispatcher,
        }
    }

    /// Adds a TEA model to the container and registers it with the dispatcher.
    ///
    /// Returns a handle that can be used to read state and dispatch messages.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let container = StateContainer::new(dispatcher);
    /// let counter_handle = container.add_tea(CounterModel::init().0);
    /// ```
    pub fn add_tea<M>(&self, model: M) -> TeaHandle<M>
    where
        M: TeaModel + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<M>();
        let model_arc = Arc::new(RwLock::new(Box::new(model) as Box<dyn Any + Send + Sync>));

        self.tea_models
            .write()
            .unwrap()
            .insert(type_id, Arc::clone(&model_arc));

        // Register message handler with dispatcher
        let model_clone = Arc::clone(&model_arc);
        self.dispatcher.register_tea(move |msg: &M::Msg| {
            let mut model_guard = model_clone.write().unwrap();
            if let Some(tea_model) = model_guard.downcast_mut::<M>() {
                let _cmd = tea_model.update(msg.clone());
                // TODO: Execute command
            }
        });

        TeaHandle {
            model: model_arc,
            dispatcher: Arc::clone(&self.dispatcher),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Adds a Flux store to the container and registers it with the dispatcher.
    ///
    /// Returns a handle that can be used to read state and dispatch actions.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let container = StateContainer::new(dispatcher);
    /// let user_handle = container.add_flux(UserStore::new());
    /// ```
    pub fn add_flux<S>(&self, store: S) -> FluxHandle<S>
    where
        S: FluxStore + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<S>();
        let store_arc = Arc::new(RwLock::new(Box::new(store) as Box<dyn Any + Send + Sync>));

        self.flux_stores
            .write()
            .unwrap()
            .insert(type_id, Arc::clone(&store_arc));

        // Register action handler with dispatcher
        let store_clone = Arc::clone(&store_arc);
        self.dispatcher.register_flux(move |action: &S::Action| {
            let mut store_guard = store_clone.write().unwrap();
            if let Some(flux_store) = store_guard.downcast_mut::<S>() {
                flux_store.reduce(action);
            }
        });

        FluxHandle {
            store: store_arc,
            dispatcher: Arc::clone(&self.dispatcher),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Gets a handle to a TEA model if it exists.
    pub fn get_tea<M: TeaModel + 'static>(&self) -> Option<TeaHandle<M>> {
        let type_id = TypeId::of::<M>();
        self.tea_models
            .read()
            .unwrap()
            .get(&type_id)
            .map(|model| TeaHandle {
                model: Arc::clone(model),
                dispatcher: Arc::clone(&self.dispatcher),
                _phantom: std::marker::PhantomData,
            })
    }

    /// Gets a handle to a Flux store if it exists.
    pub fn get_flux<S: FluxStore + 'static>(&self) -> Option<FluxHandle<S>> {
        let type_id = TypeId::of::<S>();
        self.flux_stores
            .read()
            .unwrap()
            .get(&type_id)
            .map(|store| FluxHandle {
                store: Arc::clone(store),
                dispatcher: Arc::clone(&self.dispatcher),
                _phantom: std::marker::PhantomData,
            })
    }
}

/// Handle to a TEA model in the state container.
///
/// Provides read access to the model's state and the ability to dispatch messages.
pub struct TeaHandle<M: TeaModel> {
    model: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
    dispatcher: Arc<UnifiedDispatcher>,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: TeaModel + 'static> TeaHandle<M> {
    /// Gets a snapshot of the model's current state.
    pub fn state(&self) -> M::State {
        let model = self.model.read().unwrap();
        let tea_model = model.downcast_ref::<M>().unwrap();
        tea_model.state()
    }

    /// Dispatches a message to this model.
    ///
    /// The message will be processed by the model's update function.
    pub fn dispatch(&self, msg: M::Msg) {
        // Dispatch the message through the unified dispatcher
        let model_clone = Arc::clone(&self.model);
        let mut model_guard = model_clone.write().unwrap();
        if let Some(tea_model) = model_guard.downcast_mut::<M>() {
            let _cmd = tea_model.update(msg);
            // TODO: Execute command
        }
    }

    /// Gets a reference to the shared dispatcher.
    pub fn dispatcher(&self) -> Arc<UnifiedDispatcher> {
        Arc::clone(&self.dispatcher)
    }
}

impl<M: TeaModel + 'static> Clone for TeaHandle<M> {
    fn clone(&self) -> Self {
        Self {
            model: Arc::clone(&self.model),
            dispatcher: Arc::clone(&self.dispatcher),
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Handle to a Flux store in the state container.
///
/// Provides read access to the store's state and the ability to dispatch actions.
pub struct FluxHandle<S: FluxStore> {
    store: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
    dispatcher: Arc<UnifiedDispatcher>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S: FluxStore + 'static> FluxHandle<S> {
    /// Gets a snapshot of the store's current state.
    pub fn state(&self) -> S::State {
        let store = self.store.read().unwrap();
        let flux_store = store.downcast_ref::<S>().unwrap();
        flux_store.state()
    }

    /// Dispatches an action to this store.
    ///
    /// The action will be processed by the store's reduce function.
    pub fn dispatch(&self, action: S::Action) {
        // Dispatch the action through the unified dispatcher
        let store_clone = Arc::clone(&self.store);
        let mut store_guard = store_clone.write().unwrap();
        if let Some(flux_store) = store_guard.downcast_mut::<S>() {
            flux_store.reduce(&action);
        }
    }

    /// Gets a reference to the shared dispatcher.
    pub fn dispatcher(&self) -> Arc<UnifiedDispatcher> {
        Arc::clone(&self.dispatcher)
    }
}

impl<S: FluxStore + 'static> Clone for FluxHandle<S> {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            dispatcher: Arc::clone(&self.dispatcher),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestModel {
        count: i32,
    }

    #[derive(Debug, Clone)]
    enum TestMsg {
        Increment,
        Decrement,
    }

    impl Message for TestMsg {}

    impl TeaModel for TestModel {
        type State = i32;
        type Msg = TestMsg;

        fn init() -> (Self, crate::tea::Command<Self::Msg>) {
            (Self { count: 0 }, crate::tea::Command::None)
        }

        fn update(&mut self, msg: Self::Msg) -> crate::tea::Command<Self::Msg> {
            match msg {
                TestMsg::Increment => self.count += 1,
                TestMsg::Decrement => self.count -= 1,
            }
            crate::tea::Command::None
        }

        fn state(&self) -> Self::State {
            self.count
        }
    }

    #[test]
    fn test_add_tea_model() {
        let dispatcher = Arc::new(UnifiedDispatcher::new());
        let container = StateContainer::new(dispatcher);

        let handle = container.add_tea(TestModel::init().0);
        assert_eq!(handle.state(), 0);
    }

    #[test]
    fn test_tea_dispatch() {
        let dispatcher = Arc::new(UnifiedDispatcher::new());
        let container = StateContainer::new(dispatcher);

        let handle = container.add_tea(TestModel::init().0);
        handle.dispatch(TestMsg::Increment);
        assert_eq!(handle.state(), 1);

        handle.dispatch(TestMsg::Increment);
        assert_eq!(handle.state(), 2);

        handle.dispatch(TestMsg::Decrement);
        assert_eq!(handle.state(), 1);
    }
}
