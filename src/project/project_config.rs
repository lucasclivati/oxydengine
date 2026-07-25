use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub path: String,
    pub created_at: String,
    pub engine_version: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsHistory {
    pub default_projects_dir: String,
    pub recent_projects: Vec<ProjectConfig>,
}

impl ProjectsHistory {
    pub fn load() -> Self {
        let root = get_projects_root_dir();
        let recents = list_available_projects();
        Self {
            default_projects_dir: root,
            recent_projects: recents,
        }
    }

    pub fn add_project(&mut self, proj: ProjectConfig) {
        if !self.recent_projects.iter().any(|p| p.path == proj.path) {
            self.recent_projects.push(proj.clone());
        }
        save_project_to_history(&proj.path);
    }

    pub fn set_default_dir(&mut self, dir: &str) {
        self.default_projects_dir = dir.to_string();
    }
}

impl ProjectConfig {
    pub fn new(name: &str, path: &str) -> Self {
        let abs_path = normalize_path(path);

        let thumb = Path::new(&abs_path).join("thumbnail.jpg");
        let thumbnail_path = if thumb.exists() {
            Some(normalize_path(&thumb.to_string_lossy()))
        } else {
            Some("logo.jpg".to_string())
        };

        Self {
            name: name.to_string(),
            path: abs_path,
            created_at: "2026-07-24".to_string(),
            engine_version: "0.0.1".to_string(),
            thumbnail_path,
        }
    }

    pub fn project_file_path(&self) -> PathBuf {
        Path::new(&self.path).join("project.oxyd")
    }

    pub fn load_from_dir(path: &str) -> std::io::Result<Self> {
        let norm_path = normalize_path(path);
        let p_file = Path::new(&norm_path).join("project.oxyd");

        if p_file.exists() {
            let content = fs::read_to_string(p_file)?;
            if let Ok(config) = serde_json::from_str::<ProjectConfig>(&content) {
                return Ok(config);
            }
        }

        let name = Path::new(&norm_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Oxyd_Project".to_string());

        let config = Self::new(&name, &norm_path);
        let _ = config.save();
        Ok(config)
    }

    pub fn save(&self) -> std::io::Result<()> {
        let dir = Path::new(&self.path);
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let thumb_target = dir.join("thumbnail.jpg");
        if !thumb_target.exists() && Path::new("logo.jpg").exists() {
            let _ = fs::copy("logo.jpg", &thumb_target);
        }

        let file_path = self.project_file_path();
        let content = serde_json::to_string_pretty(self)?;
        fs::write(file_path, content)?;

        let content_dir = dir.join("Content");
        let subfolders = ["Actors", "Maps", "Materials", "Meshes", "Textures", "Decals", "VFX"];
        for folder in subfolders {
            let _ = fs::create_dir_all(content_dir.join(folder));
        }

        let maps_dir = content_dir.join("Maps");
        let map_files = ["Map_MainMenu.oxydlevel", "Map_Lobby.oxydlevel", "Map_Transition.oxydlevel", "Map_CityZombieSurvival.oxydlevel"];
        for map_file in map_files {
            let p = maps_dir.join(map_file);
            if !p.exists() {
                let _ = fs::write(p, format!("{{\"level_name\": \"{}\"}}", map_file));
            }
        }

        Ok(())
    }
}

pub fn get_projects_root_dir() -> String {
    let mut root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    root.push("projects");
    normalize_path(&root.to_string_lossy())
}

pub fn list_available_projects() -> Vec<ProjectConfig> {
    let mut projects = Vec::new();
    let root = get_projects_root_dir();
    let root_path = Path::new(&root);

    if !root_path.exists() {
        let _ = fs::create_dir_all(root_path);
    }

    let default_proj_dir = root_path.join("AlchemySurvival57old");
    if !default_proj_dir.exists() {
        let proj = ProjectConfig::new("AlchemySurvival57old", &normalize_path(&default_proj_dir.to_string_lossy()));
        let _ = proj.save();
    }

    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let proj_file = path.join("project.oxyd");
                if proj_file.exists() {
                    if let Ok(content) = fs::read_to_string(&proj_file) {
                        if let Ok(config) = serde_json::from_str::<ProjectConfig>(&content) {
                            projects.push(config);
                        }
                    }
                } else {
                    let folder_name = path.file_name().unwrap().to_string_lossy();
                    let config = ProjectConfig::new(&folder_name, &normalize_path(&path.to_string_lossy()));
                    let _ = config.save();
                    projects.push(config);
                }
            }
        }
    }

    let history_file = Path::new("projects_history.json");
    if history_file.exists() {
        if let Ok(content) = fs::read_to_string(history_file) {
            if let Ok(hist_paths) = serde_json::from_str::<Vec<String>>(&content) {
                for hp in hist_paths {
                    let path = Path::new(&hp);
                    if path.exists() && !projects.iter().any(|p| p.path == hp) {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        let config = ProjectConfig::new(&name, &hp);
                        projects.push(config);
                    }
                }
            }
        }
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));
    projects
}

pub fn save_project_to_history(path: &str) {
    let norm = normalize_path(path);
    let history_file = Path::new("projects_history.json");
    let mut paths: Vec<String> = Vec::new();

    if history_file.exists() {
        if let Ok(content) = fs::read_to_string(history_file) {
            if let Ok(p) = serde_json::from_str::<Vec<String>>(&content) {
                paths = p;
            }
        }
    }

    if !paths.contains(&norm) {
        paths.push(norm);
        if let Ok(content) = serde_json::to_string_pretty(&paths) {
            let _ = fs::write(history_file, content);
        }
    }
}

pub fn normalize_path(path: &str) -> String {
    let path_buf = PathBuf::from(path);
    if let Ok(canonical) = fs::canonicalize(&path_buf) {
        let mut s = canonical.to_string_lossy().to_string();
        if s.starts_with(r"\\?\") {
            s = s[4..].to_string();
        }
        s.replace('\\', "/")
    } else {
        path.replace('\\', "/")
    }
}
