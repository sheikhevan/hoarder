extern crate exif;

pub mod rename_image {

    use exif::{DateTime, In, Reader, Tag, Value};
    use std::fs::File;
    use std::io::{BufReader, Result};

    pub fn single(path: Vec<&std::string::String>) -> Result<()> {
        for i in path.iter() {
            let file = File::open(i).unwrap();
            let exif = Reader::new()
                .read_from_container(&mut BufReader::new(&file))
                .unwrap();
            if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
                match field.value {
                    Value::Ascii(ref vec) if !vec.is_empty() => {
                        if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                            let year = datetime.year;
                            let month = if datetime.month < 10 {
                                format!("0{}", datetime.month)
                            } else {
                                format!("{}", datetime.month)
                            };
                            let day = if datetime.day < 10 {
                                format!("0{}", datetime.day)
                            } else {
                                format!("{}", datetime.day)
                            };

                            let final_name = format!("IMG_{year}-{month}-{day}.jpg");
                            println!("Renaming {i} to {final_name}");
                            std::fs::rename(i, final_name)?;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    pub fn directory(path: Vec<&std::string::String>) -> std::io::Result<()> {
        for i in path.iter() {
            let file = File::open(i).unwrap();
            let exif = Reader::new()
                .read_from_container(&mut BufReader::new(&file))
                .unwrap();
            if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
                match field.value {
                    Value::Ascii(ref vec) if !vec.is_empty() => {
                        if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                            let year = datetime.year;
                            let month = if datetime.month < 10 {
                                format!("0{}", datetime.month)
                            } else {
                                format!("{}", datetime.month)
                            };
                            let day = if datetime.day < 10 {
                                format!("0{}", datetime.day)
                            } else {
                                format!("{}", datetime.day)
                            };

                            let final_name = format!("{year}/IMG_{year}-{month}-{day}.jpg");
                            println!("Renaming {i} to {final_name}");
                            std::fs::rename(i, final_name)?;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
