# BMS-rs

Read BMS memory mapped data in rust

Quick example:
```rs
use bms::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mem = IntellivibeData::new();
    loop {
        if mem.is_ok() {
            break;
        }
        sleep(Duration::from_millis(300)).await;
        mem = IntellivibeData::new();
    }
    let mem = mem?;

    // Threadable
    tokio::spawn(async move {
        loop {
            // Read the memory as it is right now
            dbg!(mem.read());
            sleep(Duration::from_secs(5)).await;
        }
    });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
```
    
Currently implemented interfaces:
 - `IntellivibeData` (FalconSharedMemoryArea)
 - `FlightData` (FalconIntellivibeSharedMemoryArea)
