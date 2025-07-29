use fundsp::wave::Wave;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

/* Constants for directory/file names */
pub const DIR_MODULES: &str = "modules";
pub const DIR_SAMPLES: &str = "samples";
pub const FILE_PROGRAM: &str = "program.luau";

#[derive(Debug)]
pub enum ProjectError {
    BadName(String),
    BadPath(PathBuf),
    BadTemplate,
    NoProgram,
}

#[allow(dead_code)]
impl ProjectError {
    pub fn to_string(&self) -> String {
        match &self {
            ProjectError::BadName(name) => format!("Bad name provided \"{}\"", name),
            ProjectError::BadPath(path) => format!("Failed to use path {:?}", path),
            ProjectError::BadTemplate => format!("Error occured while creating template"),
            ProjectError::NoProgram => format!("Missing program.luau"),
        }
    }
}

pub type ProjectResult = Result<Project, ProjectError>;

pub struct Project {
    /// Name of the Project
    name: String,
    /// Path to the directory the Project is stored within
    path: PathBuf,
    /// Contents of the Project's program.luau file
    program: String,
    /// Vector of all the Project's luau modules
    modules: Vec<String>,
    /// HashMap of all the Project's samples
    samples: HashMap<String, Wave>,
}

#[allow(dead_code)]
impl Project {
    /// Creates a new project at a specified file directory with the
    /// specified name
    pub fn create(path: &PathBuf, name: &String) -> Result<(), ProjectError> {
        // Sanity check name
        if !name
            .chars()
            .all(|x| x.is_alphanumeric() || x == '_' || x == '-')
        {
            return Err(ProjectError::BadName(name.clone()));
        }

        let mut project_path = path.clone();
        // Sanity check path
        if !project_path.exists() {
            return Err(ProjectError::BadPath(project_path));
        }

        project_path.push(name);

        // Create project directory
        if fs::create_dir(&project_path).is_err() {
            return Err(ProjectError::BadPath(project_path));
        }

        // Create sub-directories
        let mut module_path = project_path.clone();
        module_path.push(DIR_MODULES);

        if fs::create_dir(&module_path).is_err() {
            return Err(ProjectError::BadPath(module_path));
        }

        let mut samples_path = project_path.clone();
        samples_path.push(DIR_SAMPLES);

        if fs::create_dir(&samples_path).is_err() {
            return Err(ProjectError::BadPath(samples_path));
        }

        // Create files
        let mut program_path = project_path.clone();
        program_path.push(FILE_PROGRAM);

        let mut program = File::create(&program_path);

        if program.is_err() {
            return Err(ProjectError::BadTemplate);
        }
        // todo: When template is implemented, write contents to program

        Ok(())
    }

    /// If a directory exists, check contents and compile the contents of all
    /// files ending in .luau
    fn get_modules_under_dir(dir_path: &std::path::Path) -> std::io::Result<Vec<String>> {
        let mut modules: Vec<String> = Vec::new();

        if !dir_path.is_dir() {
            println!("Tried to get modules under an invalid path, ignoring...");
            return Ok(modules);
        }

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let sub_modules = Project::get_modules_under_dir(&path);

                if sub_modules.is_ok() {
                    modules.append(&mut sub_modules.unwrap());
                }
            } else {
                let extension = path.extension();

                if extension.is_none() {
                    continue;
                }

                let extension = extension.unwrap().to_str().unwrap_or("");
                if extension != "luau" {
                    continue;
                }

                let contents = fs::read_to_string(path);

                if contents.is_err() {
                    println!("Error reading file");
                    continue;
                }
                modules.push(contents.unwrap());
            }
        }

        return Ok(modules);
    }

    /// If a directory exists, check contents and load all
    /// files ending in .wav
    fn get_samples_under_dir(dir_path: &std::path::Path) -> std::io::Result<HashMap<String, Wave>> {
        let mut samples: HashMap<String, Wave> = HashMap::new();

        if !dir_path.is_dir() {
            println!("Tried to get samples under an invalid path, ignoring...");
            return Ok(samples);
        }

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let sub_samples = Project::get_samples_under_dir(&path);

                if sub_samples.is_ok() {
                    samples.extend(sub_samples.unwrap());
                }
            } else {
                let extension = path.extension();
                let file_name = path.file_stem();

                if extension.is_none() || file_name.is_none() {
                    continue;
                }

                let extension = extension.unwrap().to_str().unwrap_or("");
                let file_name = file_name.unwrap().to_str().unwrap_or("");
                // Ignore .txt, .md files
                // Just try and load any other file type, Wave::load
                // should fail if it is an invalid file
                if extension == "txt" || extension == "md" || extension == "" || file_name == "" {
                    continue;
                }

                let wave = Wave::load(path.clone());

                if wave.is_ok() {
                    let ret = samples.insert(file_name.to_string(), wave.unwrap());

                    if ret.is_some() {
                        println!(
                            "Overwriting sample {}, which was already loaded with {:?}",
                            file_name, path
                        );
                    }
                }
            }
        }

        return Ok(samples);
    }

    /// Loads a project from a specified directory
    pub fn load(path: &PathBuf) -> ProjectResult {
        let file_name = path.file_name();
        if file_name.is_none() {
            return Err(ProjectError::BadName("path.file_name".to_string()));
        }
        let file_name = file_name.unwrap().to_str();
        if file_name.is_none() {
            return Err(ProjectError::BadName("path.file_name".to_string()));
        }

        // User Luau program
        let mut program_path = path.clone();
        program_path.push(FILE_PROGRAM);
        let program_contents = fs::read_to_string(program_path);

        if program_contents.is_err() {
            return Err(ProjectError::NoProgram);
        }

        // Project Luau Modules
        let mut modules_path = path.clone();
        modules_path.push(DIR_MODULES);

        let module_contents: Vec<String> =
            Project::get_modules_under_dir(&modules_path).unwrap_or(Vec::new());
        let mut samples_path = path.clone();
        samples_path.push(DIR_SAMPLES);

        let sample_contents: HashMap<String, Wave> =
            Project::get_samples_under_dir(&samples_path).unwrap_or(HashMap::new());

        Ok(Project {
            name: file_name.unwrap().to_string(),
            path: path.clone(),
            program: program_contents.unwrap(),
            modules: module_contents,
            samples: sample_contents,
        })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_program(&self) -> &String {
        &self.program
    }

    pub fn get_modules(&self) -> &Vec<String> {
        &self.modules
    }

    pub fn get_samples(&self) -> &HashMap<String, Wave> {
        &self.samples
    }
}

