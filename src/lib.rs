struct XWall {
    dpy: *mut x11::xlib::Display,
    root: x11::xlib::Window,
    bitmap: *mut x11::xlib::Pixmap,
    image_x: *mut u32,
    image_y: *mut u32,
    x_hot: *mut i32,
    y_hot: *mut i32,
}

impl XWall {
    pub fn new(dpy: *mut x11::xlib::Display, root: x11::xlib::Window) -> Self {
        Self {
            dpy,
            root,
            bitmap: std::ptr::null_mut::<u64>(),
            image_x: std::ptr::null_mut::<u32>(),
            image_y: std::ptr::null_mut::<u32>(),
            x_hot: std::ptr::null_mut::<i32>(),
            y_hot: std::ptr::null_mut::<i32>(),
        }
    }

    fn read_bitmap_file(&self, filename: &str) -> Result<(), String> {
        let status: i32;
        status = unsafe {
            x11::xlib::XReadBitmapFile(
                self.dpy,
                self.root,
                filename.as_ptr().cast::<i8>(),
                self.image_x,
                self.image_y,
                self.bitmap,
                self.y_hot,
                self.y_hot,
            )
        };
        match status {
            0 => Ok(()),
            1 => Err(format!("Can't open file: {}", filename)),
            2 => Err(format!("Bad bitmap format file: {}", filename)),
            _ => Err(format!("Шnsufficient memory for bitmap: {}", filename)),
        }
    }

    fn name_to_pixel() -> u32 {
        unimplemented!();
    }

    pub fn set_background_to_bitmap() {
        unimplemented!();
    }
}
