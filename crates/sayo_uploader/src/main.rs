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
    if args.len() != 2 {
        eprintln!("用法: {} <bin文件路径>", args[0]);
        std::process::exit(1);
    }

    let bin_path = &args[1];
    
    // 读取 .bin 文件
    let bin_data = fs::read(bin_path)
        .context(format!("无法读取文件: {}", bin_path))?;
    
    println!("已读取文件: {} ({} 字节)", bin_path, bin_data.len());

    // 初始化设备API
    println!("\n正在初始化设备API...");
    init_sayo_device().await;
    
    // 获取设备列表
    println!("正在获取设备列表...");
    let devices = get_device_list().await;
    
    if devices.is_empty() {
        bail!("未找到任何设备");
    }

    // 显示设备列表
    println!("\n找到 {} 个设备:", devices.len());
    for (i, device) in devices.iter().enumerate() {
        println!("[{}] 设备: {:?}", i, device);
    }

    // 用户选择设备
    print!("\n请选择设备 (0-{}): ", devices.len() - 1);
    io::stdout().flush()?;
    
    let mut device_input = "0";//String::new();
    // io::stdin().read_line(&mut device_input)?;
    // delay 100ms
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let device_index: usize = device_input.trim().parse()
        .context("无效的设备索引")?;
    
    if device_index >= devices.len() {
        bail!("设备索引超出范围");
    }

    let selected_device = &devices[device_index];
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
    
    // 用户选择脚本索引
    print!("请选择要上传到的脚本索引 (0-{}): ", max_scripts - 1);
    io::stdout().flush()?;
    
    let mut script_input = "0";//String::new();
    //io::stdin().read_line(&mut script_input)?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let script_index: usize = script_input.trim().parse()
        .context("无效的脚本索引")?;
    
    if script_index >= max_scripts {
        bail!("脚本索引超出范围");
    }

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
