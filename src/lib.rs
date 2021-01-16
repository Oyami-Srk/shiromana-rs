mod misc;
mod library;

#[cfg(test)]
mod tests {
    use crate::library::{Library, LibrarySummary};

    #[test]
    fn it_works() {
        println!("Test");
        let a = LibrarySummary {
            media_size: 0,
            media_count: 0,
            series_count: 0,
        };
        println!("{}", a);
        match Library::open("test.mlib".to_string()) {
            Ok(lib) => (),
            Err(E) => println!("{}", E)
        }
    }
}

