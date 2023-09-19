#![allow(unused)]
fn main() {
    use vulkano::VulkanLibrary;
    use vulkano::instance::{Instance, InstanceCreateInfo};

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default())
        .expect("failed to create instance");

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    // prints out some queues that i dont fully understand
    for family in physical_device.queue_family_properties() {
        println!("Found a queue family with {:?} queue(s)", family.queue_count);
    }

    use vulkano::device::QueueFlags;

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
        .expect("failed to create device");


    use vulkano::memory::allocator::StandardMemoryAllocator;

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let queue = queues.next().unwrap();
}

