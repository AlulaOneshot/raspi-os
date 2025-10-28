use std::os::fd::AsFd;

use drm::Device as DrmDevice;
use gbm::Device as GbmDevice;

struct ShiotaCardWrapper(std::fs::File);

impl AsFd for ShiotaCardWrapper {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl ShiotaCardWrapper {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        Ok(ShiotaCardWrapper(options.open(path)?))
    }
}

impl drm::Device for ShiotaCardWrapper {}

impl drm::control::Device for ShiotaCardWrapper {}

pub struct ShiotaGLContext {
    gpu: Option<ShiotaCardWrapper>,
}

impl ShiotaGLContext {
    pub fn new() -> Self {
        Self {
            gpu: None,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let gpu = ShiotaCardWrapper::open("/dev/dri/card0").map_err(|e| e.to_string())?;
        
        let gbm = GbmDevice::new(gpu);

        

        self.gpu = Some(gpu);

        Ok(())
    }
}