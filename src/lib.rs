extern crate winit;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::collections::HashMap;
use std::mem::transmute;

#[repr(u32)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub enum MouseButton {
	Left,
	Right,
	Middle,
	Other(u8),
}

impl From<winit::MouseButton> for MouseButton {
	fn from(x: winit::MouseButton) -> MouseButton {
		use winit::MouseButton::*;
		match x {
			Left => MouseButton::Left,
			Right => MouseButton::Right,
			Middle => MouseButton::Middle,
			Other(y) => MouseButton::Other(y),
		}
	}
}

#[repr(u32)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub enum Key {
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	Key0,
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	Escape,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	F13,
	F14,
	F15,
	Snapshot,
	Scroll,
	Pause,
	Insert,
	Home,
	Delete,
	End,
	PageDown,
	PageUp,
	Left,
	Up,
	Right,
	Down,
	Back,
	Return,
	Space,
	Compose,
	Caret,
	Numlock,
	Numpad0,
	Numpad1,
	Numpad2,
	Numpad3,
	Numpad4,
	Numpad5,
	Numpad6,
	Numpad7,
	Numpad8,
	Numpad9,
	AbntC1,
	AbntC2,
	Add,
	Apostrophe,
	Apps,
	At,
	Ax,
	Backslash,
	Calculator,
	Capital,
	Colon,
	Comma,
	Convert,
	Decimal,
	Divide,
	Equals,
	Grave,
	Kana,
	Kanji,
	LAlt,
	LBracket,
	LControl,
	LMenu,
	LShift,
	LWin,
	Mail,
	MediaSelect,
	MediaStop,
	Minus,
	Multiply,
	Mute,
	MyComputer,
	NavigateForward,
	NavigateBackward,
	NextTrack,
	NoConvert,
	NumpadComma,
	NumpadEnter,
	NumpadEquals,
	OEM102,
	Period,
	PlayPause,
	Power,
	PrevTrack,
	RAlt,
	RBracket,
	RControl,
	RMenu,
	RShift,
	RWin,
	Semicolon,
	Slash,
	Sleep,
	Stop,
	Subtract,
	Sysrq,
	Tab,
	Underline,
	Unlabeled,
	VolumeDown,
	VolumeUp,
	Wake,
	WebBack,
	WebFavorites,
	WebForward,
	WebHome,
	WebRefresh,
	WebSearch,
	WebStop,
	Yen,
	Copy,
	Paste,
	Cut,
}

impl From<winit::VirtualKeyCode> for Key {
	fn from(x: winit::VirtualKeyCode) -> Key {
		unsafe { transmute(x) }
	}
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub enum EventType {
	KeyPress(Key, Modifiers),
	KeyRelease(Key, Modifiers),
	ScanCodePress(u32, Modifiers),
	ScanCodeRelease(u32, Modifiers),
	MousePress(MouseButton, Modifiers),
	MouseRelease(MouseButton, Modifiers),
	WindowResized,
	WindowMoved,
	Closed,
	DroppedFile,
	HoveredFile,
	HoveredFileCancelled,
	Focused,
	UnFocused,
	MouseMoved,
	MouseEntered,
	MouseLeft,
	MouseWheel,
	Refresh,
}

impl EventType {
	pub fn modifiers(&mut self) -> Option<&mut Modifiers> {
		use EventType::*;
		match self {
			KeyPress(_,ref mut m) | KeyRelease(_,ref mut m) |
			ScanCodePress(_,ref mut m) | ScanCodeRelease(_,ref mut m) |
			MousePress(_,ref mut m) | MouseRelease(_,ref mut m) =>
				Some(m),
			_ => None,
		}
	}
	
	pub fn modifier_combos(mut self) -> [Self; 16] {
		let mut result = [self; 16];
		if let Some(_) = self.modifiers() {
			for i in 0..16 {
				let m = Modifiers {
					shift: i % 2 > 0,
					ctrl: i % 4 > 1,
					alt: i % 8 > 3,
					logo: i % 16 > 7,
				};
				*result[i].modifiers().unwrap() = m;
			}
		}
		result
	}
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Default,Serialize,Deserialize)]
pub struct Modifiers {
	pub shift: bool,
	pub ctrl: bool,
	pub alt: bool,
	pub logo: bool,
}

impl From<winit::ModifiersState> for Modifiers {
	fn from(x: winit::ModifiersState) -> Self {
		Self {
			shift: x.shift,
			ctrl: x.ctrl,
			alt: x.alt,
			logo: x.logo,
		}
	}
}

use winit::{EventsLoop,ElementState};
use std::path::PathBuf;

pub struct Events<E> {
	events_loop: EventsLoop,
	bindings: HashMap<EventType, fn((f64, f64), Option<PathBuf>) -> E>,
	buffer: Vec<E>,
	chars: Option<Vec<char>>,
}

impl<E> Events<E> {
	pub fn new(events_loop: EventsLoop) -> Self {
		Self {
			events_loop: events_loop,
			bindings: HashMap::new(),
			buffer: Vec::new(),
			chars: None,
		}
	}
	
	pub fn inner(&mut self) -> &mut EventsLoop {
		&mut self.events_loop
	}
	
	pub fn into_inner(self) -> EventsLoop {
		self.events_loop
	}
	
	pub fn start_recording_chars(&mut self) {
		if self.chars.is_none() {
			self.chars = Some(Vec::new());
		}
	}
	
