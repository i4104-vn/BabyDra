#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallChannel {
    Release,
    Develop,
    LocalSource,
}

impl InstallChannel {
    pub fn name(&self) -> &'static str {
        match self {
            InstallChannel::Release => "Release Channel (Official Stable)",
            InstallChannel::Develop => "Develop Channel (Community & Feature Builds)",
            InstallChannel::LocalSource => "Local Directory (Pre-built Binaries)",
        }
    }

    pub fn git_branch(&self) -> &'static str {
        match self {
            InstallChannel::Release => "release",
            InstallChannel::Develop => "develop",
            InstallChannel::LocalSource => "local",
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &'static str {
        match self {
            InstallChannel::Release => {
                "Cài đặt phiên bản chính thức ổn định được kiểm thử và phát hành bởi tác giả từ nhánh 'release'."
            }
            InstallChannel::Develop => {
                "Cài đặt phiên bản phát triển cộng đồng chứa các tính năng mới nhất từ nhánh 'develop'."
            }
            InstallChannel::LocalSource => {
                "Cài đặt trực tiếp từ thư mục nhị phân target/release có sẵn trên hệ thống cục bộ."
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BranchMetadata {
    pub channel: InstallChannel,
    pub branch_name: String,
    pub commit_hash: String,
    pub author_name: String,
    pub update_date: String,
    pub commit_msg: String,
}

impl Default for BranchMetadata {
    fn default() -> Self {
        Self {
            channel: InstallChannel::Release,
            branch_name: "release".into(),
            commit_hash: "N/A".into(),
            author_name: "N/A".into(),
            update_date: "N/A".into(),
            commit_msg: "Không có dữ liệu commit".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetProfile {
    FullDesktop,
    BinariesAndBundle,
    Custom,
}

impl PresetProfile {
    pub fn name(&self) -> &'static str {
        match self {
            PresetProfile::FullDesktop => "Full Desktop (Khuyến nghị)",
            PresetProfile::BinariesAndBundle => "Chỉ cài đặt Tệp nhị phân & /var/lib",
            PresetProfile::Custom => "Tùy chỉnh từng thành phần",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PresetProfile::FullDesktop => {
                "Cài đặt toàn bộ tệp nhị phân, staging /var/lib, cấu hình labwc, themes, We10X icons và greetd."
            }
            PresetProfile::BinariesAndBundle => {
                "Sao chép các tệp nhị phân vào ~/.local/bin và /var/lib/babydra. Bỏ qua dotfiles và greetd."
            }
            PresetProfile::Custom => {
                "Tùy chỉnh chọn lọc từng gói phần mềm, tệp nhị phân và cấu hình hệ thống."
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericOptionItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub detail: String,
    pub selected: bool,
    pub requires_root: bool,
}
