use include_dir::{include_dir, Dir};
use std::io::Read;
use thiserror::Error;
use std::str::FromStr;

static INPUT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/inputs");

#[derive(Error, Debug)]
pub enum InputFileError {
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("General Error: {0}")]
    GeneralError(String),
    #[error("Could not parse int")]
    ParseIntError(#[from] core::num::ParseIntError),
    #[error("Could not find day {0}")]
    CouldNotFindDay(String),
}

pub struct InputFile {
    pub data : String
}

impl InputFile {
    pub fn lines(&self) -> Vec<String> {
        self.data.split("\n")
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(str::to_string)
            .collect()
    }

}

impl<O : FromStr> TryFrom<InputFile> for Vec<O> where InputFileError : From<<O as FromStr>::Err> {
    type Error = InputFileError;

    fn try_from(input_file : InputFile) -> Result<Self, Self::Error> {
        Ok(input_file.lines().into_iter()
            .map(|s| O::from_str(s.as_str()))
            .collect::<Result<Vec<O>, O::Err>>()?)
    }
}

pub fn load_sample<O : TryFrom<InputFile>>(day : usize) -> Result<O, InputFileError> where InputFileError: From<<O as TryFrom<InputFile>>::Error> {
    load_file(format!("day{}_sample", day))
}

pub fn load_input<O : TryFrom<InputFile>>(day : usize) -> Result<O, InputFileError> where InputFileError: From<<O as TryFrom<InputFile>>::Error> {
    load_file(format!("day{}", day))
}

fn load_file<O : TryFrom<InputFile>>(file_name : String) -> Result<O, InputFileError> where InputFileError: From<<O as TryFrom<InputFile>>::Error> {
    match INPUT_DIR.get_file(file_name.clone()) {
        Some(file) => {
            let mut buffer = String::new();
            file.contents().read_to_string(&mut buffer)?;
            Ok(InputFile {
                data: buffer,
            }.try_into()?)
        },
        None => Err(InputFileError::CouldNotFindDay(file_name))
    }
}

