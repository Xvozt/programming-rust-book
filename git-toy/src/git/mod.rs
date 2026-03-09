use std::{
    error,
    ffi::{CStr, CString},
    fmt,
    marker::PhantomData,
    mem,
    os::raw::c_int,
    path::Path,
    ptr, result,
};

use libc::c_char;

mod raw;

#[allow(dead_code)]
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

    pub fn reference_name_to_id(&self, name: &str) -> Result<Oid> {
        let name = CString::new(name)?;
        unsafe {
            let oid = {
                let mut oid = mem::MaybeUninit::uninit();
                check(raw::git_reference_name_to_id(
                    oid.as_mut_ptr(),
                    self.raw,
                    name.as_ptr() as *const c_char,
                ))?;

                oid.assume_init()
            };
            Ok(Oid { raw: oid })
        }
    }

    pub fn find_commit<'repo, 'id>(&'repo self, oid: &'id Oid) -> Result<Commit<'repo>> {
        let mut commit = ptr::null_mut();
        unsafe {
            check(raw::git_commit_lookup(&mut commit, self.raw, &oid.raw))?;
        }
        Ok(Commit {
            raw: commit,
            _marker: PhantomData,
        })
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        unsafe {
            raw::git_repository_free(self.raw);
        }
    }
}

pub struct Oid {
    pub raw: raw::git_oid,
}

pub struct Commit<'repo> {
    raw: *mut raw::git_commit,
    _marker: PhantomData<&'repo Repository>,
}

impl<'repo> Commit<'repo> {
    pub fn author<'commit>(&self) -> Signature<'commit> {
        unsafe {
            Signature {
                raw: raw::git_commit_author(self.raw),
                _marker: PhantomData,
            }
        }
    }

    pub fn message(&self) -> Option<&str> {
        unsafe {
            let message = raw::git_commit_message(self.raw);
            char_ptr_to_str(self, message)
        }
    }
}

impl<'repo> Drop for Commit<'repo> {
    fn drop(&mut self) {
        unsafe {
            raw::git_commit_free(self.raw);
        }
    }
}

pub struct Signature<'text> {
    raw: *const raw::git_signature,
    _marker: PhantomData<&'text str>,
}

impl<'text> Signature<'text> {
    pub fn name(&self) -> Option<&str> {
        unsafe { char_ptr_to_str(self, (*self.raw).name) }
    }

    pub fn email(&self) -> Option<&str> {
        unsafe { char_ptr_to_str(self, (*self.raw).email) }
    }
}

unsafe fn char_ptr_to_str<T>(_owner: &T, ptr: *const c_char) -> Option<&str> {
    if ptr.is_null() {
        return None;
    } else {
        unsafe { CStr::from_ptr(ptr).to_str().ok() }
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
