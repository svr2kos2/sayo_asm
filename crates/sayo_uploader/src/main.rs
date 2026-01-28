use anyhow::{Context, Result, bail};
use sayo_api_rs::device::get_device_list;
use sayo_api_rs::device::init_sayo_device;
use sayo_api_rs::structures::SayoScriptContent;
use sayo_api_rs::byte_converter::RwBytes;
use std::env;
use std::fs;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Parse arguments
    let mut bin_path: Option<String> = None;
    let mut vpm_filter: Option<u64> = None;
    let mut script_index_arg: Option<usize> = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--vpm" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --vpm requires an argument");
                    std::process::exit(1);
                }
                let vpm_str = &args[i + 1].replace("_", "");
                vpm_filter = Some(
                    u64::from_str_radix(vpm_str.trim_start_matches("0x"), 16)
                        .context("Invalid VPM value, should be hexadecimal")?
                );
                i += 2;
            }
            "--index" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --index requires an argument");
                    std::process::exit(1);
                }
                script_index_arg = Some(
                    args[i + 1].parse()
                        .context("Invalid index value, should be a number")?
                );
                i += 2;
            }
            _ => {
                if bin_path.is_none() {
                    bin_path = Some(args[i].clone());
                } else {
                    eprintln!("Error: Unknown or extra parameter: {}", args[i]);
                    std::process::exit(1);
                }
                i += 1;
            }
        }
    }
    
    let bin_path = match bin_path {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <bin file path> [--vpm <VPM value>] [--index <script index>]", args[0]);
            eprintln!("Example: {} program.bin --vpm 0x8089000900014 --index 0", args[0]);
            std::process::exit(1);
        }
    };
    
    // Read .bin file
    let bin_data = fs::read(&bin_path)
        .context(format!("Unable to read file: {}", bin_path))?;
    
    println!("Read file: {} ({} bytes)", bin_path, bin_data.len());

    // VPM example: VPM = VID (2 bytes) | PID (2 bytes) | ModelCode (2 bytes)
    // For example: 0x8089_0009_0014
    // VID = 0x8089, PID = 0x0009, ModelCode = 0x0014
    
    // Initialize device API
    println!("\nInitializing device API...");
    init_sayo_device().await;
    
    // Get device list
    println!("Getting device list...");
    let devices = get_device_list().await;
    
    if devices.is_empty() {
        bail!("No devices found");
    }

    // If VPM parameter is provided, filter devices
    let selected_device = if let Some(vpm) = vpm_filter {
        println!("\nFiltering devices by VPM: 0x{:012X}", vpm);
        
        let vid: u16 = (vpm >> 32) as u16;
        let pid: u16 = ((vpm & 0xFFFFFFFF) >> 16) as u16;
        let model_code: u16 = (vpm & 0xFFFF) as u16;
        
        println!("VID: 0x{:04X}, PID: 0x{:04X}, ModelCode: 0x{:04X}", vid, pid, model_code);
        
        // Find matching device
        let mut matched_device = None;
        for device in &devices {
            let dev_vid = device.vid();
            let dev_pid = device.pid();
            let dev_model_code = match device.get_device_info().await {
                Some(info) => info.model_code(None),
                None => None,
            };
            println!(
                "Checking device: {:?}, VID: 0x{:04X}, PID: 0x{:04X}, ModelCode: {:?}",
                device,
                dev_vid,
                dev_pid,
                dev_model_code.map(|mc| format!("0x{:04X}", mc)).unwrap_or("None".to_string())
            );
            
            if dev_vid == vid && dev_pid == pid {
                if let Some(dev_mc) = dev_model_code {
                    if dev_mc == model_code {
                        matched_device = Some(device);
                        println!("✓ Found matching device: {:?}", device);
                        break;
                    }
                }
            }
        }
        
        match matched_device {
            Some(device) => device,
            None => {
                bail!("No device matching VPM 0x{:012X} found", vpm);
            }
        }
    } else {
        // Display device list and let user select
        println!("\nFound {} device(s):", devices.len());
        for (i, device) in devices.iter().enumerate() {
            println!("[{}] Device: {:?}", i, device);
        }

        // User selects device
        print!("\nPlease select device (0-{}): ", devices.len() - 1);
        io::stdout().flush()?;
        
        let mut device_input = String::new();
        io::stdin().read_line(&mut device_input)?;
        let device_index: usize = device_input.trim().parse()
            .context("Invalid device index")?;
        
        if device_index >= devices.len() {
            bail!("Device index out of range");
        }

        &devices[device_index]
    };

    println!("\nSelected device: {:?}", selected_device);

    // Get script name list (to determine the number of scripts supported by the device)
    let script_names = selected_device.get_script_names().await;
    
    let max_scripts = script_names.len();
    println!("\nNumber of scripts supported by device: {}", max_scripts);

    // Get script length limit
    let script_max_len = selected_device.get_script_address_len(0).await;
    
    println!("Script maximum length: {} bytes", script_max_len);

    // Check if file size exceeds the limit
    if bin_data.len() > script_max_len as usize {
        bail!(
            "File too large! File size: {} bytes, device limit: {} bytes",
            bin_data.len(),
            script_max_len
        );
    }

    // Display script slot information
    println!("\nAvailable script slots: 0 to {}", max_scripts - 1);
    
    // Determine script index
    let script_index = if let Some(index) = script_index_arg {
        println!("Using specified script index: {}", index);
        
        if index >= max_scripts {
            bail!("Script index {} out of range, device max supported index is {}", index, max_scripts - 1);
        }
        
        index
    } else {
        // User selects script index
        print!("Please select the script index to upload to (0-{}): ", max_scripts - 1);
        io::stdout().flush()?;
        
        let mut script_input = String::new();
        io::stdin().read_line(&mut script_input)?;
        let index: usize = script_input.trim().parse()
            .context("Invalid script index")?;
        
        if index >= max_scripts {
            bail!("Script index out of range");
        }
        
        index
    };

    // Upload script
    println!("\nUploading script to index {}...", script_index);
    
    // Create script content object with binary data
    let script_content = SayoScriptContent {
        bytes: RwBytes::new(bin_data.clone()),
    };
    
    // set_script(index, content, address, on_progress)
    // on_progress is a progress callback function
    let result = selected_device.set_script(
        script_index as u8, 
        &script_content, 
        0,
        |progress| {
            Box::pin(async move {
                print!("\rUpload progress: {:.1}%", progress * 100.0);
                io::stdout().flush().ok();
                true // Return true to continue uploading
            })
        }
    ).await;
    
    println!(); // 换行
    
    if result {
        selected_device.save_all().await;
        println!("✓ Script uploaded successfully!");
    } else {
        bail!("Script upload failed");
    }

    Ok(())
}
