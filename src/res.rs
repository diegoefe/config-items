#[cfg(feature = "logging")]
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

// Magic trait to cast a ERes<T> into a SRes<T>
pub trait CastSRes<T> {
    fn into_sres(self) -> SRes<T>;
}

// Cast a ERes<T> into a SRes<T>
impl<T> CastSRes<T> for ERes<T> {
    fn into_sres(self) -> SRes<T> {
        self.map_err(|e| Box::from(e.to_string()) as Box<StdErrSS>)
    }
}

// It does nothing, already compatible
impl<T> CastSRes<T> for SRes<T> {
    fn into_sres(self) -> SRes<T> {
        self
    }
}

// Magic trait to cast a ERes<T> into a SRes<T>
pub trait CastERes<T> {
    fn into_eres(self) -> ERes<T>;
}

// Cast a ERes<T> into a SRes<T>
impl<T> CastERes<T> for SRes<T> {
    fn into_eres(self) -> ERes<T> {
        self.map_err(|e| Box::from(e.to_string()) as Box<StdErr>)
    }
}

// It does nothing, already compatible
impl<T> CastERes<T> for ERes<T> {
    fn into_eres(self) -> ERes<T> {
        self
    }
}

/// Fail if second parameter is None
pub fn get_some<T>(name:&str, opt:Option<T>) -> ERes<T> {
    match opt {
        Some(v)=>Ok(v),
        None=>{
            let e = format!("get_some({name}) must be Some()");
            #[cfg(feature = "logging")]
            error!("{e}");
            Err(e.into())
        }
    }
}

/// Fail if second parameter is None, thread-safely
pub fn get_somes<T>(name:&str, opt:Option<T>) -> SRes<T> {
    Ok(get_some(name, opt).into_sres()?)
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

    fn returns_eres() -> ERes<()> {
        Ok(())
    }

    fn returns_sres() -> SRes<()> {
        Ok(())
    }

    #[test]
    fn test_into_sres() -> SRes<()> {
        returns_sres()?;
        Ok(returns_eres().into_sres()?)
    }

    #[test]
    fn test_into_eres() -> ERes<()> {
        returns_eres()?;
        Ok(returns_sres().into_eres()?)
    }
}