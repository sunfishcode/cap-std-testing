use crate::fs::{DirBuilder, File, Metadata, OpenOptions, Permissions, ReadDir};
use std::{
    fmt, fs, io,
    os::wasi::{
        fs::OpenOptionsExt,
        io::{AsRawFd, IntoRawFd},
    },
    path::{Path, PathBuf},
};
//use yanix::file::{linkat, mkdirat, unlinkat, AtFlag, Mode, OFlag};

pub(crate) struct Dir {
    std_file: fs::File,
}

impl Dir {
    #[inline]
    pub(crate) fn from_std_file(std_file: fs::File) -> Self {
        Self { std_file }
    }

    #[inline]
    pub(crate) fn into_std_file(self) -> fs::File {
        self.std_file
    }

    #[inline]
    pub(crate) fn as_raw_fd(&self) -> i32 {
        self.std_file.as_raw_fd() as i32
    }

    #[inline]
    pub(crate) fn into_raw_fd(self) -> i32 {
        self.std_file.into_raw_fd() as i32
    }

    pub(crate) fn open_file_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        options.open_at(&self.std_file, path).map(File::from_std)
    }

    pub(crate) fn open_dir(&self, path: &Path) -> io::Result<crate::fs::Dir> {
        /*
        self.open_file_with(
            path,
            OpenOptions::new()
                .read(true)
                .custom_flags(OFlag::DIRECTORY.bits()),
        )
        .map(|file| crate::fs::Dir::from_std_file(file.std))
        */
        unimplemented!("Dir::open_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn create_dir(&self, path: &Path) -> io::Result<()> {
        /*
        unsafe {
            mkdirat(
                self.std_file.as_raw_fd(),
                path,
                Mode::from_bits(0o777).unwrap(),
            )
        }
        */
        unimplemented!("Dir::create_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        // TODO Implement canoncalize without returning an absolute path.
        unimplemented!("Dir::canonicalize({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn copy(&self, from: &Path, to: &Path) -> io::Result<u64> {
        unimplemented!(
            "Dir::copy({:?}, {}, {})",
            self.std_file,
            from.display(),
            to.display()
        )
    }

    pub(crate) fn hard_link(&self, src: &Path, dst_dir: &Dir, dst: &Path) -> io::Result<()> {
        /*
        unsafe {
            linkat(
                self.std_file.as_raw_fd(),
                src,
                dst_dir.std_file.as_raw_fd(),
                dst,
                AtFlag::from_bits(0).unwrap(),
            )
        }
        */
        unimplemented!(
            "Dir::link_at({:?}, {}, {:?}, {})",
            self.std_file,
            src.display(),
            dst_dir,
            dst.display()
        )
    }

    pub(crate) fn metadata(&self, path: &Path) -> io::Result<Metadata> {
        unimplemented!("Dir::metadata({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn read_dir(&self, path: &Path) -> io::Result<ReadDir> {
        unimplemented!("Dir::read_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn read_link(&self, path: &Path) -> io::Result<PathBuf> {
        unimplemented!("Dir::read_link({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn remove_dir(&self, path: &Path) -> io::Result<()> {
        unimplemented!("Dir::remove_dir({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::remove_dir_all({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn remove_file(&self, path: &Path) -> io::Result<()> {
        /*
        unsafe { unlinkat(self.std_file.as_raw_fd(), path, AtFlag::empty()) }
        */
        unimplemented!("Dir::remove_file({:?}, {})", self.std_file, path.display())
    }

    pub(crate) fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::rename({:?}, {}, {})",
            self.std_file,
            from.display(),
            to.display()
        )
    }

    pub(crate) fn set_permissions(&self, path: &Path, perm: Permissions) -> io::Result<()> {
        unimplemented!(
            "Dir::set_permissions({:?}, {}, {:?})",
            self.std_file,
            path.display(),
            perm
        )
    }

    pub(crate) fn symlink_metadata(&self, path: &Path) -> io::Result<Metadata> {
        unimplemented!(
            "Dir::symlink_metadata({:?}, {:?})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn create_with_dir_builder(
        &self,
        dir_builder: &DirBuilder,
        path: &Path,
    ) -> io::Result<()> {
        unimplemented!(
            "Dir::create_with_dir_builder({:?}, {})",
            self.std_file,
            path.display()
        )
    }

    pub(crate) fn symlink(&self, src: &Path, dst: &Path) -> io::Result<()> {
        unimplemented!(
            "Dir::symlink({:?}, {}, {})",
            self.std_file,
            src.display(),
            dst.display()
        )
    }

    pub(crate) fn try_clone(&self) -> io::Result<Dir> {
        Ok(Self::from_std_file(self.std_file.try_clone()?))
    }
}

impl fmt::Debug for Dir {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("Dir");

        unsafe fn get_mode(fd: std::os::wasi::io::RawFd) -> Option<(bool, bool)> {
            let mode = yanix::fcntl::get_status_flags(fd);
            if mode.is_err() {
                return None;
            }
            match mode.unwrap() & yanix::file::OFlag::ACCMODE {
                yanix::file::OFlag::RDONLY => Some((true, false)),
                yanix::file::OFlag::RDWR => Some((true, true)),
                yanix::file::OFlag::WRONLY => Some((false, true)),
                _ => None,
            }
        }

        let fd = self.std_file.as_raw_fd();
        b.field("fd", &fd);
        if let Some((read, write)) = unsafe { get_mode(fd) } {
            b.field("read", &read).field("write", &write);
        }

        b.finish()
    }
}