#[cfg(test)]
mod tests {
    use super::{DIR_MODULES, DIR_SAMPLES, FILE_PROGRAM};
    use crate::{project::Project, test_utils::make_test_dir};
    use fundsp::wave::Wave;
    use std::io::Write;

    #[test]
    fn test_project_create() {
        // Setup
        let tmp = make_test_dir("project_create");
        assert!(tmp.is_some());
        let tmp = tmp.unwrap();

        // Test
        let mut test;
        let mut name: String;

        // Should be created
        name = "abc123".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), true);

        // Confirm project contents
        let mut test_path = tmp.clone();
        test_path.push("abc123");
        assert!(test_path.exists());

        let mut modules_dir = test_path.clone();
        modules_dir.push(DIR_MODULES);
        assert!(modules_dir.exists());

        let mut samples_dir = test_path.clone();
        samples_dir.push(DIR_SAMPLES);
        assert!(samples_dir.exists());

        let mut program_dir = test_path.clone();
        program_dir.push(FILE_PROGRAM);
        assert!(program_dir.exists());

        // Should also be created
        name = "project-success".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), true);

        name = "project_success".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), true);

        // Shouldn't be created
        name = "project fail".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), false);

        name = "project$fail".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), false);

        name = "project.fail".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), false);

        name = "project/fail".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), false);

        name = "project\\fail".to_string();
        test = Project::create(&tmp, &name);
        assert_eq!(test.is_ok(), false);
    }

    #[test]
    fn test_project_get_modules_under_dir() {
        // Environment Setup
        let tmp = make_test_dir("project_get_modules_under_dir");
        assert!(tmp.is_some());
        let tmp = tmp.unwrap();

        let mut file1 = tmp.clone();
        file1.push("file1.luau");

        let mut dir = tmp.clone();
        dir.push("dir");
        let mut file2 = dir.clone();
        file2.push("file2.luau");

        let mut file1 = std::fs::File::create(file1).unwrap();
        assert!(file1.write_all(b"test").is_ok());

        assert!(std::fs::create_dir(dir).is_ok());
        let mut file2 = std::fs::File::create(file2).unwrap();
        assert!(file2.write_all(b"test").is_ok());

        // Test function
        let modules_vec = Project::get_modules_under_dir(&tmp);

        assert!(modules_vec.is_ok());
        let modules_vec = modules_vec.unwrap();
        assert_eq!(modules_vec.len(), 2);
        assert_eq!(&modules_vec[0], "test");
        assert_eq!(&modules_vec[1], "test");
    }

    #[test]
    fn test_project_get_samples_under_dir() {
        // Environment Setup
        let tmp = make_test_dir("project_get_samples_under_dir");
        assert!(tmp.is_some());
        let tmp = tmp.unwrap();

        let mut test_wave_path = tmp.clone();
        test_wave_path.push("test_wave.wav");

        let mut test_wave = Wave::new(2, 44100.0);

        test_wave.push((0.0, 0.0));
        test_wave.push((0.1, 0.1));
        test_wave.push((0.0, 0.0));

        assert!(test_wave.save_wav16(test_wave_path).is_ok());

        // Test function
        let samples = Project::get_samples_under_dir(&tmp);

        assert!(samples.is_ok());
        let samples = samples.unwrap();
        assert_eq!(samples.len(), 1);
        assert!(samples.get("test_wave").is_some());
    }

    #[test]
    fn test_project_load() {
        // Setup
        let tmp = make_test_dir("project_load");
        assert!(tmp.is_some());
        let tmp = tmp.unwrap();

        let name: String = "winner".to_string();
        assert_eq!(Project::create(&tmp, &name).is_ok(), true);

        // Test Success
        let mut test_path = tmp.clone();
        test_path.push("winner");

        let test = Project::load(&test_path);
        assert_eq!(test.is_ok(), true);
        let test = test.unwrap();
        assert_eq!(test.get_name(), &name);
        assert_eq!(test.get_path(), &test_path);
        // todo: test program contents when lua template is created
        assert_eq!(test.get_modules().len(), 0);

        // Test Failure
        let test = Project::load(&tmp);
        assert_eq!(test.is_err(), true);
    }
}
