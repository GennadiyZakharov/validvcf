#[derive(Copy, Clone)]
pub enum VcfErrorCode {
    FileNotFound = 1,
    FileReadError = 2,
    IncorrectHeader =3 ,
    IncorrectEntriesNumber =4,
    EmptyVcfEntry =5,
    EmptyInfoEntry =6,
    IncorrectInfoEntry = 7,
    EmptyFormatEntry = 8,
    EmptySampleEntry = 9,
    IncorrectSampleEntriesNumber = 10,
}

impl VcfErrorCode {
    pub fn error_message(&self) -> &str {
        match self {
            VcfErrorCode::FileNotFound => "File not found",
            VcfErrorCode::FileReadError => "Cannot read file",
            VcfErrorCode::IncorrectEntriesNumber => "Incorrect number of entries",
            VcfErrorCode::IncorrectHeader => "Incorrect VCF column header line",
            VcfErrorCode::EmptyVcfEntry => "Empty VCF entry",
            VcfErrorCode::EmptyInfoEntry => "Empty INFO entry",
            VcfErrorCode::IncorrectInfoEntry => "Incorrectly formatted INFO entry",
            VcfErrorCode::EmptyFormatEntry => "Empty INFO entry",
            VcfErrorCode::EmptySampleEntry => "Empty SAMPLE entry",
            VcfErrorCode::IncorrectSampleEntriesNumber => "Incorrect number of FORMAT entries",
        }
    }
    pub fn error_code(&self) -> i32 {
       *self as i32
    }
}