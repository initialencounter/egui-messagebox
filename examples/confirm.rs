use egui_messagebox::{confirm, DialogParams};
#[tokio::main]
async fn main() {
    // let icon_data = egui_messagebox::load_icon_from_path(r"..\Ziafp\resources\rocket_launch_48dp_EA3323_FILL0_wght400_GRAD0_opsz48.png");
    // let icon_data_bytes = std::fs::read(r"..\Ziafp\resources\rocket_launch_48dp_EA3323_FILL0_wght400_GRAD0_opsz48.png").unwrap();
    // let icon_data= egui_messagebox::load_icon_from_bytes(&icon_data_bytes);
    let icon_data = egui_messagebox::load_icon_from_url("https://q1.qlogo.cn/g?b=qq&nk=3118087750&s=640").await;
    let dialog_prams = DialogParams::create(
        Some(icon_data),
        Some("None".to_string()),
        Some("None".to_string()),
        Some([320.0, 320.0]),
        Some("Confirm".to_string()),
        Some("Cancel".to_string()),
    );
    let result = confirm(dialog_prams).await;
    println!("result: {}", result);
}