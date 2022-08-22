use event_derive::Event;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::Callback;

/// A trait that makes a struct usable as an event;
pub trait Event: Debug {
    /// Returns a string representing the event type of the event.
    /// This should be passed to addEventListener as the first argument.
    fn get_event_type(&self) -> &str;

    /// Returns an `Option<String>` representing the value of a DOM
    /// attribute. `None` indicates a boolean attribute, such as `disabled`,
    /// which has no value.
    fn get_callback(&self) -> Rc<dyn Fn(&web_sys::Event)>;
}

impl PartialEq for dyn Event {
    fn eq(&self, other: &Self) -> bool {
        self.get_event_type() == other.get_event_type()
            && Rc::ptr_eq(&self.get_callback(), &other.get_callback())
    }
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

/// A helper macro to build the structs that represent events as unique types
macro_rules! gen_event_structs {
    ($const_type:ident: $struct_name:ident, $($rest:tt)*) => {
        gen_event_structs!($const_type: $struct_name);
        gen_event_structs!($const_type: $($rest)*);
    };

    ($const_type:ident: $struct_name:ident) => {
        #[derive(Event)]
        #[event_type(web_sys::$const_type)]
        pub struct $struct_name(Rc<dyn Fn(&web_sys::Event)>);

        impl Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    .field("0", &String::from("a callback function"))
                    .finish()
            }
        }

        #[cfg(feature = "yew")]
        impl From<Callback<web_sys::$const_type>> for $struct_name {
            fn from(cb: Callback<web_sys::$const_type>) -> Self {
                let func = move |e: &web_sys::Event| {
                    let event = e.clone();
                    cb.emit(event.dyn_into::<web_sys::$const_type>().unwrap());
                };
                Self(Rc::new(func))
            }
        }
    };
}

gen_event_structs!(
    Event: Abort,
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

gen_event_structs!(
    MouseEvent: AuxClick,
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

gen_event_structs!(FocusEvent: Blur, Focus, FocusIn, FocusOut, Submit);

gen_event_structs!(
    DragEvent: Drag,
    DragEnd,
    DragEnter,
    DragExit,
    DragLeave,
    DragOver,
    DragStart,
    Drop
);

gen_event_structs!(InputEvent: Input);

gen_event_structs!(KeyboardEvent: Keydown, KeyPress, KeyUp);

gen_event_structs!(ProgressEvent: LoadStart, Progress, LoadEnd);

gen_event_structs!(WheelEvent: Wheel);

gen_event_structs!(
    AnimationEvent: AnimationCancel,
    AnimationEnd,
    AnimationIteration,
    AnimationStart
);

gen_event_structs!(
    PointerEvent: GotPointerCapture,
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

gen_event_structs!(TouchEvent: TouchCancel, TouchEnd, TouchMove, TouchStart);

gen_event_structs!(
    TransitionEvent: TransitionCancel,
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

    fn get_callback(&self) -> Rc<dyn Fn(&web_sys::Event)> {
        self.callback.clone()
    }
}

impl PartialEq for CustomEvent {
    fn eq(&self, other: &Self) -> bool {
        self.event_type == other.event_type && Rc::ptr_eq(&self.callback, &other.callback)
    }
}

impl Debug for CustomEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomEvent")
            .field("event_type", &self.event_type)
            .field("callback", &String::from("a callback function"))
            .finish()
    }
}
