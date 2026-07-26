use serde_json;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

use crate::task::Task;

pub struct TaskStore;

impl TaskStore {
    /// Load tasks from json. Returns an empty list if the file doesn't exist yet.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Vec<Task>> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Deserializes directly from the file reader stream!
        let tasks = serde_json::from_reader(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(tasks)
    }

    /// Save tasks to json file.
    pub fn save<P: AsRef<Path>>(path: P, tasks: &[Task]) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, tasks)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(())
    }
}
