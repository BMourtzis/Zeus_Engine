use gfx_hal::{
    adapter::{Adapter, PhysicalDevice},
    format:: {self, Properties},
    queue::{QueueFamily, QueueGroup},
    window::Surface,
    Backend,
};

#[derive(Debug)]
pub struct DeviceState<B: Backend> {
    pub device: B::Device,
    pub physical_device: B::PhysicalDevice,
    pub queues: QueueGroup<B>,
}

impl<B: Backend> DeviceState<B> {
    pub fn new(
        adapter: Adapter<B>,
        surface: &B::Surface,
    ) -> Self {
        let family = adapter
            .queue_families.iter()
            .find(|family| {
                // println!("{:?}", family);
                surface.supports_queue_family(family) && family.queue_type().supports_graphics()
            }).unwrap();

        let mut gpu = unsafe {
            adapter.physical_device
                .open(&[(family, &[1.0])], gfx_hal::Features::empty())
                .unwrap()
        };

        DeviceState {
            device: gpu.device,
            queues: gpu.queue_groups.pop().unwrap(),
            physical_device: adapter.physical_device,
        }
    }

    pub fn physical_device_format_properties(&self, format: Option<format::Format>) -> Properties {
        self.physical_device.format_properties(format)
    }
}
