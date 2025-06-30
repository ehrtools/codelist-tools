//! This file contains the logging information for a codelist. The aim of this module
//! is to provide a structured way to log events related to the codelist operations. We
//! want to be able to use the logs to be recreate the codelist at a later time, when
//! given the same input data.
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};


/// Represents the type of action that was logged in the codelist.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogType {
    Add(AddType),
    Edit(EditType),
    Remove(RemoveType),
    Save,
    Note,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AddType {
    Code,
    Metadata,
    Comment
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EditType {
    Term,
    Comment,
    Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RemoveType {
    Code,
    Comment,
    Term,
}


/// Represents a single log entry in the codelist log.
///
/// Fields:
/// - `timestamp`: The time when the log entry was created, in RFC 3339 format.
/// - `action_type`: The type of action that was logged (e.g., adding a code, removing a code).
/// /// - `log`: A message describing the action that was logged.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: String,
    pub action_type: LogType,
    pub log: String,
}

impl LogEntry {
    /// Create a new log entry with the current timestamp, action type, and log message.
    pub fn new(action_type: LogType, log: String) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        LogEntry {
            timestamp,
            action_type,
            log,
        }
    }

    /// Edit the log message of the entry.
    pub fn edit_log(&mut self, new_log: String) {
        self.log = new_log;
    }

    /// Edit the action type of the entry.
    pub fn edit_action_type(&mut self, new_action_type: LogType) {
        self.action_type = new_action_type;
    }
}

/// Contains the log entries for a codelist.
///
/// Fields:
/// - `entries`: A vector of `LogEntry` objects representing the logged actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodelistLog {
    pub entries: Vec<LogEntry>,
}

impl Default for CodelistLog {
    fn default() -> Self {
        CodelistLog::new()
    }
}

impl CodelistLog {
    /// Create a new, empty codelist log.
    pub fn new() -> Self {
        CodelistLog {
            entries: Vec::new(),
        }
    }

    /// Add a new log entry to the codelist log.
    pub fn add_entry(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }

    /// Filter log entries by action type.
    pub fn filter_by_type(&self, action_type: LogType) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.action_type == action_type)
            .collect()
    }

    /// Get length of the log entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Restart the log, clearing all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Write log to file in text or JSON format.
    pub fn write_to_file(&self, file_path: &str) -> std::io::Result<()> {

        // Get end of the file path so match on the type of file
        let format = match file_path.rsplit('.').next() {
            Some(ext) => ext,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "File path must have an extension")),
        };
        let mut file = File::create(file_path)?;

        match format {
            "json" => {
                let json = serde_json::to_string_pretty(self)?;
                file.write_all(json.as_bytes())?;
            }
            "txt" => {
                for entry in &self.entries {
                    writeln!(file, "{} - {:?}: {}", entry.timestamp, entry.action_type, entry.log)?;
                }
            }
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported format: must be JSON or CSV")),
        }

        Ok(())
    }

}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_codelist_log_add_entry() {
        let mut log = CodelistLog::new();
        let entry = LogEntry::new(LogType::Add(AddType::Code), "Added code 123".to_string());
        log.add_entry(entry);
        assert_eq!(log.len(), 1);
    }
}
