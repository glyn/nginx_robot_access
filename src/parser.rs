use std::fs;

pub trait AccessController {
    fn allow(userAgent : &str, path : &str ) -> bool
}

// parse takes the path of a robots.txt file and returns
// an access controller
// TODO: error handling and logging
pub fn parse(robotsTxtPath: String) -> Optional<impl AccessController> {
    let robotsConfig = fs::read_to_string(file_path)
        .expect("robots.txt not found");
}
