use gfx_hal::{
    device::Device,
    pso::{
        Descriptor, DescriptorArrayIndex, DescriptorBinding, DescriptorPool,
        DescriptorSetLayoutBinding, DescriptorSetWrite,
    },
    Backend,
};

use std::{borrow, cell::RefCell, rc::Rc};

use super::device::DeviceState;

#[derive(Debug)]
pub struct DescSetLayout<B: Backend> {
    pub layout: Option<B::DescriptorSetLayout>,
    pub device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> DescSetLayout<B> {
    pub fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        binding: Vec<DescriptorSetLayoutBinding>,
    ) -> Self {
        let desc_set_layout = unsafe {
            device.borrow()
                .device.create_descriptor_set_layout(binding, &[])
        }.ok();

        DescSetLayout {
            layout: desc_set_layout,
            device,
        }
    }

    pub fn create_desc_set(
        self,
        desc_pool: &mut B::DescriptorPool,
    ) -> DescSet<B> {
        let desc_set = unsafe { 
            desc_pool.allocate_set(self.layout.as_ref().unwrap())
        }.unwrap();

        DescSet {
            layout: self,
            set: Some(desc_set),
        }
    }
}

impl<B: Backend> Drop for DescSetLayout<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_descriptor_set_layout(self.layout.take().unwrap());
        }
    }
}

#[derive(Debug)]
pub struct DescSet<B: Backend> {
    pub set: Option<B::DescriptorSet>,
    pub layout: DescSetLayout<B>,
}

impl<B: Backend> DescSet<B> {
    pub fn write_to_state<'a, 'b: 'a, W>(
        &'b mut self,
        write: Vec<DescSetWrite<W>>,
        device: &mut B::Device,
    ) where
        W: IntoIterator,
        W::Item: borrow::Borrow<Descriptor<'a, B>>,
    {
        let set = self.set.as_ref().unwrap();
        let write: Vec<_> = write.into_iter()
            .map(|d| DescriptorSetWrite {
                binding: d.binding,
                array_offset: d.array_offset,
                descriptors: d.descriptors,
                set,
            }).collect();

        unsafe { 
            device.write_descriptor_sets(write)
        };
    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.layout.layout.as_ref()
            .unwrap()
    }
}

pub struct DescSetWrite<W> {
    pub binding: DescriptorBinding,
    pub array_offset: DescriptorArrayIndex,
    pub descriptors: W,
}
