use ash::vk;

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub struct FrameSync {
    pub image_available: [vk::Semaphore; MAX_FRAMES_IN_FLIGHT],
    pub in_flight: [vk::Fence; MAX_FRAMES_IN_FLIGHT],
    pub render_finished: Vec<vk::Semaphore>, // per swapchain image
    pub current_frame: usize,
}

impl FrameSync {
    pub fn new(device: &ash::Device, image_count: usize) -> Self {
        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        let mut image_available = [vk::Semaphore::null(); MAX_FRAMES_IN_FLIGHT];
        let mut in_flight = [vk::Fence::null(); MAX_FRAMES_IN_FLIGHT];
        for i in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                image_available[i] = device.create_semaphore(&semaphore_info, None).unwrap();
                in_flight[i] = device.create_fence(&fence_info, None).unwrap();
            }
        }

        let mut render_finished = Vec::with_capacity(image_count);
        for _ in 0..image_count {
            let sem = unsafe { device.create_semaphore(&semaphore_info, None).unwrap() };
            render_finished.push(sem);
        }

        Self {
            image_available,
            in_flight,
            render_finished,
            current_frame: 0,
        }
    }

    pub fn next_frame(&mut self) -> usize {
        let frame = self.current_frame;
        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
        frame
    }

    pub fn wait_for_fence(&self, device: &ash::Device, frame: usize) {
        unsafe {
            device
                .wait_for_fences(&[self.in_flight[frame]], true, u64::MAX)
                .unwrap();
            device.reset_fences(&[self.in_flight[frame]]).unwrap();
        }
    }

    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            for i in 0..MAX_FRAMES_IN_FLIGHT {
                device.destroy_semaphore(self.image_available[i], None);
                device.destroy_fence(self.in_flight[i], None);
            }
            for sempahore in &self.render_finished {
                device.destroy_semaphore(*sempahore, None);
            }
        }
    }
}
