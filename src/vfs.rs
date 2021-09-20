//! Access to the [Virtual File System (VFS) layer](http://doc.riot-os.org/group__sys__vfs.html)
//!
//! This abstraction tries not to be smart about modes -- a [File] opened with RDONLY will still
//! have a write method, and because file operations are generally fallible, writes will just fail.
//!
//! ## Panics
//!
//! This module violently asserts that file names are UTF-8 encoded (a condition easily satisified
//! if only ASCII file names are used).
//!
//! ## Incomplete
//!
//! So far, only a subset of VFS is implemented; in particular, the file system is read-only.

use core::marker::PhantomData;
use core::mem::MaybeUninit;

use riot_sys::libc;

use crate::error::{NegativeErrorExt, NumericError};

/// A file handle
#[derive(Debug)]
pub struct File {
    // Nonnegative, actually -- but as long as NumericError isn't known-negative, this doesn't help
    // with returning results.
    fileno: libc::c_int,
    // Sending file descriptors around is currently possible in RIOT, but discouraged
    _not_send_sync: PhantomData<*const ()>,
}

/// Results of a file stat operation
#[derive(Debug)]
pub struct Stat(riot_sys::stat);

impl Stat {
    /// The current size of the file
    pub fn size(&self) -> usize {
        self.0.st_size as _
    }
}

/// Parameter for seeking in a file
///
/// It is analogous to [std::io::SeekFrom].
#[derive(Debug)]
pub enum SeekFrom {
    /// Seek to the given position from the start of the file
    Start(usize),
    /// Seek to the given position relative to the end of the file
    End(isize),
    /// Seek to the given position relative to the current cursor position
    Current(isize),
}

impl File {
    /// Open a file in read-only mode.
    pub fn open(path: &str) -> Result<Self, NumericError> {
        let fileno = unsafe {
            riot_sys::vfs_open(
                path as *const str as *const libc::c_char,
                riot_sys::O_RDONLY as _,
                0,
            )
        }
        .negative_to_error()?;
        Ok(File {
            fileno,
            _not_send_sync: PhantomData,
        })
    }

    /// Obtain metadata of the file.
    pub fn stat(&self) -> Result<Stat, NumericError> {
        let mut stat = MaybeUninit::uninit();
        (unsafe { riot_sys::vfs_fstat(self.fileno, stat.as_mut_ptr()) }).negative_to_error()?;
        let stat = unsafe { stat.assume_init() };
        Ok(Stat(stat))
    }

    /// Read into the given buffer from the current cursor position in the file, and advance the
    /// cursor by the read length, which is also returned.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, NumericError> {
        (unsafe {
            riot_sys::vfs_read(
                self.fileno,
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.len() as _,
            )
        })
        .negative_to_error()
        .map(|len| len as _)
    }

    /// Move the file cursor to the indicated position.
    pub fn seek(&mut self, pos: SeekFrom) -> Result<usize, NumericError> {
        let (off, whence) = match pos {
            SeekFrom::Start(i) => (i as _, riot_sys::SEEK_SET as _),
            SeekFrom::Current(i) => (i as _, riot_sys::SEEK_CUR as _),
            SeekFrom::End(i) => (i as _, riot_sys::SEEK_END as _),
        };
        (unsafe { riot_sys::vfs_lseek(self.fileno, off, whence) })
            .negative_to_error()
            .map(|r| r as _)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { riot_sys::vfs_close(self.fileno) };
    }
}


/// A directory in the file system
///
/// The directory can be iterated over, producing directory entries one by one.
pub struct Dir(riot_sys::vfs_DIR);

impl Dir {
    pub fn open(dir: &str) -> Result<Self, NumericError> {
        let mut dirp = MaybeUninit::uninit();
        (unsafe {
            riot_sys::vfs_opendir(dirp.as_mut_ptr(), dir as *const str as *const libc::c_char)
        })
        .negative_to_error()?;
        let dirp = unsafe { dirp.assume_init() };
        Ok(Dir(dirp))
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        unsafe { riot_sys::vfs_closedir(&mut self.0) };
    }
}

impl Iterator for Dir {
    type Item = Dirent;

    fn next(&mut self) -> Option<Dirent> {
        let mut ent = MaybeUninit::uninit();
        let ret = (unsafe { riot_sys::vfs_readdir(&mut self.0, ent.as_mut_ptr()) })
            .negative_to_error()
            .ok()?;
        if ret > 0 {
            Some(Dirent(unsafe { ent.assume_init() }))
        } else {
            None
        }
    }
}

/// Directory entry inside a file
///
/// The entry primarily indicates the file's name.
pub struct Dirent(riot_sys::vfs_dirent_t);

impl Dirent {
    /// Name of the file
    ///
    /// This will panic if the file name is not encoded in UTF-8.
    pub fn name(&self) -> &str {
        // unsafe: File systems need to provide null termination, buffer is sized accordingly.
        // cast: Some platforms have i8 chars, but that can be converted.
        let mut name = unsafe { cstr_core::CStr::from_ptr((&self.0.d_name).as_ptr()) }
            .to_str()
            .expect("File name not UTF-8 encoded");

        // Workaround for https://github.com/RIOT-OS/RIOT/issues/14635
        while name.starts_with("/") {
            name = &name[1..];
        }

        name
    }
}
