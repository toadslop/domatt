use super::{
    Abort, AnimationCancel, AnimationEnd, AnimationIteration, AnimationStart, AuxClick, Blur,
    CanPlay, CanPlayThrough, Change, Click, Close, ContextMenu, Copy, CueChange, Cut, DblClick,
    Drag, DragEnd, DragEnter, DragExit, DragLeave, DragOver, DragStart, Drop, DurationChange,
    Emptied, Ended, Error, Event, Focus, FocusIn, FocusOut, FormData, GotPointerCapture, Input,
    Invalid, KeyPress, KeyUp, Keydown, Load, LoadEnd, LoadStart, LoadedData, LoadedMetadata,
    LostPointerCapture, MousEenter, MouseDown, MouseLeave, MouseMove, MouseOut, MouseOver, MouseUp,
    Paste, Pause, Play, Playing, PointerCancel, PointerDown, PointerEnter, PointerLeave,
    PointerLockChange, PointerLockError, PointerMove, PointerOut, PointerOver, PointerUp, Progress,
    RateChange, Reset, Resize, Scroll, SecurityPolicyViolation, Seeked, Seeking, Select,
    SelectStart, SelectionChange, Show, SlotChange, Stalled, Submit, Suspend, TimeUpdate, Toggle,
    TouchCancel, TouchEnd, TouchMove, TouchStart, TransitionCancel, TransitionEnd, TransitionRun,
    TransitionStart, VolumeChange, Waiting,
};
use crate::events::Wheel;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::Callback;

macro_rules! add_from_yew_callbacl_impl {
    ($const_type:ident: $struct_name:ident, $($rest:tt)*) => {
        impl From<Callback<web_sys::$const_type>> for $struct_name {
            fn from(cb: Callback<web_sys::$const_type>) -> Self {
                let func = move |e: &web_sys::Event| {
                    let event = e.clone();
                    cb.emit(event.dyn_into::<web_sys::$const_type>().unwrap());
                };
                Self(Rc::new(func))
            }
        }

        add_from_yew_callbacl_impl!($const_type: $($rest)*);
    };

    ($const_type:ident: $struct_name:ident) => {
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

add_from_yew_callbacl_impl!(
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

add_from_yew_callbacl_impl!(
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

add_from_yew_callbacl_impl!(FocusEvent: Blur, Focus, FocusIn, FocusOut, Submit);

add_from_yew_callbacl_impl!(
    DragEvent: Drag,
    DragEnd,
    DragEnter,
    DragExit,
    DragLeave,
    DragOver,
    DragStart,
    Drop
);

add_from_yew_callbacl_impl!(InputEvent: Input);

add_from_yew_callbacl_impl!(KeyboardEvent: Keydown, KeyPress, KeyUp);

add_from_yew_callbacl_impl!(ProgressEvent: LoadStart, Progress, LoadEnd);

add_from_yew_callbacl_impl!(WheelEvent: Wheel);

add_from_yew_callbacl_impl!(
    AnimationEvent: AnimationCancel,
    AnimationEnd,
    AnimationIteration,
    AnimationStart
);

add_from_yew_callbacl_impl!(
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

add_from_yew_callbacl_impl!(TouchEvent: TouchCancel, TouchEnd, TouchMove, TouchStart);

add_from_yew_callbacl_impl!(
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

impl std::fmt::Debug for CustomEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomEvent")
            .field("event_type", &self.event_type)
            .field("callback", &String::from("a callback function"))
            .finish()
    }
}
