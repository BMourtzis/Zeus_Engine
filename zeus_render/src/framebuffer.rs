use gfx_hal::{
    device::Device,
    pool::{CommandPool, CommandPoolCreateFlags},
    Backend,
};

use std::{
    cell::RefCell,
    rc::Rc
};

use super::{
    device::DeviceState
};

pub struct FramebufferState<B: Backend> {
    command_pools: Option<Vec<B::CommandPool>>,
    command_buffer_lists: Vec<Vec<B::CommandBuffer>>,
    present_semaphores: Option<Vec<B::Semaphore>>,
    device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> FramebufferState<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        num_frames: u32
    ) -> Self {
        let mut command_pools: Vec<_> = vec![];
        let mut command_buffer_lists = Vec::new();
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        for _ in 0..num_frames {
            command_pools.push(
                device.borrow()
                    .device.create_command_pool(
                        device.borrow().queues.family,
                        CommandPoolCreateFlags::empty(),
                    ).expect("Can't create command pool"),
            );
            command_buffer_lists.push(Vec::new());

            present_semaphores.push(device.borrow().device.create_semaphore().unwrap());
        }

        FramebufferState {
            command_pools: Some(command_pools),
            command_buffer_lists,
            present_semaphores: Some(present_semaphores),
            device
        }
    }

    pub fn get_frame_data(
        &mut self,
        idx: usize
    ) -> FrameData<B> {
        FrameData {
            cmd_pool: &mut self.command_pools.as_mut().unwrap()[idx],
            cmd_buffers: &mut self.command_buffer_lists[idx],
            present_sem: &mut self.present_semaphores.as_mut().unwrap()[idx]

        }
    }
}

impl<B: Backend> Drop for FramebufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;

        unsafe {
            for (mut command_pool, command_buffer_list) in 
                self.command_pools.take()
                .unwrap().into_iter().zip(self.command_buffer_lists.drain(..))
            {
                command_pool.free(command_buffer_list);
                device.destroy_command_pool(command_pool);
            }

            for present_semaphore in self.present_semaphores.take().unwrap() {
                device.destroy_semaphore(present_semaphore);
            }
        }
    }
}

#[derive(Debug)]
pub struct FrameData<'a, B: Backend> {
    
    pub cmd_pool: &'a mut B::CommandPool,
    pub cmd_buffers: &'a mut Vec<B::CommandBuffer>,
    pub present_sem: &'a mut B::Semaphore
}
