use crate::history::Entry;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Recency,
    Alpha,
    Length,
}

impl SortMode {
    pub fn cycle(self) -> Self {
        match self {
            Self::Recency => Self::Alpha,
            Self::Alpha => Self::Length,
            Self::Length => Self::Recency,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Recency => "Recency",
            Self::Alpha => "Alpha",
            Self::Length => "Length",
        }
    }
}

pub fn apply_sort(indices: &mut [usize], entries: &[Entry], mode: SortMode, reverse: bool) {
    match mode {
        SortMode::Recency => {
            indices.sort_unstable_by(|a, b| {
                let ord = a.cmp(b);
                if reverse {
                    ord.reverse()
                } else {
                    ord
                }
            });
        }
        SortMode::Alpha => {
            indices.sort_unstable_by(|a, b| {
                let ord = cmp_ignore_ascii_case(&entries[*a].cmd, &entries[*b].cmd)
                    .then_with(|| a.cmp(b));
                if reverse {
                    ord.reverse()
                } else {
                    ord
                }
            });
        }
        SortMode::Length => {
            indices.sort_unstable_by(|a, b| {
                let ord = entries[*a]
                    .cmd
                    .len()
                    .cmp(&entries[*b].cmd.len())
                    .then_with(|| a.cmp(b));
                if reverse {
                    ord.reverse()
                } else {
                    ord
                }
            });
        }
    }
}

fn cmp_ignore_ascii_case(a: &str, b: &str) -> Ordering {
    for (ca, cb) in a.bytes().zip(b.bytes()) {
        let ord = ca.to_ascii_lowercase().cmp(&cb.to_ascii_lowercase());
        if ord != Ordering::Equal {
            return ord;
        }
    }

    a.len().cmp(&b.len()).then_with(|| a.cmp(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<Entry> {
        vec![
            Entry {
                id: 0,
                cmd: "git status".to_string(),
            },
            Entry {
                id: 1,
                cmd: "ls".to_string(),
            },
            Entry {
                id: 2,
                cmd: "Cargo test".to_string(),
            },
        ]
    }

    #[test]
    fn sorts_by_recency_default_order() {
        let entries = sample_entries();
        let mut idx = vec![2, 1, 0];
        apply_sort(&mut idx, &entries, SortMode::Recency, false);
        assert_eq!(idx, vec![0, 1, 2]);
    }

    #[test]
    fn sorts_alpha_case_insensitive() {
        let entries = sample_entries();
        let mut idx = vec![0, 1, 2];
        apply_sort(&mut idx, &entries, SortMode::Alpha, false);
        assert_eq!(idx, vec![2, 0, 1]);
    }

    #[test]
    fn sorts_by_length_reverse() {
        let entries = sample_entries();
        let mut idx = vec![0, 1, 2];
        apply_sort(&mut idx, &entries, SortMode::Length, true);
        assert_eq!(idx, vec![2, 0, 1]);
    }
}
