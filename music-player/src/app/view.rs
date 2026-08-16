use crate::app::{CosmicAppletMusic, Message};
use cosmic::widget::Id;
use cosmic::Element;
use mpris::PlaybackStatus;
use std::sync::LazyLock;

pub mod view_window;

static AUTOSIZE_MAIN_ID: LazyLock<Id> = LazyLock::new(|| Id::new("autosize-main"));

pub enum AppIcon {
    Playing,
    Paused,
}

impl AppIcon {
    fn to_str(&self) -> &'static str {
        match self {
            AppIcon::Playing => "media-playback-start-symbolic",
            AppIcon::Paused => "media-playback-pause-symbolic",
        }
    }
}

pub fn resolve_playback_icon(
    config_manager: Option<&crate::config::ConfigManager>,
    icon: &AppIcon,
) -> cosmic::widget::icon::Handle {
    let custom_path = config_manager.and_then(|config| match icon {
        AppIcon::Playing => config.get_custom_play_icon(),
        AppIcon::Paused => config.get_custom_pause_icon(),
    });

    match custom_path {
        Some(path) if !path.is_empty() => {
            cosmic::widget::icon::from_path(std::path::PathBuf::from(path))
        }
        _ => cosmic::widget::icon::from_name(icon.to_str())
            .symbolic(true)
            .into(),
    }
}

pub fn view(app: &CosmicAppletMusic) -> Element<'_, Message> {
    // Check if in multi-player mode
    let show_all_players = app
        .config_manager
        .as_ref()
        .map(|config| config.get_show_all_players())
        .unwrap_or(false);

    let icon = if show_all_players {
        // In multi-player mode, check if ANY player is playing
        let any_playing = app
            .all_players_info
            .iter()
            .any(|p| p.status == PlaybackStatus::Playing);

        if any_playing {
            AppIcon::Paused // Show pause when any player is playing
        } else {
            AppIcon::Playing // Show play when nothing is playing
        }
    } else {
        // Single-player mode
        match app.player_info.status {
            PlaybackStatus::Playing => AppIcon::Paused, // Show pause when playing
            PlaybackStatus::Paused => AppIcon::Playing, // Show play when paused
            PlaybackStatus::Stopped => AppIcon::Playing, // Show play when stopped
        }
    };

    use cosmic::iced::mouse;

    let icon_handle = resolve_playback_icon(app.config_manager.as_ref(), &icon);

    cosmic::widget::autosize::autosize(
        cosmic::widget::mouse_area(
            app.core
                .applet
                .icon_button_from_handle(icon_handle)
                .on_press_down(Message::TogglePopup),
        )
        .on_scroll(|delta| match delta {
            mouse::ScrollDelta::Lines { y, .. } => {
                if y > 0.0 {
                    Message::ScrollUp
                } else {
                    Message::ScrollDown
                }
            }
            mouse::ScrollDelta::Pixels { y, .. } => {
                if y > 0.0 {
                    Message::ScrollUp
                } else {
                    Message::ScrollDown
                }
            }
        })
        .on_middle_press(Message::MiddleClick),
        AUTOSIZE_MAIN_ID.clone(),
    )
    .into()
}
