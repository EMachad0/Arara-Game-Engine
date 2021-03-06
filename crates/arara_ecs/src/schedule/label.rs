use arara_utils::define_label;
pub use ecs_macros::{AmbiguitySetLabel, RunCriteriaLabel, StageLabel, SystemLabel};

define_label!(StageLabel);
define_label!(SystemLabel);
define_label!(AmbiguitySetLabel);
define_label!(RunCriteriaLabel);

pub(crate) type BoxedStageLabel = Box<dyn StageLabel>;
pub(crate) type BoxedSystemLabel = Box<dyn SystemLabel>;
pub(crate) type BoxedAmbiguitySetLabel = Box<dyn AmbiguitySetLabel>;
pub(crate) type BoxedRunCriteriaLabel = Box<dyn RunCriteriaLabel>;
