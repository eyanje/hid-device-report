use super::usage::{Usage, UsagePage};

// Usage IDs.
// From the HID Usages and Descriptions document.

pub mod page {
    use super::UsagePage;

    pub const GENERIC_DESKTOP: UsagePage = 0x0001;
    pub const SIMULATION_CONTROLS: UsagePage = 0x0002;
    pub const VR_CONTROLS: UsagePage = 0x0003;
    pub const SPORTS_CONTROL: UsagePage = 0x0004;
    pub const GAME_CONTROLS: UsagePage = 0x0005;
    pub const GENERIC_DEVICE_CONTROLS: UsagePage = 0x0006;
    pub const KEYBOARD_KEYPAD: UsagePage = 0x0007;
    pub const LED: UsagePage = 0x0008;
    pub const BUTTON: UsagePage = 0x0009;
    pub const ORDINAL: UsagePage = 0x000A;
    pub const TELEPHONY_DEVICE: UsagePage = 0x000B;
    pub const CONSUMER: UsagePage = 0x000C;
    pub const DIGITIZERS: UsagePage = 0x000D;
    pub const HAPTICS: UsagePage = 0x000E;
    pub const PHYSICAL_INPUT_DEVICE: UsagePage = 0x000F;
    pub const UNICODE: UsagePage = 0x0010;
    pub const SOC: UsagePage = 0x0011;
    pub const EYE_AND_HEAD_TRACKERS: UsagePage = 0x0012;

    pub const AUXILIARY_DISPLAY: UsagePage = 0x0014;

    pub const SENSORS: UsagePage = 0x0020;

    pub const MEDICAL_INSTRUMENT: UsagePage = 0x0040;
    pub const BRAILLE_DISPLAY: UsagePage = 0x0041;

    pub const LIGHTING_AND_ILLUMINATION: UsagePage = 0x0059;

    pub const MONITOR: UsagePage = 0x0080;
    pub const MONITOR_ENUMERATED: UsagePage = 0x0081;
    pub const VESA_VIRTUAL_CONTROLS: UsagePage = 0x0082;

    pub const POWER: UsagePage = 0x0084;
    pub const BATTERY_SYSTEM: UsagePage = 0x0085;

    pub const BARCODE_SCANNER: UsagePage = 0x008C;
    pub const SCALES: UsagePage = 0x008D;
    pub const MAGNETIC_STRIPE_READER: UsagePage = 0x008E;

    pub const CAMERA_CONTROL: UsagePage = 0x0090;
    pub const ARCADE: UsagePage = 0x0091;
    pub const GAMING_DEVICE: UsagePage = 0x0092;

    pub const FIDO_ALLIANCE: UsagePage = 0xF1D0;
}

pub mod generic_desktop {
    use super::{page, Usage, UsagePage};

    pub const PAGE: UsagePage = page::GENERIC_DESKTOP;

    pub const POINTER: Usage = Usage::new(PAGE, 0x0001);
    pub const MOUSE: Usage = Usage::new(PAGE, 0x0002);

    pub const JOYSTICK: Usage = Usage::new(PAGE, 0x0004);
    pub const GAMEPAD: Usage = Usage::new(PAGE, 0x0005);
    pub const KEYBOARD: Usage = Usage::new(PAGE, 0x0006);
    pub const KEYPAD: Usage = Usage::new(PAGE, 0x0007);
    pub const MULTI_AXIS_CONTROLLER: Usage = Usage::new(PAGE, 0x0008);
    pub const TABLET_PC_SYSTEM_CONTROLS: Usage = Usage::new(PAGE, 0x0009);
    pub const WATER_COOLING_DEVICE: Usage = Usage::new(PAGE, 0x000A);
    pub const COMPUTER_CHASSIS_DEVICE: Usage = Usage::new(PAGE, 0x000B);
    pub const WIRELESS_RADIO_CONTROLS: Usage = Usage::new(PAGE, 0x000C);
    pub const PORTABLE_DEVICE_CONTROL: Usage = Usage::new(PAGE, 0x000D);
    pub const SYSTEM_MULTI_AXIS_CONTROLLER: Usage = Usage::new(PAGE, 0x000E);
    pub const SPATIAL_CONTROLLER: Usage = Usage::new(PAGE, 0x000F);
    pub const ASSISTIVE_CONTROL: Usage = Usage::new(PAGE, 0x0010);
    pub const DEVICE_DOCK: Usage = Usage::new(PAGE, 0x0011);
    pub const DOCKABLE_DEVICE: Usage = Usage::new(PAGE, 0x0012);
    pub const CALL_STATE_MANAGEMENT_CONTROL: Usage = Usage::new(PAGE, 0x0013);

