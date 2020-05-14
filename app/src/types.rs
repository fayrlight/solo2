use crate::hal;
use hal::drivers::UsbBus;
use littlefs2::{
    const_ram_storage,
};
use trussed::types::{LfsResult, LfsStorage};
use trussed::store;
use ctap_types::consts;
use fido_authenticator::SilentAuthenticator;
// use usbd_ctaphid::insecure::InsecureRamAuthenticator;

pub type FlashStorage = hal::drivers::FlashGordon;

pub type Authenticator = fido_authenticator::Authenticator<SilentAuthenticator>;

pub type Piv = piv_card::App;

pub use trussed::client::TrussedSyscall;
// #[derive(Default)]
// pub struct CryptoSyscall {}

// impl trussed::pipe::Syscall for CryptoSyscall {
//     fn syscall(&mut self) {
//         rtfm::pend(hal::raw::Interrupt::OS_EVENT);
//     }
// }

// 8KB of RAM
const_ram_storage!(
    name=VolatileStorage,
    trait=LfsStorage,
    erase_value=0xff,
    read_size=1,
    write_size=1,
    cache_size_ty=consts::U104,
    // this is a limitation of littlefs
    // https://git.io/JeHp9
    block_size=104,
    // block_size=128,
    block_count=8192/104,
    lookaheadwords_size_ty=consts::U1,
    filename_max_plus_one_ty=consts::U256,
    path_max_plus_one_ty=consts::U256,
    result=LfsResult,
);

// minimum: 2 blocks
// TODO: make this optional
const_ram_storage!(ExternalStorage, 1024);

store!(Store,
    Internal: FlashStorage,
    External: ExternalStorage,
    Volatile: VolatileStorage
);

pub type CryptoService = trussed::Service<
    hal::peripherals::rng::Rng<hal::Enabled>,
    Store,
>;

#[cfg(feature = "highspeed")]
pub type EnabledUsbPeripheral = hal::peripherals::usbhs::EnabledUsbhsDevice;
#[cfg(not(feature = "highspeed"))]
pub type EnabledUsbPeripheral = hal::peripherals::usbfs::EnabledUsbfsDevice;

pub type CcidClass = usbd_ccid::Ccid<UsbBus<EnabledUsbPeripheral>>;
// pub type CtapHidClass = usbd_ctaphid::CtapHid<'static, InsecureRamAuthenticator, UsbBus>;
pub type CtapHidClass = usbd_ctaphid::CtapHid<'static, UsbBus<EnabledUsbPeripheral>>;

pub type SerialClass = usbd_serial::SerialPort<'static, UsbBus<EnabledUsbPeripheral>>;
// pub type SerialClass = usbd_serial::CdcAcmClass<'static, UsbBus>;
pub type Usbd = usb_device::device::UsbDevice<'static, UsbBus<EnabledUsbPeripheral>>;

