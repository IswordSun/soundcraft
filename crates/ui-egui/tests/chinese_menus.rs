//! The menu bar follows the system language: Simplified Chinese labels over the same English
//! paths, so a Chinese machine shows Chinese menus and scripts dispatch exactly as before.

use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use soundcraft_ui_egui::{Services, SoundApp, i18n, menus};

fn harness() -> Harness<'static, SoundApp> {
    let app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    let mut h = Harness::builder().with_size(vec2(1600.0, 1000.0)).build_ui_state(
        |ui, app: &mut SoundApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    h.run_steps(6);
    h
}

#[test]
fn chinese_menus_render_and_keep_english_dispatch_paths() {
    i18n::set(i18n::Language::SimplifiedChinese);
    let mut h = harness();
    for root in ["文件", "编辑", "轨道", "片段", "事件", "AudioSuite", "选项", "设置", "视图", "窗口"] {
        assert!(h.query_by_label(root).is_some(), "menu bar is missing {root}");
    }

    h.get_by_label("文件").click();
    h.run_steps(4);
    // Menu items carry their shortcut in the accessibility label ("保存工程 Ctrl+S").
    assert!(h.query_by_label_contains("保存工程").is_some(), "File menu is not translated");
    assert!(h.query_by_label_contains("新建…").is_some(), "File ▸ New is not translated");
    assert!(h.query_by_label_contains("Save Session").is_none(), "English menu item is still drawn");

    // The label changed, the path did not: the same id is found from the English menu path.
    let extra = menus::ui_aliases();
    assert_eq!(menus::command_for_path("File > Save Session", &extra).as_deref(), Some("session.save"));
    assert_eq!(menus::command_for_path("File > Import > Audio...", &extra).as_deref(), Some("file.import_audio"));
}

#[test]
fn undo_and_redo_labels_follow_the_language() {
    i18n::set(i18n::Language::SimplifiedChinese);
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    assert_eq!(menus::live_label(&app, Some("edit.undo")).as_deref(), Some("无法撤销"));
    app.run("mix.mute", serde_json::json!({"track": "Kick"})).unwrap();
    assert_eq!(menus::live_label(&app, Some("edit.undo")).as_deref(), Some("撤销 Mute"));
    i18n::set(i18n::Language::English);
    assert_eq!(menus::live_label(&app, Some("edit.undo")).as_deref(), Some("Undo Mute"));
}

#[test]
fn language_parsing_covers_the_documented_overrides() {
    // The detection itself is unit-tested; this pins the documented spelling of the override.
    assert_eq!(i18n::Language::parse("zh-Hans"), Some(i18n::Language::SimplifiedChinese));
    assert_eq!(i18n::Language::parse("en"), Some(i18n::Language::English));
    assert_eq!(i18n::Language::from_locale("zh-Hans-CN"), i18n::Language::SimplifiedChinese);
    assert_eq!(i18n::Language::from_locale("en-US"), i18n::Language::English);
}
