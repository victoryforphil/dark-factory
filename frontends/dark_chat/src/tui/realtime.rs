use crate::core::ChatRealtimeEvent;

pub fn event_requires_refresh(event: &ChatRealtimeEvent, active_session_id: Option<&str>) -> bool {
    if let Some(session_id) = event.session_id.as_deref() {
        if Some(session_id) != active_session_id {
            return false;
        }
    }

    event.event_type.starts_with("message.")
        || event.event_type.starts_with("session.")
        || event.event_type.starts_with("permission.")
        || event.event_type.starts_with("question.")
        || event.event_type == "server.connected"
}
