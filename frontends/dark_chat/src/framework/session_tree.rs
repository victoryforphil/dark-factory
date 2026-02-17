use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionTreeRow {
    pub session_id: String,
    pub title: String,
    pub status: String,
    pub is_active: bool,
    pub depth: usize,
    pub is_last: bool,
    pub ancestors_are_last: Vec<bool>,
    pub child_count: usize,
    pub created_at: Option<String>,
}

pub trait SessionLike {
    fn id(&self) -> &str;
    fn parent_id(&self) -> Option<&str>;
    fn title(&self) -> &str;
    fn status(&self) -> &str;
    fn created_at(&self) -> Option<&str>;
}

pub fn walk_session_tree<T: SessionLike>(
    sessions: &[T],
    active_session_id: Option<&str>,
) -> Vec<SessionTreeRow> {
    if sessions.is_empty() {
        return Vec::new();
    }

    let mut index_by_id = HashMap::new();
    for (index, session) in sessions.iter().enumerate() {
        index_by_id.insert(session.id().to_string(), index);
    }

    let mut children_by_parent: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut roots = Vec::new();
    for (index, session) in sessions.iter().enumerate() {
        let parent_index = session
            .parent_id()
            .and_then(|id| index_by_id.get(id).copied())
            .filter(|parent| *parent != index);

        if let Some(parent) = parent_index {
            children_by_parent.entry(parent).or_default().push(index);
        } else {
            roots.push(index);
        }
    }

    let mut rows = Vec::with_capacity(sessions.len());
    let mut visited = HashSet::new();

    for (position, root) in roots.iter().copied().enumerate() {
        let is_last = position + 1 == roots.len();
        walk(
            sessions,
            root,
            0,
            &[],
            is_last,
            active_session_id,
            &children_by_parent,
            &mut visited,
            &mut rows,
        );
    }

    for index in 0..sessions.len() {
        if visited.contains(&index) {
            continue;
        }

        walk(
            sessions,
            index,
            0,
            &[],
            true,
            active_session_id,
            &children_by_parent,
            &mut visited,
            &mut rows,
        );
    }

    rows
}

fn walk<T: SessionLike>(
    sessions: &[T],
    index: usize,
    depth: usize,
    ancestors_are_last: &[bool],
    is_last: bool,
    active_session_id: Option<&str>,
    children_by_parent: &HashMap<usize, Vec<usize>>,
    visited: &mut HashSet<usize>,
    rows: &mut Vec<SessionTreeRow>,
) {
    if !visited.insert(index) {
        return;
    }

    let session = &sessions[index];
    let child_count = children_by_parent.get(&index).map(Vec::len).unwrap_or(0);

    rows.push(SessionTreeRow {
        session_id: session.id().to_string(),
        title: session.title().to_string(),
        status: session.status().to_string(),
        is_active: active_session_id.is_some_and(|id| id == session.id()),
        depth,
        is_last,
        ancestors_are_last: ancestors_are_last.to_vec(),
        child_count,
        created_at: session.created_at().map(ToString::to_string),
    });

    let Some(children) = children_by_parent.get(&index) else {
        return;
    };

    for (position, child) in children.iter().copied().enumerate() {
        let mut next_ancestors = ancestors_are_last.to_vec();
        next_ancestors.push(is_last);
        walk(
            sessions,
            child,
            depth + 1,
            &next_ancestors,
            position + 1 == children.len(),
            active_session_id,
            children_by_parent,
            visited,
            rows,
        );
    }
}

pub fn tree_prefix(depth: usize, is_last: bool, ancestors_are_last: &[bool]) -> String {
    let mut prefix = String::new();

    for ancestor_is_last in ancestors_are_last.iter().take(depth.saturating_sub(1)) {
        if *ancestor_is_last {
            prefix.push_str("   ");
        } else {
            prefix.push_str("|  ");
        }
    }

    if depth == 0 {
        prefix.push_str("o ");
        return prefix;
    }

    if is_last {
        prefix.push_str("\\- ");
    } else {
        prefix.push_str("|- ");
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::{SessionLike, tree_prefix, walk_session_tree};

    #[derive(Clone)]
    struct Session {
        id: &'static str,
        parent_id: Option<&'static str>,
        title: &'static str,
        status: &'static str,
    }

    impl SessionLike for Session {
        fn id(&self) -> &str {
            self.id
        }

        fn parent_id(&self) -> Option<&str> {
            self.parent_id
        }

        fn title(&self) -> &str {
            self.title
        }

        fn status(&self) -> &str {
            self.status
        }

        fn created_at(&self) -> Option<&str> {
            None
        }
    }

    #[test]
    fn walk_session_tree_builds_parent_child_rows() {
        let sessions = vec![
            Session {
                id: "root",
                parent_id: None,
                title: "Root",
                status: "idle",
            },
            Session {
                id: "child-a",
                parent_id: Some("root"),
                title: "A",
                status: "idle",
            },
            Session {
                id: "child-b",
                parent_id: Some("root"),
                title: "B",
                status: "running",
            },
        ];

        let rows = walk_session_tree(&sessions, Some("child-b"));

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].session_id, "root");
        assert_eq!(rows[1].depth, 1);
        assert_eq!(rows[2].depth, 1);
        assert!(rows[2].is_active);
        assert_eq!(rows[0].child_count, 2);
    }

    #[test]
    fn tree_prefix_uses_ascii_connectors() {
        assert_eq!(tree_prefix(0, true, &[]), "o ");
        assert_eq!(tree_prefix(1, false, &[]), "|- ");
        assert_eq!(tree_prefix(1, true, &[]), "\\- ");
        assert_eq!(tree_prefix(2, false, &[false]), "|  |- ");
    }
}
