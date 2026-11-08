//! Audio-only menu bar shell. No UI timer, device scanner, socket server or updater.
mod audio;
mod localization;
mod logging;

use audio::AudioService;
use localization::Language;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{define_class, msg_send, sel, DefinedClass, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSControlStateValueOff,
    NSControlStateValueOn, NSImage, NSMenu, NSMenuDelegate, NSMenuItem, NSSquareStatusItemLength,
    NSStatusBar,
};
use objc2_foundation::{
    NSAutoreleasePool, NSLocale, NSNotification, NSObject, NSObjectProtocol, NSString,
    NSUserDefaults,
};
use std::cell::Cell;

const FOLLOW_KEY: &str = "FollowAudioOutput";
const LANGUAGE_KEY: &str = "MenuLanguage";

struct ControllerIvars {
    audio: AudioService,
    language: Cell<Language>,
    follow: Retained<NSMenuItem>,
    status: Retained<NSMenuItem>,
    language_menu: Retained<NSMenuItem>,
    languages: Vec<Retained<NSMenuItem>>,
    quit: Retained<NSMenuItem>,
}

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = objc2::MainThreadOnly]
    #[ivars = ControllerIvars]
    struct Controller;

    unsafe impl NSObjectProtocol for Controller {}

    unsafe impl NSApplicationDelegate for Controller {
        #[unsafe(method(applicationWillTerminate:))]
        fn application_will_terminate(&self, _notification: &NSNotification) {
            self.ivars().audio.shutdown();
        }
    }

    unsafe impl NSMenuDelegate for Controller {
        #[unsafe(method(menuWillOpen:))]
        fn menu_will_open(&self, _menu: &NSMenu) {
            self.refresh();
        }
    }

    impl Controller {
        #[unsafe(method(toggleFollow:))]
        fn toggle_follow(&self, _sender: Option<&AnyObject>) {
            let enabled = !self.ivars().audio.enabled();
            NSUserDefaults::standardUserDefaults().setBool_forKey(enabled, &NSString::from_str(FOLLOW_KEY));
            self.ivars().audio.set_enabled(enabled);
            self.refresh();
        }

        #[unsafe(method(selectLanguage:))]
        fn select_language(&self, sender: &NSMenuItem) {
            if let Some(language) = Language::ALL.get(sender.tag() as usize).copied() {
                self.ivars().language.set(language);
                // SAFETY: The value is an NSString, a valid property-list object.
                unsafe {
                    NSUserDefaults::standardUserDefaults().setObject_forKey(
                        Some(&NSString::from_str(language.code())), &NSString::from_str(LANGUAGE_KEY),
                    );
                }
                self.refresh();
            }
        }

        #[unsafe(method(quit:))]
        fn quit(&self, _sender: Option<&AnyObject>) {
            NSApplication::sharedApplication(self.mtm()).terminate(None);
        }
    }
);

impl Controller {
    fn new(mtm: MainThreadMarker, ivars: ControllerIvars) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(ivars);
        // SAFETY: Initialize our NSObject subclass after setting its Rust ivars.
        unsafe { msg_send![super(this), init] }
    }

    fn refresh(&self) {
        let items = self.ivars();
        let language = items.language.get();
        items
            .follow
            .setTitle(&NSString::from_str(&language.follow()));
        items.follow.setState(if items.audio.enabled() {
            NSControlStateValueOn
        } else {
            NSControlStateValueOff
        });
        items
            .status
            .setTitle(&NSString::from_str(&language.status(&items.audio.status())));
        items
            .language_menu
            .setTitle(&NSString::from_str(&language.language()));
        items.quit.setTitle(&NSString::from_str(&language.quit()));
        for (item, choice) in items.languages.iter().zip(Language::ALL) {
            item.setState(if language == choice {
                NSControlStateValueOn
            } else {
                NSControlStateValueOff
            });
        }
    }
}

fn main() {
    logging::init();
    let mtm = MainThreadMarker::new().expect("SleepAudioFix must run on the main thread");
    // SAFETY: AppKit setup and the application event loop stay on the main thread.
    unsafe {
        let _pool = NSAutoreleasePool::new();
        let defaults = NSUserDefaults::standardUserDefaults();
        let key = NSString::from_str(FOLLOW_KEY);
        let enabled = defaults.objectForKey(&key).is_none() || defaults.boolForKey(&key);
        let language = defaults
            .stringForKey(&NSString::from_str(LANGUAGE_KEY))
            .or_else(|| NSLocale::preferredLanguages().firstObject())
            .map(|s| Language::from_code(&s.to_string()))
            .unwrap_or(Language::English);
        let audio = AudioService::start(enabled);
        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
        let menu = NSMenu::new(mtm);
        menu.setAutoenablesItems(false);
        let empty = NSString::new();
        let follow =
            menu.addItemWithTitle_action_keyEquivalent(&empty, Some(sel!(toggleFollow:)), &empty);
        let status = menu.addItemWithTitle_action_keyEquivalent(&empty, None, &empty);
        status.setEnabled(false);
        menu.addItem(&NSMenuItem::separatorItem(mtm));
        let language_menu = menu.addItemWithTitle_action_keyEquivalent(&empty, None, &empty);
        let submenu = NSMenu::new(mtm);
        submenu.setAutoenablesItems(false);
        let languages: Vec<_> = Language::ALL
            .iter()
            .enumerate()
            .map(|(index, language)| {
                let item = submenu.addItemWithTitle_action_keyEquivalent(
                    &NSString::from_str(language.name()),
                    Some(sel!(selectLanguage:)),
                    &empty,
                );
                item.setTag(index as isize);
                item
            })
            .collect();
        language_menu.setSubmenu(Some(&submenu));
        menu.addItem(&NSMenuItem::separatorItem(mtm));
        let quit = menu.addItemWithTitle_action_keyEquivalent(
            &empty,
            Some(sel!(quit:)),
            &NSString::from_str("q"),
        );
        let controller = Controller::new(
            mtm,
            ControllerIvars {
                audio: audio.clone(),
                language: Cell::new(language),
                follow,
                status,
                language_menu,
                languages,
                quit,
            },
        );
        for item in [&controller.ivars().follow, &controller.ivars().quit]
            .into_iter()
            .chain(controller.ivars().languages.iter())
        {
            item.setTarget(Some(controller.as_ref()));
        }
        menu.setDelegate(Some(ProtocolObject::from_ref(&*controller)));
        app.setDelegate(Some(ProtocolObject::from_ref(&*controller)));
        controller.refresh();
        let bar = NSStatusBar::systemStatusBar();
        let status_item = bar.statusItemWithLength(NSSquareStatusItemLength);
        if let Some(button) = status_item.button(mtm) {
            if let Some(image) = NSImage::imageWithSystemSymbolName_accessibilityDescription(
                &NSString::from_str("speaker.wave.2.fill"),
                Some(&NSString::from_str("Crossover Sleep-Audio-fix")),
            ) {
                image.setTemplate(true);
                button.setImage(Some(&image));
            } else {
                button.setTitle(&NSString::from_str("Audio"));
            }
        }
        status_item.setMenu(Some(&menu));
        log::info!(
            "Crossover Sleep-Audio-fix started; language={}",
            language.code()
        );
        app.run();
        audio.shutdown();
        menu.setDelegate(None);
        app.setDelegate(None);
        bar.removeStatusItem(&status_item);
    }
}
