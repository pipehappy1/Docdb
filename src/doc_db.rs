use serde_json::{Value, self};
use std::fs::OpenOptions;
use std::path::{PathBuf};
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use std::io::Error;


pub struct FileDrv {
    path: Rc<RefCell<PathBuf>>,
    fh: Rc<RefCell<File>>,
    header_offset: i64,
}

impl FileDrv {
    pub fn new(db_file: &str) -> FileDrv {
        let mut fp = PathBuf::new();
        fp.push(db_file);

        let file = match OpenOptions::new().read(true).write(true).create(true).open(&fp) {
            Err(why) => panic!("couldn't open {}: {}", db_file, why),
            Ok(file) => file,
        };

        FileDrv {
            path: Rc::new(RefCell::new(fp)),
            fh: Rc::new(RefCell::new(file)),
            header_offset: 1024,
        }
    }

    pub fn reload(&mut self) {
        let db_file = self.path.borrow().clone();
        
        let file = match OpenOptions::new().read(true).write(true).open(db_file.clone()) {
            Err(why) => panic!("couldn't open {}: {}", db_file.into_os_string().into_string().unwrap(), why),
            Ok(file) => file,
        };

        self.fh = Rc::new(RefCell::new(file));
    }

    pub fn append_doc(&self, val: &Value) -> Result<(), Error> {
        let j = serde_json::to_string(val)?;
        self.fh.borrow_mut().write(j.as_bytes())?;
        return Ok(())
    }
}


fn untyped_example() -> Result<(), serde_json::Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][1]);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        untyped_example().unwrap();

        //assert_eq!(mdata.size(), [4, 3]);

    }

    #[test]
    fn test_write() {
        let mut file_driver = FileDrv::new("./docdb.db");
        
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data).unwrap();
        
        file_driver.append_doc(&v).unwrap();
    }
}