    pub const X: Usage = Usage::new(PAGE, 0x0030);
    pub const Y: Usage = Usage::new(PAGE, 0x0031);
    pub const Z: Usage = Usage::new(PAGE, 0x0032);
    pub const RX: Usage = Usage::new(PAGE, 0x0033);
    pub const RY: Usage = Usage::new(PAGE, 0x0034);
    pub const RZ: Usage = Usage::new(PAGE, 0x0035);
    pub const SLIDER: Usage = Usage::new(PAGE, 0x0036);
    pub const DIAL: Usage = Usage::new(PAGE, 0x0037);
    pub const WHEEL: Usage = Usage::new(PAGE, 0x0038);
    pub const HAT_SWITCH: Usage = Usage::new(PAGE, 0x0039);
    pub const COUNTED_BUFFER: Usage = Usage::new(PAGE, 0x003A);
    pub const BYTE_COUNT: Usage = Usage::new(PAGE, 0x003B);
    pub const MOTION_WAKEUP: Usage = Usage::new(PAGE, 0x003C);
    pub const START: Usage = Usage::new(PAGE, 0x003D);
    pub const SELECT: Usage = Usage::new(PAGE, 0x003E);
    
    pub const VX: Usage = Usage::new(PAGE, 0x0040);
    pub const VY: Usage = Usage::new(PAGE, 0x0041);
    pub const VZ: Usage = Usage::new(PAGE, 0x0042);
    pub const VBRX: Usage = Usage::new(PAGE, 0x0043);
    pub const VBRY: Usage = Usage::new(PAGE, 0x0044);
    pub const VBRZ: Usage = Usage::new(PAGE, 0x0045);
    pub const VNO: Usage = Usage::new(PAGE, 0x0046);
    pub const FEATURE_NOTIFICATION: Usage = Usage::new(PAGE, 0x0047);
    pub const RESOLUTION_MULTIPLIER: Usage = Usage::new(PAGE, 0x0048);
    pub const QX: Usage = Usage::new(PAGE, 0x0049);
    pub const QY: Usage = Usage::new(PAGE, 0x004A);
    pub const QZ: Usage = Usage::new(PAGE, 0x004B);
    pub const QW: Usage = Usage::new(PAGE, 0x004C);

    pub const SYSTEM_CONTROL: Usage = Usage::new(PAGE, 0x0080);
    pub const SYSTEM_POWER_DOWN: Usage = Usage::new(PAGE, 0x0081);
    pub const SYSTEM_SLEEP: Usage = Usage::new(PAGE, 0x0082);
    pub const SYSTEM_WAKE_UP: Usage = Usage::new(PAGE, 0x0083);
    pub const SYSTEM_CONTEXT_MENU: Usage = Usage::new(PAGE, 0x0084);
    pub const SYSTEM_MAIN_MENU: Usage = Usage::new(PAGE, 0x0085);
    pub const SYSTEM_APP_MENU: Usage = Usage::new(PAGE, 0x0086);
    pub const SYSTEM_MENU_HELP: Usage = Usage::new(PAGE, 0x0087);
    pub const SYSTEM_MENU_EXIT: Usage = Usage::new(PAGE, 0x0088);
    pub const SYSTEM_MENU_SELECT: Usage = Usage::new(PAGE, 0x0089);
    pub const SYSTEM_MENU_RIGHT: Usage = Usage::new(PAGE, 0x008A);
    pub const SYSTEM_MENU_LEFT: Usage = Usage::new(PAGE, 0x008B);
    pub const SYSTEM_MENU_UP: Usage = Usage::new(PAGE, 0x008C);
    pub const SYSTEM_MENU_DOWN: Usage = Usage::new(PAGE, 0x008D);
    pub const SYSTEM_COLD_RESTART: Usage = Usage::new(PAGE, 0x008E);
    pub const SYSTEM_WARM_RESTART: Usage = Usage::new(PAGE, 0x008F);
    pub const D_PAD_UP: Usage = Usage::new(PAGE, 0x0090);
    pub const D_PAD_DOWN: Usage = Usage::new(PAGE, 0x0091);
    pub const D_PAD_RIGHT: Usage = Usage::new(PAGE, 0x0092);
    pub const D_PAD_LEFT: Usage = Usage::new(PAGE, 0x0093);
    pub const INDEX_TRIGGER: Usage = Usage::new(PAGE, 0x0094);
    pub const PALM_TRIGGER: Usage = Usage::new(PAGE, 0x0095);
    pub const THUMBSTICK: Usage = Usage::new(PAGE, 0x0096);
    pub const SYSTEM_FUNCTION_SHIFT: Usage = Usage::new(PAGE, 0x0097);
    pub const SYSTEM_FUNCTION_SHIFT_LOCK: Usage = Usage::new(PAGE, 0x0098);
    pub const SYSTEM_FUNCTION_SHIFT_LOCK_INDICATOR: Usage = Usage::new(PAGE, 0x0099);
    pub const SYSTEM_DISMISS_NOTIFICATION: Usage = Usage::new(PAGE, 0x009A);
    pub const SYSTEM_DO_NOT_DISTURB: Usage = Usage::new(PAGE, 0x009B);

