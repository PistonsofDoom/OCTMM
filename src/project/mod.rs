use std::fs;
use std::fs::File;
use std::path::PathBuf;

/* Constants for directory/file names */
const DIR_MODULES: &str = "modules";
const FILE_PROGRAM: &str = "program.luau";

#[derive(Debug)]
pub enum ProjectError {
    BadName,
    BadPath,
    BadTemplate,
    NoProgram,
}

pub type ProjectResult = Result<Project, ProjectError>;

pub struct Project {
    /// Name of the Project
    name: String,
    /// Path to the directory the Project is stored within
    path: PathBuf,
    /// Contents of the Project's program.luau file
    program: String,
}

#[allow(dead_code)]
impl Project {
    /// Creates a new project at a specified file directory with the
    /// specified name
    pub fn new(path: &PathBuf, name: &String) -> Result<(), ProjectError> {
        // Sanity check name
        if !name.chars().all(char::is_alphanumeric) {
            return Err(ProjectError::BadName);
        }

        let mut project_path = path.clone();
        // Sanity check path
        if !project_path.exists() {
            return Err(ProjectError::BadPath);
        }

        project_path.push(name);

        // Create project directory
        if fs::create_dir(&project_path).is_err() {
            return Err(ProjectError::BadPath);
        }

        // Create sub-directories
        let mut module_path = project_path.clone();
        module_path.push(DIR_MODULES);

        if fs::create_dir(&module_path).is_err() {
            return Err(ProjectError::BadTemplate);
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

    /// Loads a project from a specified directory
    pub fn load(path: &PathBuf) -> ProjectResult {
        let file_name = path.file_name();
        if file_name.is_none() {
            return Err(ProjectError::BadPath);
        }
        let file_name = file_name.unwrap().to_str();
        if file_name.is_none() {
            return Err(ProjectError::BadPath);
        }

        let mut program_path = path.clone();
        program_path.push(FILE_PROGRAM);
        let program_contents = fs::read_to_string(program_path);

        if program_contents.is_err() {
            return Err(ProjectError::NoProgram);
        }

        Ok(Project {
            name: file_name.unwrap().to_string(),
            path: path.clone(),
            program: program_contents.unwrap(),
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
}

#[cfg(test)]
mod tests {
    use super::{DIR_MODULES, FILE_PROGRAM};
    use crate::{project::Project, project::ProjectResult, test_utils::make_test_dir};
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_project_new() {
        // Setup
        let tmp = make_test_dir("project_new");
        assert!(tmp.is_some());
        let tmp = tmp.unwrap();

        // Test
        let mut test;
        let mut name: String;

        // Should be created
        name = "abc123".to_string();
        test = Project::new(&tmp, &name);
        assert_eq!(test.is_ok(), true);

        // Confirm project contents
        let mut test_path = tmp.clone();
        test_path.push("abc123");
        assert!(test_path.exists());

        let mut modules_dir = test_path.clone();
        modules_dir.push(DIR_MODULES);
        assert!(modules_dir.exists());

        let mut program_dir = test_path.clone();
        program_dir.push(FILE_PROGRAM);
        assert!(program_dir.exists());

        // Shouldn't be created
        name = "project$fail".to_string();
        test = Project::new(&tmp, &name);
        assert_eq!(test.is_ok(), false);

        name = "project.fail".to_string();
        test = Project::new(&tmp, &name);
        assert_eq!(test.is_ok(), false);

        name = "project/fail".to_string();
        test = Project::new(&tmp, &name);
        assert_eq!(test.is_ok(), false);

        name = "project\\fail".to_string();
        test = Project::new(&tmp, &name);
        assert_eq!(test.is_ok(), false);
    }

    #[test]
    fn test_project_load() {
        // Setup
        let tmp = make_test_dir("project_load");
        assert!(tmp.is_some());
        let tmp = tmp.unwrap();

        let mut test;
        let mut name: String;

        name = "winner".to_string();
        test = Project::new(&tmp, &name);
        assert_eq!(test.is_ok(), true);

        // Test
        let test = Project::load(&tmp);
        assert_eq!(test.is_err(), true);

        let mut test_path = tmp.clone();
        test_path.push("winner");

        let test = Project::load(&test_path);
        assert_eq!(test.is_ok(), true);
        let test = test.unwrap();
        assert_eq!(test.get_name(), "winner");
        assert_eq!(test.get_path(), &test_path);
        // todo: test program contents when lua template is created
    }
}
