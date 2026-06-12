use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FavoriteBlock {
    pub id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub lines: Vec<String>,
}

impl FavoriteBlock {
    pub fn display_title(&self) -> String {
        self.title
            .clone()
            .unwrap_or_else(|| format!("favorite {}", self.id))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FavoriteFile {
    version: u32,
    favorites: Vec<FavoriteBlock>,
}

#[derive(Debug, Clone)]
pub struct FavoritesStore {
    path: Option<PathBuf>,
    pub blocks: Vec<FavoriteBlock>,
}

impl FavoritesStore {
    #[cfg(test)]
    pub fn new_in_memory() -> Self {
        Self {
            path: None,
            blocks: Vec::new(),
        }
    }

    pub fn load_default() -> Result<Self> {
        Self::load_from_path(default_favorites_path())
    }

    pub fn load_from_path(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                path: Some(path),
                blocks: Vec::new(),
            });
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read favorites file: {}", path.display()))?;

        if content.trim().is_empty() {
            return Ok(Self {
                path: Some(path),
                blocks: Vec::new(),
            });
        }

        let parsed: FavoriteFile = serde_json::from_str(&content)
            .with_context(|| format!("failed to parse favorites file: {}", path.display()))?;

        Ok(Self {
            path: Some(path),
            blocks: parsed.favorites,
        })
    }

    pub fn add_block(&mut self, lines: Vec<String>) -> Result<AddFavoriteResult> {
        if lines.is_empty() {
            return Ok(AddFavoriteResult::Empty);
        }

        if let Some(existing) = self.blocks.iter().find(|block| block.lines == lines) {
            return Ok(AddFavoriteResult::Duplicate(existing.id));
        }

        let id = self
            .blocks
            .iter()
            .map(|block| block.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);

        self.blocks.insert(
            0,
            FavoriteBlock {
                id,
                title: None,
                lines,
            },
        );
        self.persist()?;
        Ok(AddFavoriteResult::Added(id))
    }

    pub fn remove_block(&mut self, block_index: usize) -> Result<Option<FavoriteBlock>> {
        if block_index >= self.blocks.len() {
            return Ok(None);
        }

        let removed = self.blocks.remove(block_index);
        self.persist()?;
        Ok(Some(removed))
    }

    pub fn rename_block(&mut self, block_index: usize, title: Option<String>) -> Result<bool> {
        let Some(block) = self.blocks.get_mut(block_index) else {
            return Ok(false);
        };

        block.title = title;
        self.persist()?;
        Ok(true)
    }

    fn persist(&self) -> Result<()> {
        let Some(path) = &self.path else {
            return Ok(());
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create favorites directory: {}", parent.display())
            })?;
        }

        let payload = FavoriteFile {
            version: 1,
            favorites: self.blocks.clone(),
        };
        let json = serde_json::to_string_pretty(&payload).context("failed to encode favorites")?;

        fs::write(path, json)
            .with_context(|| format!("failed to write favorites file: {}", path.display()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddFavoriteResult {
    Added(u64),
    Duplicate(u64),
    Empty,
}

pub fn default_favorites_path() -> PathBuf {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(config_home)
            .join("hline")
            .join("favorites.json");
    }

    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".config")
            .join("hline")
            .join("favorites.json");
    }

    Path::new(".").join(".hline-favorites.json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn add_block_rejects_duplicates() {
        let mut store = FavoritesStore::new_in_memory();
        let lines = vec![
            "npm run db:migrate".to_string(),
            "npx prisma generate".to_string(),
        ];

        assert_eq!(
            store.add_block(lines.clone()).expect("first add"),
            AddFavoriteResult::Added(1)
        );
        assert_eq!(
            store.add_block(lines).expect("duplicate add"),
            AddFavoriteResult::Duplicate(1)
        );
        assert_eq!(store.blocks.len(), 1);
    }

    #[test]
    fn remove_block_deletes_and_persists() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("favorites.json");

        let mut store = FavoritesStore::load_from_path(path.clone()).expect("load empty");
        store
            .add_block(vec!["first".to_string()])
            .expect("add first");
        store
            .add_block(vec!["second".to_string()])
            .expect("add second");

        let removed = store.remove_block(0).expect("remove");
        assert_eq!(
            removed.map(|block| block.lines),
            Some(vec!["second".to_string()])
        );
        assert!(store.remove_block(5).expect("out of range").is_none());

        let loaded = FavoritesStore::load_from_path(path).expect("reload");
        assert_eq!(loaded.blocks.len(), 1);
        assert_eq!(loaded.blocks[0].lines, vec!["first"]);
    }

    #[test]
    fn rename_persists_title_and_loads_old_files_without_one() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("favorites.json");

        let mut store = FavoritesStore::load_from_path(path.clone()).expect("load empty");
        store
            .add_block(vec!["cargo build".to_string()])
            .expect("add");
        assert_eq!(store.blocks[0].display_title(), "favorite 1");

        assert!(store
            .rename_block(0, Some("build stuff".to_string()))
            .expect("rename"));
        assert!(!store.rename_block(5, None).expect("out of range"));

        let loaded = FavoritesStore::load_from_path(path).expect("reload");
        assert_eq!(loaded.blocks[0].title.as_deref(), Some("build stuff"));
        assert_eq!(loaded.blocks[0].display_title(), "build stuff");
    }

    #[test]
    fn persists_and_loads_blocks() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("favorites.json");

        let mut store = FavoritesStore::load_from_path(path.clone()).expect("load empty");
        store
            .add_block(vec!["npm run seed:complete".to_string()])
            .expect("persist add");

        let loaded = FavoritesStore::load_from_path(path).expect("reload");
        assert_eq!(loaded.blocks.len(), 1);
        assert_eq!(loaded.blocks[0].lines, vec!["npm run seed:complete"]);
    }
}