    pub const SYSTEM_DOCK: Usage = Usage::new(PAGE, 0x00A0);
    pub const SYSTEM_UNDOCK: Usage = Usage::new(PAGE, 0x00A1);
    pub const SYSTEM_SETUP: Usage = Usage::new(PAGE, 0x00A2);
    pub const SYSTEM_BREAK: Usage = Usage::new(PAGE, 0x00A3);
    pub const SYSTEM_DEBUGGER_BREAK: Usage = Usage::new(PAGE, 0x00A4);
    pub const APPLICATION_BREAK: Usage = Usage::new(PAGE, 0x00A5);
    pub const APPLICATION_DEBUGGER_BREAK: Usage = Usage::new(PAGE, 0x00A6);
    pub const SYSTEM_SPEAKER_MUTE: Usage = Usage::new(PAGE, 0x00A7);
    pub const SYSTEM_HIBERNATE: Usage = Usage::new(PAGE, 0x00A8);
    pub const SYSTEM_MICROPHONE_MUTE: Usage = Usage::new(PAGE, 0x00A9);

    pub const SYSTEM_DISPLAY_INVERT: Usage = Usage::new(PAGE, 0x00B0);
    pub const SYSTEM_DISPLAY_INTERNAL: Usage = Usage::new(PAGE, 0x00B1);
    pub const SYSTEM_DISPLAY_EXTERNAL: Usage = Usage::new(PAGE, 0x00B2);
    pub const SYSTEM_DISPLAY_BOTH: Usage = Usage::new(PAGE, 0x00B3);
    pub const SYSTEM_DISPLAY_DUAL: Usage = Usage::new(PAGE, 0x00B4);
    pub const SYSTEM_DISPLAY_TOGGLE_INT_EXT_MODE: Usage = Usage::new(PAGE, 0x00B5);
    pub const SYSTEM_DISPLAY_SWAP_PRIMARY_SECONDARY: Usage = Usage::new(PAGE, 0x00B6);
    pub const SYSTEM_DISPLAY_TOGGLE_LCD_AUTOSCALE: Usage = Usage::new(PAGE, 0x00B7);

    pub const SENSOR_ZONE: Usage = Usage::new(PAGE, 0x00C0);
    pub const RPM: Usage = Usage::new(PAGE, 0x00C1);
    pub const COOLANT_LEVEL: Usage = Usage::new(PAGE, 0x00C2);
    pub const COOLANT_CRITICAL_LEVEL: Usage = Usage::new(PAGE, 0x00C3);
    pub const COOLANT_PUMP: Usage = Usage::new(PAGE, 0x00C4);
    pub const CHASSIS_ENCLOSURE: Usage = Usage::new(PAGE, 0x00C5);
    pub const WIRELESS_RADIO_BUTTON: Usage = Usage::new(PAGE, 0x00C6);
    pub const WIRELESS_RATIO_LED: Usage = Usage::new(PAGE, 0x00C7);
    pub const WIRELESS_RADIO_SLIDER_SWITCH: Usage = Usage::new(PAGE, 0x00C8);
    pub const SYSTEM_DISPLAY_ROTATION_LOCK_BUTTON: Usage = Usage::new(PAGE, 0x00C9);
    pub const SYSTEM_DISPLAY_ROTATION_LOCK_SLIDER_SWITCH: Usage = Usage::new(PAGE, 0x00CA);
    pub const CONTROL_ENABLE: Usage = Usage::new(PAGE, 0x00CB);

    pub const DOCKABLE_DEVICE_UNIQUE_ID: Usage = Usage::new(PAGE, 0x00D0);
    pub const DOCKABLE_DEVICE_VENDOR_ID: Usage = Usage::new(PAGE, 0x00D1);
    pub const DOCKABLE_DEVICE_PRIMARY_USAGE_PAGE: Usage = Usage::new(PAGE, 0x00D2);
    pub const DOCKABLE_DEVICE_PRIMARY_USAGE_ID: Usage = Usage::new(PAGE, 0x00D3);
    pub const DOCKABLE_DEVICE_DOCKING_STATE: Usage = Usage::new(PAGE, 0x00D4);
    pub const DOCKABLE_DEVICE_DISPLAY_OCCLUSION: Usage = Usage::new(PAGE, 0x00D5);
    pub const DOCKABE_DEVICE_OBJECT_TYPE: Usage = Usage::new(PAGE, 0x00D6);

    pub const CALL_ACTIVE_LED: Usage = Usage::new(PAGE, 0x00E0);
    pub const CALL_MUTE_TOGGLE: Usage = Usage::new(PAGE, 0x00E1);
    pub const CALL_MUTE_LED: Usage = Usage::new(PAGE, 0x00E2);
}

pub mod keyboard_keypad {
    use super::{page, Usage};

    pub fn new(key: u16) -> Usage {
        Usage::new(page::KEYBOARD_KEYPAD, key)
    }
}

pub mod led {
    use super::{page, Usage};

    pub fn new(led: u16) -> Usage {
        Usage::new(page::LED, led)
    }
}

pub mod button {
    use super::{page, Usage};

    pub fn new(button: u16) -> Usage {
        Usage::new(page::BUTTON, button)
    }
}