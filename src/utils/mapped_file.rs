use std::path::Path;

pub struct MappedFile {
    file: std::fs::File,
    length: usize,
    ptr: *mut libc::c_void,
}

impl MappedFile {
    pub fn new<P>(path: &P, length: usize) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        file.set_len(length as u64)?;

        Self::from_existing_file(file)
    }

    pub fn from_existing_file(file: std::fs::File) -> std::io::Result<Self> {
        use std::os::unix::io::AsRawFd;

        let length = file.metadata()?.len() as usize;

        // SAFETY: File was opened successfully, file mode is RW, offset is aligned
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                length,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                file.as_raw_fd(),
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err(std::io::Error::last_os_error());
        }

        if unsafe { libc::madvise(ptr, length, libc::MADV_RANDOM) } != 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Self { file, length, ptr })
    }

    pub fn length(&self) -> usize {
        self.length
    }

    /// Copies chunk of bytes to the specified buffer
    ///
    /// # Safety
    /// The caller must take care that the buffer is not out of the mapped memory!
    pub unsafe fn read_exact_at(&self, offset: usize, buffer: &mut [u8]) {
        std::ptr::copy_nonoverlapping(
            (self.ptr as *const u8).add(offset),
            buffer.as_mut_ptr(),
            buffer.len(),
        )
    }

    /// Copies buffer to the mapped memory
    ///
    /// # Safety
    /// The caller must take care that the buffer is not out of the mapped memory!
    pub unsafe fn write_all_at(&self, offset: usize, buffer: &[u8]) {
        std::ptr::copy_nonoverlapping(
            buffer.as_ptr(),
            (self.ptr as *mut u8).add(offset),
            buffer.len(),
        )
    }
}

impl Drop for MappedFile {
    fn drop(&mut self) {
        // SAFETY: File still exists, ptr and length were initialized once on creation
        if unsafe { libc::munmap(self.ptr, self.length) } != 0 {
            // TODO: how to handle this?
            panic!("failed to unmap file: {}", std::io::Error::last_os_error());
        }

        let _ = self.file.set_len(0);
        let _ = self.file.sync_all();
    }
}

unsafe impl Send for MappedFile {}
