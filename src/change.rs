use crate::{device_ops::open_device, devices::DevicePool};
use rusb::{Context, Result};

pub fn change(vendor_id: u16, product_id: u16, prop: &str, value: &str) -> Result<()> {
    let pool = DevicePool::new();

    // let mut context = Context::new()?;
    // let (mut _device, mut handle) =
    //     open_device(&mut context, vendor_id, product_id).expect("Failed to open device");

    let (mut _device, mut handle) = pool
        .find_one(vendor_id, product_id)
        .unwrap()
        .open_device()
        .expect("Failed to open device");

    let iface = 5;

    handle.set_auto_detach_kernel_driver(true).unwrap();

    // println!(
    //     "Changing {} to {} for {}-{}",
    //     prop, value, vendor_id, product_id
    // );

    let payload = vec![
        0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, 0x00, 0x00, 0xff, 0xff, 0x32, 0xc8, 0xc8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    // println!("Payload size: {}", payload.len());

    match handle.claim_interface(iface) {
        Ok(()) => {
            match handle.write_control(
                rusb::request_type(
                    rusb::Direction::Out,
                    rusb::RequestType::Class,
                    rusb::Recipient::Interface,
                ),
                9,
                0x0206,
                iface.into(),
                &payload,
                std::time::Duration::from_secs(1),
            ) {
                Ok(size) => {
                    println!("{} bytes transferred", size);
                }
                Err(e) => {
                    return Err(e);
                }
            }

            handle.release_interface(iface).unwrap();
        }
        Err(e) => {
            println!("Could not claim interface: {}", e);
        }
    }

    Ok(())
}
