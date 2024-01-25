#![no_main]
#![no_std]

extern crate alloc;

use basic_kernel::{
    allocator,
    fs::FileSystem,
    memory::{self, BootInfoFrameAllocator},
    println,
    task::{executor::Executor, keyboard, Task},
};
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, {}!", "world");

    basic_kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialized failed");

    let filesystem = FileSystem::new();
    println!(
        "etc/fstab contents: {:?}",
        filesystem.get_file("/etc/fstab").unwrap().contents
    );

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

// PANIC HANDLER

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    basic_kernel::hlt_loop();
}
