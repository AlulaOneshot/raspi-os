use std::{fs::File, os::fd::AsFd};

use drm::{control::{connector, Device}};

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
    drm_device: Option<ShiotaCardWrapper>,
}

impl ShiotaGLContext {
    pub fn new() -> Self {
        Self {
            drm_device: None,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let device = ShiotaCardWrapper::open("/dev/dri/card0").map_err(|e| e.to_string())?;
        let res_handles = device.resource_handles().unwrap();
        println!("Connectors: {:?}", res_handles.connectors());

        for conn in res_handles.connectors() {
            let info = device.get_connector(*conn, false).unwrap();
            if let connector::State::Connected = info.state() {

            }
        }

        Ok(())
    }
}