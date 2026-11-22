/// The structure box a piece's `bounding_box` answers, in world blocks.
pub use foundry_core::bounding_box::BoundingBox;
pub use foundry_core::equipment_slot::EquipmentSlot;
pub use foundry_core::identifier::Identifier;
/// A level's key (`LevelDimension::Custom`).
pub use foundry_core::level_key::LevelKey;
pub use foundry_core::nbt::Compound;
pub use foundry_core::nbt::Tag;
pub use foundry_core::use_animation::UseAnimation;
pub use {
    foundry_core::{
        connection_kind::ConnectionKind, dimension::Dimension as LevelDimension,
        neighbour_update::NeighbourUpdate,
    },
    foundry_mod_runtime::attachments::{
        AttachmentCodec, AttachmentError, AttachmentForm, AttachmentPolicy, AttachmentStore,
        AttachmentType, AttachmentValue, AttachmentWrite, Holder,
    },
};
// hidden: the game's maps behind the store, not part of the mod surface
#[doc(hidden)]
pub use foundry_mod_runtime::attachments::{AttachmentAccess, AttachmentHost, Attachments};
