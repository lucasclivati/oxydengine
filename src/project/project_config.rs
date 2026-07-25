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

        // Criar estrutura de pastas do projeto Unreal (Content, Blueprints, Maps, Materials, Meshes, Textures, Decals)
        let content_dir = dir.join("Content");
        let subfolders = ["Blueprints", "Maps", "Materials", "Meshes", "Textures", "Decals", "VFX_Niagara"];
        for folder in subfolders {
            let _ = fs::create_dir_all(content_dir.join(folder));
        }

        // Criar arquivos de mapas de exemplo dentro do projeto
        let _ = fs::write(content_dir.join("Maps").join("Map_MainMenu.oxydlevel"), r#"{"level_name": "Map_MainMenu"}"#);
        let _ = fs::write(content_dir.join("Maps").join("Map_Lobby.oxydlevel"), r#"{"level_name": "Map_Lobby"}"#);
        let _ = fs::write(content_dir.join("Maps").join("Map_Transition.oxydlevel"), r#"{"level_name": "Map_Transition"}"#);
        let _ = fs::write(content_dir.join("Maps").join("Map_CityZombieSurvival.oxydlevel"), r#"{"level_name": "Map_CityZombieSurvival"}"#);

        Ok(())
    }

    pub fn load_from_dir(dir_path: &str) -> Result<Self, String> {
        let abs_dir = normalize_path(dir_path);

        let file_path = Path::new(&abs_dir).join("project.oxyd");
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Falha ao ler project.oxyd: {}", e))?;
        let mut config: ProjectConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Falha ao processar project.oxyd: {}", e))?;
        
        let thumb = Path::new(&abs_dir).join("thumbnail.jpg");
        if thumb.exists() {
            config.thumbnail_path = Some(normalize_path(&thumb.to_string_lossy()));
        } else if Path::new("logo.jpg").exists() {
            config.thumbnail_path = Some("logo.jpg".to_string());
        }
        
        config.path = abs_dir;
        Ok(config)
    }
}

fn normalize_path(p: &str) -> String {
    let path = Path::new(p);
    let s = if let Ok(abs) = path.canonicalize() {
        abs.to_string_lossy().to_string()
    } else {
        p.to_string()
    };

    s.replace(r"\\?\", "")
     .replace(r"\\", "/")
     .replace('\\', "/")
     .trim_end_matches('/')
     .to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsHistory {
    pub default_projects_dir: String,
    pub recent_projects: Vec<ProjectConfig>,
}

impl ProjectsHistory {
    pub fn load() -> Self {
        let mut history = if let Ok(content) = fs::read_to_string("projects_history.json") {
            serde_json::from_str::<ProjectsHistory>(&content).unwrap_or_else(|_| Self::default_history())
        } else {
            Self::default_history()
        };

        history.default_projects_dir = normalize_path(&history.default_projects_dir);
        history.deduplicate_and_scan();
        history
    }

    pub fn default_history() -> Self {
        let default_dir = normalize_path("projects");
        
        // APENAS O PROJETO AlchemySurvival57old
        let alchemy_proj = ProjectConfig::new("AlchemySurvival57old", &format!("{}/AlchemySurvival57old", default_dir));
        let _ = alchemy_proj.save();

        let history = Self {
            default_projects_dir: default_dir,
            recent_projects: vec![alchemy_proj],
        };
        history.save();
        history
    }

    pub fn deduplicate_and_scan(&mut self) {
        let default_dir = self.default_projects_dir.clone();
        
        let alchemy_proj = ProjectConfig::new("AlchemySurvival57old", &format!("{}/AlchemySurvival57old", default_dir));
        let _ = alchemy_proj.save();

        let mut unique: Vec<ProjectConfig> = Vec::new();
        // Manter estritamente AlchemySurvival57old
        unique.push(alchemy_proj);

        self.recent_projects = unique;
        self.save();
    }

    pub fn save(&self) {
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write("projects_history.json", content);
        }
    }

    pub fn add_project(&mut self, proj: ProjectConfig) {
        let norm_path = normalize_path(&proj.path).to_lowercase();
        self.recent_projects.retain(|p| normalize_path(&p.path).to_lowercase() != norm_path && p.name != proj.name);
        
        let mut clean_proj = proj;
        clean_proj.path = normalize_path(&clean_proj.path);
        self.recent_projects.insert(0, clean_proj);
        self.save();
    }

    pub fn set_default_dir(&mut self, path: String) {
        self.default_projects_dir = normalize_path(&path);
        self.deduplicate_and_scan();
        self.save();
    }
}
