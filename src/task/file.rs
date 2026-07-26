use directories::ProjectDirs;
use serde_json;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

use crate::task::Task;

pub struct TaskStore;

impl TaskStore {
    /// The name of our app
    pub const APP_NAME: &'static str = "todo-rs";

    /// The exact name of the JSON file with our tasks
    pub const TASKS_FILE_NAME: &'static str = "tasks.json";

    /// Gets the path to our tasks file
    ///
    /// It uses directories::ProjectDirs, or CWD if `$HOME` is unset
    fn get_path() -> PathBuf {
        if let Some(project_dirs) = ProjectDirs::from("", "", Self::APP_NAME) {
            let share_dir = project_dirs.data_dir();
            let _ = fs::create_dir_all(share_dir);

            share_dir.join(Self::TASKS_FILE_NAME)
        } else {
            // Use CWD if `$HOME` is unset
            PathBuf::from(Self::TASKS_FILE_NAME)
        }
    }

    /// Load tasks from json.
    ///
    /// # Errors
    /// Returns an error if the file doesn't exist
    pub fn load() -> io::Result<Vec<Task>> {
        let path = Self::get_path();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let tasks = serde_json::from_reader(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(tasks)
    }

    /// Save tasks to json file.
    pub fn save(tasks: &[Task]) -> io::Result<()> {
        let path = Self::get_path();
        let tmp_path = path.with_extension("tmp");

        let file = File::create(&tmp_path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, tasks)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Rename it to the actual path
        fs::rename(tmp_path, path)?;

        Ok(())
    }
}
