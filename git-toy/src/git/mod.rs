use std::{
    error,
    ffi::{CStr, CString},
    fmt,
    os::raw::c_int,
    path::Path,
    ptr, result,
};

mod raw;

#[derive(Debug)]
pub struct Error {
    code: i32,
    message: String,
    class: i32,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error {
            code: -1,
            message,
            class: 0,
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Self {
        Error {
            code: -1,
            message: e.to_string(),
            class: 0,
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub struct Repository {
    raw: *mut raw::git_repository,
}

impl Repository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository> {
        ensure_initialized();
        let path = path_to_cstring(path.as_ref())?;
        let mut repo = ptr::null_mut();
        unsafe {
            check(raw::git_repository_open(&mut repo, path.as_ptr()))?;
        }
        Ok(Repository { raw: repo })
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        unsafe {
            raw::git_repository_free(self.raw);
        }
    }
}
#[cfg(unix)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;

    Ok(CString::new(path.as_os_str().as_bytes())?)
}

#[cfg(windows)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    match path.to_str() {
        Some(s) => Ok(CString::new(s)?),
        None => {
            let message = format!("Couldn't convert path '{}' to UTF-8", path.display());
            Err(message.into())
        }
    }
}

fn ensure_initialized() {
    static ONCE: std::sync::Once = std::sync::Once::new();

    ONCE.call_once(|| unsafe {
        check(raw::git_libgit2_init()).expect("Initializing libgit2 failed");
        assert_eq!(libc::atexit(shutdown), 0);
    });
}

extern "C" fn shutdown() {
    unsafe {
        if let Err(e) = check(raw::git_libgit2_shutdown()) {
            eprintln!("shutting down libgit2 failed: {}", e);
            std::process::abort()
        }
    }
}

fn check(code: c_int) -> Result<c_int> {
    if code >= 0 {
        return Ok(code);
    }

    unsafe {
        let error = raw::giterr_last();

        let message = CStr::from_ptr((*error).message)
            .to_string_lossy()
            .into_owned();

        Err(Error {
            code,
            message,
            class: (*error).klass as i32,
        })
    }
}
