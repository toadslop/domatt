use event_derive::Event;
use std::rc::Rc;
use wasm_bindgen::JsCast;

/// A trait that makes a struct usable as an event;
pub trait Event {
    /// Returns a string representing the event type of the event.
    /// This should be passed to addEventListener as the first argument.
    fn get_event_type(&self) -> &str;

    /// Returns an `Option<String>` representing the value of a DOM
    /// attribute. `None` indicates a boolean attribute, such as `disabled`,
    /// which has no value.
    fn get_callback(&mut self) -> Rc<dyn Fn(&web_sys::Event)>;
}

// macro_rules! gen_event_traits {
//     ($trait_name:ident) => {
//         pub trait $trait_name: Event {}
//     };

//     ($trait_name:ident, $($next:tt)*) => {
//         pub trait $trait_name: Event {}

//         gen_event_traits!($($next)*);
//     }
// }

macro_rules! gen_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::Event)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::Event)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_event_structs!($($next)*);
    }
}

gen_event_structs!(
    Abort,
    Cancel,
    CanPlay,
    CanPlayThrough,
    Change,
    Close,
    CueChange,
    DurationChange,
    Emptied,
    Ended,
    Error,
    FormData,
    Invalid,
    Load,
    LoadedData,
    LoadedMetadata,
    Pause,
    Play,
    Playing,
    RateChange,
    Reset,
    Resize,
    Scroll,
    SecurityPolicyViolation,
    Seeked,
    Seeking,
    Select,
    SlotChange,
    Stalled,
    Suspend,
    TimeUpdate,
    Toggle,
    VolumeChange,
    Waiting,
    Copy,
    Cut,
    Paste,
    SelectionChange,
    SelectStart,
    Show,
    PointerLockChange,
    PointerLockError
);

macro_rules! gen_mouse_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::MouseEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::MouseEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_mouse_event_structs!($($next)*);
    }
}

gen_mouse_event_structs!(
    AuxClick,
    Click,
    ContextMenu,
    DblClick,
    MouseDown,
    MousEenter,
    MouseLeave,
    MouseMove,
    MouseOut,
    MouseOver,
    MouseUp
);

macro_rules! gen_focus_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::FocusEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::FocusEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_focus_event_structs!($($next)*);
    }
}

gen_focus_event_structs!(Blur, Focus, FocusIn, FocusOut, Submit);

macro_rules! gen_drag_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::DragEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::DragEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_drag_event_structs!($($next)*);
    }
}

gen_drag_event_structs!(Drag, DragEnd, DragEnter, DragExit, DragLeave, DragOver, DragStart, Drop);

#[derive(Event)]
#[event_type(web_sys::InputEvent)]
pub struct Input(Rc<dyn Fn(&web_sys::Event)>);

macro_rules! gen_keyboard_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::KeyboardEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::KeyboardEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_keyboard_event_structs!($($next)*);
    }
}

gen_keyboard_event_structs!(Keydown, KeyPress, KeyUp);

macro_rules! gen_progress_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::ProgressEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::ProgressEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_progress_event_structs!($($next)*);
    }
}

gen_progress_event_structs!(LoadStart, Progress, LoadEnd);

#[derive(Event)]
#[event_type(web_sys::WheelEvent)]
pub struct Wheel(Rc<dyn Fn(&web_sys::Event)>);

macro_rules! gen_animation_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::AnimationEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::AnimationEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_animation_event_structs!($($next)*);
    }
}

gen_animation_event_structs!(
    AnimationCancel,
    AnimationEnd,
    AnimationIteration,
    AnimationStart
);

macro_rules! gen_pointer_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::PointerEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::PointerEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_pointer_event_structs!($($next)*);
    }
}

gen_pointer_event_structs!(
    GotPointerCapture,
    LostPointerCapture,
    PointerCancel,
    PointerDown,
    PointerEnter,
    PointerLeave,
    PointerMove,
    PointerOut,
    PointerOver,
    PointerUp
);

macro_rules! gen_touch_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::TouchEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::TouchEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_touch_event_structs!($($next)*);
    }
}

gen_touch_event_structs!(TouchCancel, TouchEnd, TouchMove, TouchStart);

macro_rules! gen_transition_event_structs {
    ($struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::TransitionEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);
    };

    ($struct_name:ident, $($next:tt)*) => {
        #[derive(Event)]
        #[event_type(web_sys::TransitionEvent)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        gen_transition_event_structs!($($next)*);
    }
}

gen_transition_event_structs!(
    TransitionCancel,
    TransitionEnd,
    TransitionRun,
    TransitionStart
);

pub struct CustomEvent {
    event_type: &'static str,
    callback: Rc<dyn Fn(&web_sys::Event)>,
}

impl CustomEvent {
    pub fn get_event_type(&self) -> &'static str {
        self.event_type.clone()
    }

    pub fn get_callback(&self) -> Rc<dyn Fn(&web_sys::Event)> {
        self.callback.clone()
    }
}

impl CustomEvent {
    pub fn new(event_type: &'static str, callback: Rc<dyn Fn(&web_sys::Event)>) -> Self {
        Self {
            event_type,
            callback,
        }
    }
}

impl Event for CustomEvent {
    fn get_event_type(&self) -> &str {
        self.event_type
    }

    fn get_callback(&mut self) -> Rc<dyn Fn(&web_sys::Event)> {
        self.callback.clone()
    }
}
