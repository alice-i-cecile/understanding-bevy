use bevy::prelude::*;

// By defining a trait, we can ensure that all of our types
// have the required common functionality
trait MyTimer {
    fn access(&mut self) -> &mut Timer;
}

struct SlowTimer(Timer);
struct FastTimer(Timer);

impl Default for FastTimer {
    fn default() -> Self {
        FastTimer(Timer::from_seconds(0.3, true))
    }
}

impl Default for SlowTimer {
    fn default() -> Self {
        SlowTimer(Timer::from_seconds(2.0, true))
    }
}

impl MyTimer for FastTimer {
    fn access(&mut self) -> &mut Timer {
        &mut self.0
    }
}

impl MyTimer for SlowTimer {
    fn access(&mut self) -> &mut Timer {
        &mut self.0
    }
}

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        // We can customize two otherwise identical systems by specifying its type
        .add_system(run_timer::<SlowTimer>.system())
        // The timer resource in these two systems are distinct
        .add_system(run_timer::<FastTimer>.system())
        .run();
}

// System-local resources initialize their value with their
// The trait bounds are needed to ensure that any T we might provide will work
fn run_timer<T: Send + Sync + Default + 'static + MyTimer>(time: Res<Time>, mut timer: Local<T>) {
    if timer.access().tick(time.delta_seconds()).just_finished() {
        println!("The time is {:?}", *time);
    }
}
