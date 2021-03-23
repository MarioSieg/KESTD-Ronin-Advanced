use super::prelude::*;
use crate::impls::platform::prelude::*;

pub struct PlatformSystem {
    pub win_data: WindowData,
    pub sys_info: SystemInfo,
}

impl SubSystem for PlatformSystem {
    type Args = ();

    fn initialize(cfg: &mut CoreConfig, _: &Self::Args) -> Self {
        let sys_info = get_and_print_system_info();
        let win_data = WindowData::create_window(cfg);

        Self { win_data, sys_info }
    }

    fn prepare(&mut self) {
        self.win_data.window.focus();
        self.win_data.window.show();
    }

    fn tick(&mut self, _world: &mut World) -> bool {
        self.win_data.context.poll_events();
        for (_, _) in flush_messages(&self.win_data.events) {}
        !self.win_data.window.should_close()
    }
}
