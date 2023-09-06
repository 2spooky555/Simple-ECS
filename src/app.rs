use crate::world::World;
use crate::systems::{System, StartUpSystem};
use crate::scheduler::Scheduler;

#[derive(Default)]
pub struct App {
    world: World,
    scheduler: Scheduler,
    run: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            run: true,
            ..Default::default()
        }
    }

    pub fn add_startup_system(&mut self, system: impl StartUpSystem + 'static) {
        self.scheduler.add_startup_system(system);
    }

    pub fn add_system(&mut self, system: impl System + 'static) {
        self.scheduler.add_system(system);
    }

    pub fn run(&mut self) {
        self.run = self.scheduler.run_startup_systems(&mut self.world);
        while self.run {
            self.run = self.scheduler.run_systems(&mut self.world);
        }
    }
}