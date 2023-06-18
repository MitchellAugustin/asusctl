use std::error::Error;
use std::time::Instant;

use log::error;
use rog_anime::usb::{PROD_ID, VENDOR_ID};
use rog_anime::USB_PREFIX2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use uhid_virt::{Bus, CreateParams, UHIDDevice};

pub struct VirtAnimeMatrix {
    device: UHIDDevice<std::fs::File>,
    buffer: [u8; 640],
    time: Instant,
}

impl Default for VirtAnimeMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtAnimeMatrix {
    pub fn new() -> Self {
        VirtAnimeMatrix {
            time: Instant::now(),
            buffer: [0; 640],
            device: UHIDDevice::create(CreateParams {
                name: String::from("ROG_Virtual Anime Matrix"),
                phys: String::from(""),
                uniq: String::from(""),
                bus: Bus::USB,
                vendor: VENDOR_ID as u32,
                product: PROD_ID as u32,
                version: 0,
                country: 0,
                // This is a device which emits the usage code as a whole, rather than as bits
                rd_data: [
                    0x06, 0x31, 0xff, // Usage Page (Vendor Defined 0xFF31)
                    0x09, 0x76, // Usage (0x76)
                    0xa1, 0x01, // Collection (Application)
                    0x85, 0x5a, //   Report ID (90)
                    0x19, 0x00, //   Usage Minimum (0x00)
                    0x2a, 0xff, 0x00, //   Usage Maximum (0xFF)
                    0x15, 0x00, //   Logical Minimum (0)
                    0x26, 0xff, 0x00, //   Logical Maximum (255)
                    0x75, 0x08, //   Report Size (8)
                    0x95, 0x05, //   Report Count (5)
                    0x81,
                    0x00, /*   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null
                           * Position) */
                    0x19, 0x00, //   Usage Minimum (0x00)
                    0x2a, 0xff, 0x00, //   Usage Maximum (0xFF)
                    0x15, 0x00, //   Logical Minimum (0)
                    0x26, 0xff, 0x00, //   Logical Maximum (255)
                    0x75, 0x08, //   Report Size (8)
                    0x95, 0x3f, //   Report Count (63)
                    0xb1,
                    0x00, /*   Feature (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null
                           * Position,Non-volatile) */
                    0xc0, // End Collection
                    0x06, 0x31, 0xff, // Usage Page (Vendor Defined 0xFF31)
                    0x09, 0x80, // Usage (0x80)
                    0xa1, 0x01, // Collection (Application)
                    0x85, 0x5e, //   Report ID (94)
                    0x19, 0x00, //   Usage Minimum (0x00)
                    0x2a, 0xff, 0x00, //   Usage Maximum (0xFF)
                    0x15, 0x00, //   Logical Minimum (0)
                    0x26, 0xff, 0x00, //   Logical Maximum (255)
                    0x75, 0x08, //   Report Size (8)
                    0x95, 0x05, //   Report Count (5)
                    0x81,
                    0x00, /*   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null
                           * Position) */
                    0x19, 0x00, //   Usage Minimum (0x00)
                    0x2a, 0xff, 0x00, //   Usage Maximum (0xFF)
                    0x15, 0x00, //   Logical Minimum (0)
                    0x26, 0xff, 0x00, //   Logical Maximum (255)
                    0x96, 0x7f, 0x02, //   Report Count (639)
                    0xb1,
                    0x00, /*   Feature (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null
                           * Position,Non-volatile) */
                    0xc0, /* End Collection */

                          /* 85 bytes */
                ]
                .to_vec(),
            })
            .map_err(|err| error!("Could not create virtual device: {:?}", err))
            .expect("Could not create virtual device"),
        }
    }

    // /// A single on/off key press
    // pub fn press(&mut self, input: [u8; 32]) {
    //     self.device.write(&input).unwrap();
    //     let mut reset = [0u8; 32];
    //     reset[0] = input[0];
    //     self.device.write(&reset).unwrap();
    // }

    pub fn read(&mut self) {
        if let Ok(event) = self.device.read() {
            match event {
                // uhid_virt::OutputEvent::Start { dev_flags } => todo!(),
                // uhid_virt::OutputEvent::Stop => todo!(),
                // uhid_virt::OutputEvent::Open => todo!(),
                // uhid_virt::OutputEvent::Close => todo!(),
                uhid_virt::OutputEvent::Output { data } => {
                    for (i, b) in self.buffer.iter_mut().enumerate() {
                        *b = 0;
                        if let Some(n) = data.get(i) {
                            *b = *n;
                        }
                    }
                    let now = Instant::now();
                    dbg!(now - self.time);
                    self.time = Instant::now();
                }
                uhid_virt::OutputEvent::GetReport {
                    id,
                    report_number,
                    report_type,
                } => {
                    dbg!(id, report_number, report_type);
                }
                // uhid_virt::OutputEvent::SetReport { id, report_number, report_type, data } =>
                // todo!(),
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut dev = VirtAnimeMatrix::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        dev.read(); // it's blocking, and damned hard to sync with arc/mutex
        let one = dev.buffer[0..7] != USB_PREFIX2;

        for (mut i, b) in dev.buffer.iter().skip(7).enumerate() {
            if !one {
                i += 640;
            };
            if *b == 0 {
                continue;
            }
            canvas.set_draw_color(Color::RGB(*b as u8, *b as u8, *b as u8));
            let y = (i / 33) % 55;
            let x = (i % 33) as i32 * 20;
            // if i % 2 == 0 {
            //     x /= 2;
            // }
            dbg!(i, i / 33);
            canvas
                .fill_rect(Rect::new(x, y as i32 * 20, 20, 20))
                .unwrap();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if dev.buffer[0..7] == USB_PREFIX2 {
            canvas.present();
        }
        // ::std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}