use crate::{config::Config, DBUS_IFACE, DBUS_PATH};
use dbus::{
    blocking::Connection,
    tree::{Factory, MethodErr},
};
use rog_lib::core::RogCore;
use std::error::Error;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

pub struct Daemon {
    rogcore: RogCore,
    config: Config,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon {
            rogcore: RogCore::new().expect("Could not start RogCore"),
            config: Config::default().read(),
        }
    }

    pub fn load_config(mut self) -> Result<Self, Box<dyn Error>> {
        self.rogcore.aura_set_mode(&self.config.builtin)?;
        let bright = RogCore::aura_brightness_bytes(self.config.brightness)?;
        self.rogcore.aura_set_mode(&bright)?;
        Ok(self)
    }

    pub fn write_config(&mut self, bytes: &[u8]) {
        // TODO: create statics out of header bytes
        if bytes[0] == 0x5a && bytes[1] == 0xba {
            self.config.brightness = bytes[4];
        } else if bytes[0] == 0x5d && bytes[1] == 0xb3 {
            self.config.builtin = bytes.to_vec();
        }
        self.config.write();
    }

    pub fn start() -> Result<(), Box<dyn Error>> {
        let mut connection = Connection::new_system().expect("Could not set up dbus system");
        connection.request_name(DBUS_IFACE, false, true, false)?;
        let factory = Factory::new_fnmut::<()>();

        let daemon = Self::new().load_config()?;
        let daemon = Rc::new(RefCell::new(daemon));

        // We create a tree with one object path inside and make that path introspectable.
        let tree = factory.tree(()).add(
            factory.object_path(DBUS_PATH, ()).introspectable().add(
                // We add an interface to the object path...
                factory
                    .interface(DBUS_IFACE, ())
                    // ...and a method inside the interface
                    .add_m(
                        factory
                            .method("ledmessage", (), {
                                let daemon = daemon.clone();
                                move |m| {
                                    // Reads the args passed to the method
                                    let bytes: Vec<u8> = m.msg.read1()?;
                                    let s = format!("Wrote {:x?}", bytes);

                                    let mut daemon = daemon.borrow_mut();
                                    match daemon.rogcore.aura_set_mode(&bytes[..]) {
                                        Ok(_) => {
                                            daemon.write_config(&bytes);
                                            let mret = m.msg.method_return().append1(s);
                                            Ok(vec![mret])
                                        }
                                        Err(err) => Err(MethodErr::failed(&err)),
                                    }
                                }
                            })
                            // Input?
                            .outarg::<&str, _>("reply")
                            .inarg::<Vec<u8>, _>("bytearray"),
                    ),
            ),
        );

        // We add the tree to the connection so that incoming method calls will be handled.
        tree.start_receive(&connection);

        loop {
            connection.process(Duration::from_millis(5))?;
            // READ KEYBOARD
            // TODO: this needs to move to a thread
            match daemon.borrow_mut().rogcore.poll_keyboard() {
                Ok(buf) => {
                    // [5d, 1, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] up
                    // [5d, 1, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] down
                    // [5d, 1, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] left
                    // [5d, 1, 0, 0, 4f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] right

                    // [5a, c4, ; 32 bytes long] fn+up
                    // [5a, c5, ; 32 bytes long] fn+down
                    // [5a, b2, ; 32 bytes long] fn+left
                    // [5a, b3, ; 32 bytes long] fn+right

                    // To handle keys for aura:
                    // read config + inc/dec brightness byte
                    // write to aura
                    // write config
                    println!("{:x?}", buf);
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
}