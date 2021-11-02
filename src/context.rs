use std::{any::TypeId, cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc, sync::Arc};

use crate::{CachedData, Data, Entity, Event, IdManager, ModelData, MouseState, State, StateData, StateID, Store, Style, Tree, TreeExt, ViewHandler};

pub struct Context {
    pub entity_manager: IdManager<Entity>,
    pub tree: Tree,
    pub current: Entity,
    pub count: usize,
    pub views: HashMap<Entity, Box<dyn ViewHandler>>,
    pub state: HashMap<StateID, Box<dyn StateData>>,
    pub data: Data,
    pub event_queue: VecDeque<Event>,
    pub style: Rc<RefCell<Style>>,
    pub cache: CachedData,

    pub mouse: MouseState,

    pub hovered: Entity,

    pub state_count: u32,
    //pub data: HashMap<u32, Box<dyn Model>>,
    //pub handlers: HashMap<i32, Box<dyn View>>,
}

impl Context {
    pub fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        for entity in delete_list.iter().rev() {
            println!("Removing: {}", entity);
            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            //self.style.remove(*entity); TODO
            self.entity_manager.destroy(*entity);
        }
    }

    /// Get stored data from the context.
    pub fn data<T: 'static>(&self) -> Option<&T> {
        self.data
            .model_data
            .get(&TypeId::of::<T>())
            .and_then(|model| model.downcast_ref::<Store<T>>())
            .map(|store| &store.data)
    }
}
