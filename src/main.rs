slint::include_modules!();
use sysinfo::{System, Disks, Networks, Components};
use std::rc::Rc;
use chrono::Local;

fn main() -> Result<(), slint::PlatformError> {

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let mut sys = System::new_all();
    let mut net = Networks::new_with_refreshed_list();
    let mut last_net_rx = 0u64;

    // 初始抓取 CPU 型號 (只需抓一次，或在 timer 裡更新也可以)
    sys.refresh_cpu(); // 確保有數據
    let cpu_brand = sys.cpus().first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown Processor".to_string());
    ui.set_cpu_model(cpu_brand.into());

    let timer = slint::Timer::default();
    let mut last_valid_temp = 0.0f32; //

    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(1000), {
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();

            // 刷新所有數據
            sys.refresh_all();
            net.refresh();
            let disks_info = Disks::new_with_refreshed_list();
            let cpus = sys.cpus();
            ui.set_cpu_count(cpus.len() as i32);

            // 1. CPU 核心列表 (32 Threads)
            let cpu_vec: Vec<CpuData> = sys.cpus().iter().enumerate().map(|(i, c)| {
                CpuData {
                    name: format!("C{:02}", i).into(),
                    usage: c.cpu_usage(),
                    freq: format!("{:.0}MHz", c.frequency()).into(),
                    is_high: c.cpu_usage() > 80.0,
                }
            }).collect();
            ui.set_cpus(Rc::new(slint::VecModel::from(cpu_vec)).into());

            // 2. 儲存裝置
            let disk_vec: Vec<DiskData> = disks_info.iter().map(|d| {
                let total = d.total_space() as f64 / 1_073_741_824.0;
                let used = (d.total_space() - d.available_space()) as f64 / 1_073_741_824.0;
                DiskData {
                    mount: d.mount_point().to_string_lossy().to_string().into(),
                    total: total as f32,
                    used: used as f32,
                    percent: (used / total * 100.0) as f32,
                }
            }).collect();
            ui.set_disks(Rc::new(slint::VecModel::from(disk_vec)).into());

            // 3. 記憶體
            let total_mem = sys.total_memory() as f64 / 1_073_741_824.0;
            let used_mem = sys.used_memory() as f64 / 1_073_741_824.0;
            ui.set_mem_label(format!("{:.1} / {:.1} GB ({:.1}%)",
                                     used_mem, total_mem, (used_mem / total_mem) * 100.0).into());

            // 4. 狀態列資訊
            ui.set_proc_count(sys.processes().len().to_string().into());
            ui.set_last_update(Local::now().format("%H:%M:%S").to_string().into());

            let up = System::uptime();
            ui.set_uptime(format!("{}h {}m", up / 3600, (up % 3600) / 60).into());

            // 5. 溫度監測優化 (針對 AMD 閒置 0.0 進行處理)
            let components = Components::new_with_refreshed_list();
            let current_temp = components.iter()
                .find(|c| {
                    let l = c.label().to_uppercase();
                    l.contains("TCTL") || l.contains("TDIE") || l.contains("CPU")
                })
                .map(|c| c.temperature())
                .unwrap_or(0.0);

            // 如果目前抓到 0.0 且之前有有效數值，則使用舊數值；否則更新
            if current_temp > 0.0 {
                last_valid_temp = current_temp;
            }

            // 只有在真的有數值時才更新 UI，否則維持上次內容
            if last_valid_temp > 0.0 {
                ui.set_cpu_temp(format!("{:.1}°C", last_valid_temp).into());
            } else {
                ui.set_cpu_temp("Loading...".into());
            }

            // 6. 網路 (下載速度計算)
            let current_rx: u64 = net.iter().map(|(_, d)| d.received()).sum();
            let speed = (current_rx.saturating_sub(last_net_rx)) as f64 / 1024.0;
            ui.set_net_label(format!("Download: {:.1} KB/s", speed).into());
            last_net_rx = current_rx;
        }
    });

    ui.on_refresh_clicked({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(_ui) = ui_handle.upgrade() {
                // 這裡可以手動觸發一次數據抓取邏輯，或者乾脆留空讓 timer 處理
                println!("手動觸發刷新");
            }
        }
    });

    // 2. 系統資訊回調
    ui.on_info_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.upgrade().unwrap();
            let os_name = System::name().unwrap_or_else(|| "Unknown".into());
            let os_ver = System::os_version().unwrap_or_else(|| "Unknown".into());
            let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".into());
            let host = System::host_name().unwrap_or_else(|| "Unknown".into());

            ui.set_dialog_title("作業系統詳細資訊".into());

            let entries = vec![
                InfoEntry { label: "主機名稱".into(), value: host.into() },
                InfoEntry { label: "系統發行".into(), value: os_name.into() },
                InfoEntry { label: "系統版本".into(), value: os_ver.into() },
                InfoEntry { label: "內核版本".into(), value: kernel.into() },
            ];

            ui.set_info_entries(Rc::new(slint::VecModel::from(entries)).into());
            ui.invoke_show_msg_box();
        }
    });

    // 3. 關於回調 (保持填充 entries)
    ui.on_about_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.upgrade().unwrap();
            ui.set_dialog_title("關於 SysInfoPro".into());

            let entries = vec![
                InfoEntry { label: "版本".into(), value: "v0.0.1".into() },
                InfoEntry { label: "處理器".into(), value: "Ryzen 9 5900XT".into() },
                InfoEntry { label: "技術".into(), value: "Rust & Slint".into() },
            ];

            ui.set_info_entries(Rc::new(slint::VecModel::from(entries)).into());
            ui.invoke_show_msg_box();
        }
    });

    // 4. 結束回調 (新增)
    ui.on_exit_clicked(|| {
        println!("正在關閉 SysInfoPro...");
        slint::quit_event_loop().unwrap();
    });

    ui.run()
}