use crate::*;
use glfw::*;
use std::ffi::CString;
use std::mem::forget;

extern "C" fn adm_version() -> *const c_char {
	let cstr = CString::new("WanganArcadeLoader 0.1").unwrap();
	let ptr = cstr.as_ptr();
	forget(cstr);
	ptr
}

pub static mut WINDOW_HANDLE: Option<*mut c_void> = None;

#[repr(C)]
struct AdmDevice {
	ident: [u8; 4], // DEVI
	glfw: Glfw,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Default)]
struct AdmChooseMode {
	ident: [u8; 4], // MOCF
	unk_0x04: u32,
	unk_0x08: u32,
	unk_0x0C: u32,
	unk_0x10: u32,
	unk_0x14: u32,
	width: u32,
	height: u32,
	refresh: u32,
}

#[repr(C)]
#[derive(Default)]
struct AdmFBConfig {
	ident: [u8; 4], // FBCF
}

#[repr(C)]
#[derive(Default)]
struct AdmScreen {
	ident: [u8; 4], // SCRN
}

#[repr(C)]
#[derive(Default)]
struct AdmGraphicsContext {
	ident: [u8; 4], // CNTX
}

#[repr(C)]
struct AdmWindow {
	ident: [u8; 4], // WNDW
	window: PWindow,
}

extern "C" fn adm_device() -> *const AdmDevice {
	let glfw = glfw::init(glfw::fail_on_errors).unwrap();
	let adm = AdmDevice {
		ident: [b'D', b'E', b'V', b'I'],
		glfw,
	};
	let adm = Box::new(adm);
	Box::leak(adm)
}

extern "C" fn adm_config() -> *const *const AdmChooseMode {
	let adm = AdmChooseMode {
		ident: [b'M', b'O', b'C', b'F'],
		..Default::default()
	};
	Box::leak(Box::new(Box::leak(Box::new(adm)) as *const AdmChooseMode))
}

extern "C" fn adm_fb_config() -> *const AdmFBConfig {
	let adm = AdmFBConfig {
		ident: [b'F', b'B', b'C', b'F'],
		..Default::default()
	};
	let adm = Box::new(adm);
	Box::leak(adm)
}

extern "C" fn adm_screen() -> *const AdmScreen {
	let adm = AdmScreen {
		ident: [b'S', b'C', b'R', b'N'],
		..Default::default()
	};
	let adm = Box::new(adm);
	Box::leak(adm)
}

unsafe extern "C" fn adm_context() -> *const AdmGraphicsContext {
	let adm = AdmGraphicsContext {
		ident: [b'C', b'N', b'T', b'X'],
		..Default::default()
	};
	let adm = Box::new(adm);
	Box::leak(adm)
}

static mut WIDTH: u32 = 640;
static mut HEIGHT: u32 = 480;

unsafe extern "C" fn adm_window(device: *mut AdmDevice) -> *const AdmWindow {
	let device = device.as_mut().unwrap();
	let monitor = Monitor::from_primary();
	let window_mode = if let Some(config) = &CONFIG {
		if config.fullscreen {
			WindowMode::FullScreen(&monitor)
		} else {
			WindowMode::Windowed
		}
	} else {
		WindowMode::Windowed
	};
	device.glfw.window_hint(WindowHint::Resizable(false)); // Force floating on tiling window managers
	let (mut window, _) = device
		.glfw
		.create_window(WIDTH, HEIGHT, "WanganArcadeLoader", window_mode)
		.unwrap();
	WINDOW_HANDLE = Some(window.get_x11_window());
	window.make_current();
	window.set_resizable(true);
	device.glfw.set_swap_interval(SwapInterval::Sync(1));
	let adm = AdmWindow {
		ident: [b'W', b'N', b'D', b'W'],
		window,
	};
	let adm = Box::new(adm);
	let ptr = Box::leak(adm);

	gl::load_gl_funcs(&device.glfw);

	ptr
}

unsafe extern "C" fn adm_swap_buffers(window: *mut AdmWindow) -> c_int {
	let window = window.as_mut().unwrap();
	window.window.swap_buffers();

	0
}

unsafe extern "C" fn adm_setup() -> c_int {
	std::arch::asm!(
		"push ebp",
		"mov ebp, [esp + 0xDC]",
		"mov {width:e}, [ebp - 0x14]",
		"mov {height:e}, [ebp - 0x18]",
		"pop ebp",
		width = out(reg) WIDTH,
		height = out(reg) HEIGHT,
	);

	1
}
pub unsafe fn init() {
	hook::hook_symbol("admvt_setup", adm_setup as *const ());
	hook::hook_symbol("admShutdown", adachi as *const ());
	hook::hook_symbol("admGetString", adm_version as *const ());
	hook::hook_symbol("admGetNumDevices", adachi as *const ());
	hook::hook_symbol("admInitDevicei", adm_device as *const ());
	hook::hook_symbol("admChooseModeConfigi", adm_config as *const ());
	hook::hook_symbol("admModeConfigi", adachi as *const ());
	hook::hook_symbol("admChooseFBConfigi", adm_fb_config as *const ());
	hook::hook_symbol("admCreateScreeni", adm_screen as *const ());
	hook::hook_symbol("admCreateGraphicsContext", adm_context as *const ());
	hook::hook_symbol("admCreateWindowi", adm_window as *const ());
	hook::hook_symbol("admDisplayScreen", adachi as *const ());
	hook::hook_symbol("admMakeContextCurrent", adachi as *const ());
	hook::hook_symbol("admSwapInterval", adachi as *const ());
	hook::hook_symbol("admCursorAttribi", adachi as *const ());
	hook::hook_symbol("admGetDeviceAttribi", adachi as *const ());
	hook::hook_symbol("admSwapBuffers", adm_swap_buffers as *const ());
	hook::hook_symbol("admSetMonitorGamma", adachi as *const ());
	hook::hook_symbol("_ZN15clSpriteManagerD1Ev", adachi as *const ());
}
