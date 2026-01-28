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
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 解析参数
    let mut bin_path: Option<String> = None;
    let mut vpm_filter: Option<u64> = None;
    let mut script_index_arg: Option<usize> = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--vpm" => {
                if i + 1 >= args.len() {
                    eprintln!("错误: --vpm 需要一个参数");
                    std::process::exit(1);
                }
                let vpm_str = &args[i + 1].replace("_", "");
                vpm_filter = Some(
                    u64::from_str_radix(vpm_str.trim_start_matches("0x"), 16)
                        .context("无效的VPM值，应为十六进制数")?
                );
                i += 2;
            }
            "--index" => {
                if i + 1 >= args.len() {
                    eprintln!("错误: --index 需要一个参数");
                    std::process::exit(1);
                }
                script_index_arg = Some(
                    args[i + 1].parse()
                        .context("无效的index值，应为数字")?
                );
                i += 2;
            }
            _ => {
                if bin_path.is_none() {
                    bin_path = Some(args[i].clone());
                } else {
                    eprintln!("错误: 未知参数或多余的参数: {}", args[i]);
                    std::process::exit(1);
                }
                i += 1;
            }
        }
    }
    
    let bin_path = match bin_path {
        Some(path) => path,
        None => {
            eprintln!("用法: {} <bin文件路径> [--vpm <VPM值>] [--index <脚本索引>]", args[0]);
            eprintln!("示例: {} program.bin --vpm 0x8089000900014 --index 0", args[0]);
            std::process::exit(1);
        }
    };
    
    // 读取 .bin 文件
    let bin_data = fs::read(&bin_path)
        .context(format!("无法读取文件: {}", bin_path))?;
    
    println!("已读取文件: {} ({} 字节)", bin_path, bin_data.len());

    // VPM示例: VPM = VID (2字节) | PID (2字节) | ModelCode (2字节)
    // 例如: 0x8089_0009_0014
    // VID = 0x8089, PID = 0x0009, ModelCode = 0x0014
    
    // 初始化设备API
    println!("\n正在初始化设备API...");
    init_sayo_device().await;
    
    // 获取设备列表
    println!("正在获取设备列表...");
    let devices = get_device_list().await;
    
    if devices.is_empty() {
        bail!("未找到任何设备");
    }

    // 如果提供了VPM参数，进行设备筛选
    let selected_device = if let Some(vpm) = vpm_filter {
        println!("\n使用VPM筛选设备: 0x{:012X}", vpm);
        
        let vid: u16 = (vpm >> 32) as u16;
        let pid: u16 = ((vpm & 0xFFFFFFFF) >> 16) as u16;
        let model_code: u16 = (vpm & 0xFFFF) as u16;
        
        println!("VID: 0x{:04X}, PID: 0x{:04X}, ModelCode: 0x{:04X}", vid, pid, model_code);
        
        // 查找匹配的设备
        let mut matched_device = None;
        for device in &devices {
            let dev_vid = device.vid();
            let dev_pid = device.pid();
            let dev_model_code = match device.get_device_info().await {
                Some(info) => info.model_code(None),
                None => None,
            };
            println!(
                "检查设备: {:?}, VID: 0x{:04X}, PID: 0x{:04X}, ModelCode: {:?}",
                device,
                dev_vid,
                dev_pid,
                dev_model_code.map(|mc| format!("0x{:04X}", mc)).unwrap_or("None".to_string())
            );
            
            if dev_vid == vid && dev_pid == pid {
                if let Some(dev_mc) = dev_model_code {
                    if dev_mc == model_code {
                        matched_device = Some(device);
                        println!("✓ 找到匹配的设备: {:?}", device);
                        break;
                    }
                }
            }
        }
        
        match matched_device {
            Some(device) => device,
            None => {
                bail!("未找到匹配VPM 0x{:012X} 的设备", vpm);
            }
        }
    } else {
        // 显示设备列表，让用户选择
        println!("\n找到 {} 个设备:", devices.len());
        for (i, device) in devices.iter().enumerate() {
            println!("[{}] 设备: {:?}", i, device);
        }

        // 用户选择设备
        print!("\n请选择设备 (0-{}): ", devices.len() - 1);
        io::stdout().flush()?;
        
        let mut device_input = String::new();
        io::stdin().read_line(&mut device_input)?;
        let device_index: usize = device_input.trim().parse()
            .context("无效的设备索引")?;
        
        if device_index >= devices.len() {
            bail!("设备索引超出范围");
        }

        &devices[device_index]
    };

    println!("\n已选择设备: {:?}", selected_device);

    // 获取脚本名称列表（用于确定设备支持的脚本数量）
    let script_names = selected_device.get_script_names().await;
    
    let max_scripts = script_names.len();
    println!("\n设备支持的脚本数量: {}", max_scripts);

    // 获取脚本长度限制
    let script_max_len = selected_device.get_script_address_len(0).await;
    
    println!("脚本最大长度: {} 字节", script_max_len);

    // 检查文件大小是否超出限制
    if bin_data.len() > script_max_len as usize {
        bail!(
            "文件过大！文件大小: {} 字节，设备限制: {} 字节",
            bin_data.len(),
            script_max_len
        );
    }

    // 显示脚本槽信息
    println!("\n可用的脚本槽: 0 到 {}", max_scripts - 1);
    
    // 确定脚本索引
    let script_index = if let Some(index) = script_index_arg {
        println!("使用指定的脚本索引: {}", index);
        
        if index >= max_scripts {
            bail!("脚本索引 {} 超出范围，设备最大支持索引为 {}", index, max_scripts - 1);
        }
        
        index
    } else {
        // 用户选择脚本索引
        print!("请选择要上传到的脚本索引 (0-{}): ", max_scripts - 1);
        io::stdout().flush()?;
        
        let mut script_input = String::new();
        io::stdin().read_line(&mut script_input)?;
        let index: usize = script_input.trim().parse()
            .context("无效的脚本索引")?;
        
        if index >= max_scripts {
            bail!("脚本索引超出范围");
        }
        
        index
    };

    // 上传脚本
    println!("\n正在上传脚本到索引 {}...", script_index);
    
    // 创建脚本内容对象，包含二进制数据
    let script_content = SayoScriptContent {
        bytes: RwBytes::new(bin_data.clone()),
    };
    
    // set_script(index, content, address, on_progress)
    // on_progress 是一个进度回调函数
    let result = selected_device.set_script(
        script_index as u8, 
        &script_content, 
        0,
        |progress| {
            Box::pin(async move {
                print!("\r上传进度: {:.1}%", progress * 100.0);
                io::stdout().flush().ok();
                true // 返回 true 继续上传
            })
        }
    ).await;
    
    println!(); // 换行
    
    if result {
        selected_device.save_all().await;
        println!("✓ 脚本上传成功！");
    } else {
        bail!("脚本上传失败");
    }

    Ok(())
}
