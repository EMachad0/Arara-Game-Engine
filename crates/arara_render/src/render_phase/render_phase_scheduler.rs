use std::any::TypeId;

use arara_ecs::world::World;
use arara_utils::HashMap;

use crate::{DrawFunctions, PhaseItem, RenderPhase, TrackedFrame};

arara_utils::define_label!(RenderPhaseLabel);

#[derive(Default)]
pub struct RenderPhases {
    functions: HashMap<TypeId, Box<fn(&World, &mut TrackedFrame)>>,
    order: Vec<TypeId>,
}

impl RenderPhases {
    pub fn add<I: PhaseItem>(&mut self) -> &mut Self {
        self.order.push(TypeId::of::<I>());
        self.functions
            .insert(TypeId::of::<I>(), Box::new(draw_render_phase::<I>));
        self
    }

    pub fn add_before<Target: PhaseItem, I: PhaseItem>(&mut self) -> &mut Self {
        let target_index = self
            .order
            .iter()
            .enumerate()
            .find(|(_i, ty)| **ty == TypeId::of::<Target>())
            .map(|(i, _)| i)
            .unwrap_or_else(|| {
                panic!(
                    "Render Phase does not exist: {}.",
                    std::any::type_name::<Target>()
                )
            });
        self.order.insert(target_index, TypeId::of::<I>());
        self.functions
            .insert(TypeId::of::<I>(), Box::new(draw_render_phase::<I>));
        self
    }

    pub fn add_after<Target: PhaseItem, I: PhaseItem>(&mut self) -> &mut Self {
        let target_index = self
            .order
            .iter()
            .enumerate()
            .find(|(_i, ty)| **ty == TypeId::of::<Target>())
            .map(|(i, _)| i)
            .unwrap_or_else(|| {
                panic!(
                    "Render Phase does not exist: {}.",
                    std::any::type_name::<Target>()
                )
            });
        self.order.insert(target_index + 1, TypeId::of::<I>());
        self.functions
            .insert(TypeId::of::<I>(), Box::new(draw_render_phase::<I>));
        self
    }

    pub fn run(&self, world: &World, tracked_frame: &mut TrackedFrame) {
        self.order.iter().for_each(|id| {
            let f = self.functions.get(id).unwrap();
            f(world, tracked_frame);
        });
    }
}

pub fn draw_render_phase<I: PhaseItem>(world: &World, tracked_frame: &mut TrackedFrame) {
    let draw_functions = world.get_resource::<DrawFunctions<I>>().unwrap();
    let phase = world.get_resource::<RenderPhase<I>>().unwrap();
    let mut draw_functions = draw_functions.write();
    for item in phase.items.iter() {
        let draw_function = draw_functions.get_mut(item.draw_function()).unwrap();
        draw_function.draw(world, tracked_frame, item);
    }
}
