mod platform;

use std::{env, error::Error, future::pending};
use zbus::Connection;

use crate::platform::CtrlPlatform;

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let is_service = match env::var_os("IS_SERVICE") {
        Some(val) => val == "1",
        None => false,
    };
    let is_debug = match env::var_os("IS_DEBUG") {
        Some(val) => val == "1",
        None => false,
    };

    if !is_service {
        println!("gigabyted should only be run as a systemd service!");
        println!("If you need logs, use journalctl -b -u gigabyted");
        println!("gigabyted will now exit");
        return Ok(());
    }

    if is_debug {
        env::set_var("RUST_LOG", "debug");
        println!("RUST_LOG set to debug");
        println!("gigabyted will run on com.gigabyte.debug to avoid interfering with the real com.gigabyte.daemon");
    }

    let connection = Connection::system().await?;

    // Setup interface for kernel driver
    let ctrl = CtrlPlatform::new().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not initialize the AORUS platform device",
        )
    })?;

    connection
        .object_server()
        .at("/com/gigabyte/Platform", ctrl)
        .await?;

    // Request name
    if is_debug {
        connection.request_name("com.gigabyte.debug").await?;
    } else {
        connection.request_name("com.gigabyte.daemon").await?;
    }

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
