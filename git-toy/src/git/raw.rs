#![allow(non_camel_case_types, dead_code)]

use std::{
    ffi::c_int,
    os::raw::{c_char, c_uchar},
};

#[link(name = "git2")]
unsafe extern "C" {
    pub unsafe fn git_libgit2_init() -> c_int;
    pub unsafe fn git_libgit2_shutdown() -> c_int;
    pub unsafe fn giterr_last() -> *const get_error;
    pub unsafe fn git_repository_open(out: *mut *mut git_repository, path: *const c_char) -> c_int;
    pub unsafe fn git_repository_free(repo: *mut git_repository);
    pub unsafe fn git_reference_name_to_id(
        out: *mut git_oid,
        repo: *mut git_repository,
        reference: *const c_char,
    ) -> c_int;
    pub unsafe fn git_commit_lookup(
        out: *mut *mut git_commit,
        repo: *mut git_repository,
        id: *const git_oid,
    ) -> c_int;

    pub unsafe fn git_commit_author(commit: *const git_commit) -> *const git_signature;
    pub unsafe fn git_commit_message(commit: *const git_commit) -> *const c_char;
    pub unsafe fn git_commit_free(commit: *mut git_commit);
}

#[repr(C)]
pub struct git_repository {
    _private: [u8; 0],
}

#[repr(C)]
pub struct git_commit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct get_error {
    pub message: *const c_char,
    pub klass: c_int,
}

pub const GIT_OID_RAWSZ: usize = 20;

#[repr(C)]
pub struct git_oid {
    pub id: [c_uchar; GIT_OID_RAWSZ],
}

#[repr(C)]
pub struct git_signature {
    pub name: *const c_char,
    pub email: *const c_char,
    pub when: git_time,
}

pub type git_time_t = i64;

#[repr(C)]
pub struct git_time {
    pub time: git_time_t,
    pub offset: c_int,
}
