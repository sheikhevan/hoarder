extern crate exif;

pub mod rename_image {
    use exif::{DateTime, In, Reader, Tag, Value};
    use std::fs::{self, File};
    use std::io::{BufReader, Error, ErrorKind, Result};
    use std::path::Path;

    pub fn single(path: Vec<&std::string::String>) -> Result<()> {
        for i in path.iter() {
            let file = File::open(i).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to open file {}: {}", i, e),
                )
            })?;
            let exif = Reader::new()
                .read_from_container(&mut BufReader::new(&file))
                .map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to read EXIF data from {}: {}", i, e),
                    )
                })?;

            if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
                match field.value {
                    Value::Ascii(ref vec) if !vec.is_empty() => {
                        let datetime = DateTime::from_ascii(&vec[0]).map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Failed to parse DateTime for {}: {}", i, e),
                            )
                        })?;

                        let year = datetime.year;
                        let month = format!("{:02}", datetime.month);
                        let day = format!("{:02}", datetime.day);

                        let new_filename = format!("IMG_{year}-{month}-{day}.jpg");

                        let mut counter = 1;
                        let mut unique_path = new_filename.clone();
                        while Path::new(&unique_path).exists() {
                            unique_path = format!("IMG_{year}-{month}-{day}_{}.jpg", counter);
                            counter += 1;
                        }

                        fs::rename(i, &unique_path).map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Failed to rename {} to {}: {}", i, unique_path, e),
                            )
                        })?;
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Invalid DateTime format in EXIF data for {}", i),
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("No DateTime field found in EXIF data for {}", i),
                ));
            }
        }
        Ok(())
    }

    pub fn directory(path: Vec<&std::string::String>) -> Result<()> {
        for i in path.iter() {
            let file = File::open(i).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to open file {}: {}", i, e),
                )
            })?;
            let exif = Reader::new()
                .read_from_container(&mut BufReader::new(&file))
                .map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to read EXIF data from {}: {}", i, e),
                    )
                })?;

            if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
                match field.value {
                    Value::Ascii(ref vec) if !vec.is_empty() => {
                        let datetime = DateTime::from_ascii(&vec[0]).map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Failed to parse DateTime for {}: {}", i, e),
                            )
                        })?;

                        let year = datetime.year;
                        let month = format!("{:02}", datetime.month);
                        let day = format!("{:02}", datetime.day);

                        let year_dir = format!("./{}", year);
                        fs::create_dir_all(&year_dir).map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Failed to create directory {}: {}", year_dir, e),
                            )
                        })?;

                        let new_filename = format!("IMG_{year}-{month}-{day}.jpg");
                        let final_path = format!("{}/{}", year_dir, new_filename);

                        let mut counter = 1;
                        let mut unique_path = final_path.clone();
                        while Path::new(&unique_path).exists() {
                            unique_path =
                                format!("{}/IMG_{year}-{month}-{day}_{}.jpg", year_dir, counter);
                            counter += 1;
                        }

                        fs::rename(i, &unique_path).map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Failed to rename {} to {}: {}", i, unique_path, e),
                            )
                        })?;
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Invalid DateTime format in EXIF data for {}", i),
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("No DateTime field found in EXIF data for {}", i),
                ));
            }
        }
        Ok(())
    }
}
