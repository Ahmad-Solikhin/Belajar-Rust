fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests1 {
    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_log() {
        // env_logger::init(); //Default levelnya adalah erro saja

        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        error!("This error log");
        warn!("This warn log");
        info!("This info log");
        debug!("This debug log");
        trace!("This trace log");
    }
}

#[cfg(test)]
mod tests2 {
    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_log() {
        // env_logger::init(); //Default levelnya adalah erro saja

        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        error!("This error log");
        warn!("This warn log");
        info!("This info log");
        debug!("This debug log");
        trace!("This trace log");
    }
}
