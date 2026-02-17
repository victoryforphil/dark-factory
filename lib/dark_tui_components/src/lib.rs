//! Shared Ratatui components, theme contracts, and layout helpers.
//!
//! This crate provides reusable UI primitives consumed by `dark_chat` and
//! `dark_tui` so frontend crates can keep app state local while sharing
//! rendering building blocks.

pub mod action;
pub mod component;
pub mod components;
pub mod event;
pub mod theme;
pub mod utils;

pub use action::Action;
pub use component::{Component, ComponentResult, DynComponent};

pub use components::{
    CardGridComponent, ChatComposerComponent, ChatComposerProps, ChatConversationHeaderComponent,
    ChatConversationHeaderProps, ChatMessageEntry, ChatMessageListComponent, ChatMessageListProps,
    ChatMessageRole, ChatPalette, ChatStatusTone, FooterBar, FooterBarProps, KeyBind, KeyHintBar,
    LabeledField, LoadingSpinner, PaneBlockComponent, PopupAnchor, PopupHit, PopupItem,
    PopupOverlay, PopupOverlayProps, SectionHeader, StatusPill,
};
pub use event::Event;
pub use theme::{ComponentTheme, ComponentThemeLike};
pub use utils::compact::*;
pub use utils::index::*;
pub use utils::rect::*;
pub use utils::resizable::*;
pub use utils::split_layout::*;
pub use utils::viewport::*;