	pub fn stop_recording_chars(&mut self) {
		self.chars = None;
	}
	
	pub fn char_buf(&mut self) -> Option<Vec<char>> {
		::std::mem::replace(&mut self.chars, Some(Vec::new()))
	}
	
	pub fn add_binding(&mut self, typ: EventType, constructor: fn((f64, f64), Option<PathBuf>) -> E) {
		self.bindings.insert(typ, constructor);
	}
	
	pub fn add_binding_modifier_ignorant(&mut self, typ: EventType, constructor: fn((f64, f64), Option<PathBuf>) -> E) {
		for typ in typ.modifier_combos().iter() {
			self.add_binding(*typ, constructor);
		}
	}
	
	pub fn remove_binding(&mut self, typ: EventType) {
		let _ = self.bindings.remove(&typ);
	}
	
	pub fn remove_binding_modifier_ignorant(&mut self, typ: EventType) {
		for typ in typ.modifier_combos().iter() {
			self.remove_binding(*typ);
		}
	}
	
	pub fn next(&mut self) -> Option<E> {
		if self.buffer.len() > 0 {
			self.buffer.pop()
		} else {
			self.fill_buffer();
			self.buffer.pop()
		}
	}
	
	pub fn fill_buffer(&mut self) {
		let events = &mut self.events_loop;
		let bindings = &self.bindings;
		let buffer = &mut self.buffer;
		let chars = &mut self.chars;

		events.poll_events(|event| {
			if let winit::Event::WindowEvent { window_id: _, event } = event {
				use winit::WindowEvent::*;
				match event {
					KeyboardInput{ input: winit::KeyboardInput {
							scancode,
							state: ElementState::Pressed,
							virtual_keycode,
							modifiers,
						}, .. } => {
						if let Some(key) = virtual_keycode {
							bindings.get(&EventType::KeyPress(key.into(), modifiers.into())).map(|f| buffer.push(f((0.0, 0.0), None)));
						}
						bindings.get(&EventType::ScanCodePress(scancode, modifiers.into())).map(|f| buffer.push(f((0.0, 0.0), None)))
					},
					KeyboardInput{ input: winit::KeyboardInput {
							scancode,
							state: ElementState::Released,
							virtual_keycode,
							modifiers,
					}, .. } => {
						if let Some(key) = virtual_keycode {
							bindings.get(&EventType::KeyRelease(key.into(), modifiers.into())).map(|f| buffer.push(f((0.0, 0.0), None)));
						}
						bindings.get(&EventType::ScanCodeRelease(scancode, modifiers.into())).map(|f| buffer.push(f((0.0, 0.0), None)))
					},
					MouseInput{ state: ElementState::Pressed, button, modifiers, .. } =>
						bindings.get(&EventType::MousePress(button.into(), modifiers.into())).map(|f| buffer.push(f((0.0, 0.0), None))),
					MouseInput{ state: ElementState::Released, button, modifiers, .. } =>
						bindings.get(&EventType::MouseRelease(button.into(), modifiers.into())).map(|f| buffer.push(f((0.0, 0.0), None))),
					
					Resized(x,y) => bindings.get(&EventType::WindowResized).map(|f| buffer.push(f((x as f64, y as f64), None))),
					Moved(x,y) => bindings.get(&EventType::WindowResized).map(|f| buffer.push(f((x as f64,y as f64), None))),
					CloseRequested | Destroyed => bindings.get(&EventType::Closed).map(|f| buffer.push(f((0.0, 0.0), None))),
					DroppedFile(path) => bindings.get(&EventType::DroppedFile).map(|f| buffer.push(f((0.0, 0.0), Some(path)))),
					HoveredFile(path) => bindings.get(&EventType::HoveredFile).map(|f| buffer.push(f((0.0, 0.0), Some(path)))),
					HoveredFileCancelled => bindings.get(&EventType::HoveredFileCancelled).map(|f| buffer.push(f((0.0, 0.0), None))),
					ReceivedCharacter(c) => chars.as_mut().map(|b| b.push(c)),
					Focused(true) => bindings.get(&EventType::Focused).map(|f| buffer.push(f((0.0, 0.0), None))),
					Focused(false) => bindings.get(&EventType::UnFocused).map(|f| buffer.push(f((0.0, 0.0), None))),
					CursorMoved{ position, .. } => bindings.get(&EventType::MouseMoved).map(|f| buffer.push(f(position, None))),
					CursorEntered{ .. } => bindings.get(&EventType::MouseEntered).map(|f| buffer.push(f((0.0, 0.0), None))),
					CursorLeft{ .. } => bindings.get(&EventType::MouseLeft).map(|f| buffer.push(f((0.0, 0.0), None))),
					MouseWheel{ delta, .. } => bindings.get(&EventType::MouseWheel).map(|f| buffer.push(f(convert_scroll_delta(delta), None))),
					Refresh => bindings.get(&EventType::Refresh).map(|f| buffer.push(f((0.0, 0.0), None))),
					_ => {None},
				};
			}
		});
	}
}

use winit::MouseScrollDelta;

fn convert_scroll_delta(d: MouseScrollDelta) -> (f64, f64) {
	match d {
		MouseScrollDelta::LineDelta(x,y) => (x as f64, y as f64),
		MouseScrollDelta::PixelDelta(x,y) => (x as f64, y as f64),
	}
}
