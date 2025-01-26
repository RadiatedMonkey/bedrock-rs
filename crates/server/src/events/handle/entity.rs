use shipyard::EntityId;

pub struct EntityHandle(EntityId);

impl EntityHandle {
    pub fn id(&self) -> EntityId {
        self.0
    }
}
