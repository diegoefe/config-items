use log::error;

/// Standard error
pub type StdErr = dyn std::error::Error;
/// Thread-safe standard error
pub type StdErrSS = dyn std::error::Error + Send + Sync;
/// boxed StdErr
pub type BoxErr = Box<StdErr>;
/// Result template
pub type ERes<T> = std::result::Result<T, BoxErr>;
/// Thread-safe Result template
pub type SRes<T> = std::result::Result<T, Box<StdErrSS>>;

/// Fail if second parameter is None
pub fn get_some<T>(name:&str, opt:Option<T>) -> ERes<T> {
    match opt {
        Some(v)=>Ok(v),
        None=>{
            let e = format!("get_some({name}) must be Some()");
            error!("{e}");
            Err(e.into())
        }
    }
}

/// Fail if second parameter is None, thread-safely
pub fn get_somes<T>(name:&str, opt:Option<T>) -> SRes<T> {
    match opt {
        Some(v)=>Ok(v),
        None=>{
            let e = format!("get_somes({name}) must be Some()");
            error!("{e}");
            Err(e.into())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn ok() -> ERes<()> { Ok(()) }
    fn err() -> ERes<()> { Err("bad".into()) }
    
    fn none() -> Option<bool> { None }
    fn some() -> Option<bool> { Some(true) }
    
    fn sok() -> SRes<()> { Ok(()) }
    fn serr() -> SRes<()> { Err("sbad".into()) }

    #[test]
    fn test_res() {
        assert!(ok().is_ok());
        assert!(err().is_err());

        assert!(sok().is_ok());
        assert!(serr().is_err());

        match get_some("none", none()) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), "get_some(none) must be Some()".to_string()),
        }

        assert!(get_some("none", none()).is_err());
        assert!(get_some("some", some()).is_ok());
    }
}