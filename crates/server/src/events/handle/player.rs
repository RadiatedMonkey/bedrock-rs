use shipyard::EntityId;

pub struct PlayerHandle(EntityId);

impl PlayerHandle {
    pub fn id(&self) -> EntityId {
        self.0
    }
    
    
}
