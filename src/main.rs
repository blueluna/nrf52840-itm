#![no_main]
#![no_std]

use cortex_m::{iprintln, peripheral::ITM};

#[allow(unused_imports)]
use panic_itm;

use rtfm::app;

use nrf52840_pac;

#[app(device = nrf52840_pac)]
const APP: () = {
    static mut ITM: ITM = ();

    #[init]
    fn init() {
        iprintln!(&mut core.ITM.stim[0], "Initialize");

        ITM = core.ITM;
    }

    #[idle(resources = [ITM])]
    fn idle() -> ! {
        let itm = resources.ITM;
        iprintln!(&mut itm.stim[0], "...");
        panic!("end");
    }
};
