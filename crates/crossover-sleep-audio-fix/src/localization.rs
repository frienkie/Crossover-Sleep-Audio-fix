//! Menu localization; values are persisted independently of audio preferences.
use crate::audio::Status;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
    Japanese,
}

impl Language {
    pub const ALL: [Self; 3] = [Self::English, Self::Chinese, Self::Japanese];

    pub fn from_code(code: &str) -> Self {
        match code.split(['-', '_']).next().unwrap_or("") {
            "zh" => Self::Chinese,
            "ja" => Self::Japanese,
            _ => Self::English,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Chinese => "zh",
            Self::Japanese => "ja",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "简体中文",
            Self::Japanese => "日本語",
        }
    }

    pub fn text(self, english: &str, chinese: &str, japanese: &str) -> String {
        match self {
            Self::English => english,
            Self::Chinese => chinese,
            Self::Japanese => japanese,
        }
        .into()
    }

    pub fn follow(self) -> String {
        self.text(
            "Follow macOS Audio Output",
            "跟随 macOS 音频输出",
            "macOS の音声出力に追従",
        )
    }

    pub fn language(self) -> String {
        self.text("Language", "语言", "言語")
    }

    pub fn quit(self) -> String {
        self.text("Quit", "退出", "終了")
    }

    pub fn status(self, status: &Status) -> String {
        match status {
            Status::Off => self.text("Audio following off", "音频跟随已关闭", "音声の追従はオフ"),
            Status::Starting => self.text("Starting…", "正在启动…", "起動中…"),
            Status::Stopping => self.text(
                "Restoring original output…",
                "正在恢复原始输出…",
                "元の出力に戻しています…",
            ),
            Status::Waiting(output) if !output.is_empty() => {
                format!("{} · {output}", self.text("Monitoring", "监控中", "監視中"))
            }
            Status::Waiting(_) => self.text(
                "Waiting for an output device",
                "等待音频输出设备",
                "音声出力デバイスを待機中",
            ),
            Status::Connecting(output) => format!(
                "{} · {output}",
                self.text(
                    "Connecting — check audio permission",
                    "正在连接，请检查音频录制权限",
                    "接続中 — 音声収録の許可を確認"
                )
            ),
            Status::Routing { output, .. } => {
                format!("{} · {output}", self.text("Rerouting", "转发中", "転送中"))
            }
            Status::Retrying => self.text(
                "Retrying — check audio permission",
                "正在重试，请检查音频录制权限",
                "再試行中 — 音声収録の許可を確認",
            ),
            Status::AnotherInstance => self.text(
                "Handled by another Crossover Sleep-Audio-fix",
                "另一个 Crossover Sleep-Audio-fix 正在处理音频",
                "別の Crossover Sleep-Audio-fix が音声を処理中",
            ),
            Status::MissingHelper => self.text(
                "Audio helper missing — reinstall",
                "音频助手缺失，请重新安装",
                "音声ヘルパーがありません — 再インストールしてください",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preferences_and_system_language_codes_round_trip() {
        assert_eq!(Language::from_code("zh-Hans-JP"), Language::Chinese);
        assert_eq!(Language::from_code("ja_JP"), Language::Japanese);
        assert_eq!(Language::from_code("fr-FR"), Language::English);
        for language in Language::ALL {
            assert_eq!(Language::from_code(language.code()), language);
        }
    }

    #[test]
    fn monitoring_and_rerouting_are_distinct_in_every_language() {
        for language in Language::ALL {
            let waiting = language.status(&Status::Waiting("Test Output".into()));
            let routing = language.status(&Status::Routing {
                output: "Test Output".into(),
                processes: 2,
            });
            assert!(waiting.contains("Test Output") && routing.contains("Test Output"));
            assert_ne!(waiting, routing);
            for status in [
                Status::Off,
                Status::Starting,
                Status::Stopping,
                Status::Waiting(String::new()),
                Status::Connecting("Test Output".into()),
                Status::Retrying,
                Status::AnotherInstance,
                Status::MissingHelper,
            ] {
                assert!(!language.status(&status).is_empty());
            }
        }
    }
}
