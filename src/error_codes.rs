#[derive(Clone)]
pub enum VcfErrorCode {
    Ok,
    FileNotFound(String),
    FileReadError(String, String),
    IncorrectHeader(String),
    IncorrectEntriesNumber(String),
    EmptyVcfEntry(String),
    EmptyInfoEntry(String),
    IncorrectInfoEntry(String),
    EmptyFormatEntry(String),
    EmptySampleEntry(String),
    IncorrectSampleEntriesNumber(String),
}

impl VcfErrorCode {
    pub fn error_message(&self) -> String {
        match self {
            VcfErrorCode::Ok => "OK".to_string(),
            VcfErrorCode::FileNotFound(filename) => format!("File not found: {}", filename),
            VcfErrorCode::FileReadError(filename, message) => format!("File reading Cannot read file {}: {}", filename, message),
            VcfErrorCode::IncorrectEntriesNumber(line) => format!("Incorrect number of entries: {}", line),
            VcfErrorCode::IncorrectHeader(line) => format!("Incorrect VCF column header line: {}", line),
            VcfErrorCode::EmptyVcfEntry(entry) => format!("Empty VCF entry: {}", entry),
            VcfErrorCode::EmptyInfoEntry(entry) => format!("Empty INFO entry: {}", entry),
            VcfErrorCode::IncorrectInfoEntry(entry) => format!("Incorrectly formatted INFO entry: {}", entry),
            VcfErrorCode::EmptyFormatEntry(entry) => format!("Empty FORMAT entry: {}", entry),
            VcfErrorCode::EmptySampleEntry(entry) => format!("Empty SAMPLE entry: {}", entry),
            VcfErrorCode::IncorrectSampleEntriesNumber(entry) => format!("Incorrect number of FORMAT entries: {}", entry),
        }
    }
    pub fn error_code(&self) -> i32 {
        match self {
            VcfErrorCode::Ok => 0,
            VcfErrorCode::FileNotFound(_) => 1,
            VcfErrorCode::FileReadError(_, _) => 2,
            VcfErrorCode::IncorrectHeader(_) => 3,
            VcfErrorCode::IncorrectEntriesNumber(_) => 4,
            VcfErrorCode::EmptyVcfEntry(_) => 5,
            VcfErrorCode::EmptyInfoEntry(_) => 6,
            VcfErrorCode::IncorrectInfoEntry(_) => 7,
            VcfErrorCode::EmptyFormatEntry(_) => 8,
            VcfErrorCode::EmptySampleEntry(_) => 9,
            VcfErrorCode::IncorrectSampleEntriesNumber(_) => 10,
        }
    }
}