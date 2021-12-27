use arara_ecs::prelude::Component;

#[derive(Component)]
pub struct Visibility {
    pub active: bool,
    pub visible: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::active()
    }
}

impl Visibility {
    pub fn active() -> Self {
        Self {
            active: true,
            visible: true,
        }
    }

    pub fn inactive() -> Self {
        Self {
            active: false,
            visible: true,
        }
    }
}
