use bevy_ecs::prelude::*;
use tracing::trace;

#[allow(dead_code)]
fn traced_system(mut ents: Query<'_, (Entity, &i32)>) {
    trace!("Running system");

    for (ent, i) in &mut ents.iter().iter() {
        trace!("Running system on entity: {:?} {}", ent, i);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::{ParallelExecutor, Schedule};
    use bevy_tasks::{ComputeTaskPool, TaskPool};
    use tracing::*;
    use tracing_subscriber::{fmt, EnvFilter};

    #[test]
    fn test_bevy_system_with_tracing() {
        let subscriber = fmt()
            .with_env_filter(EnvFilter::new("trace"))
            .with_max_level(Level::TRACE)
            .with_test_writer()
            .finish();

        // let _guard = subscriber::set_default(subscriber);

        trace!("Example trace from test body");

        let mut world = World::default();
        let mut res = Resources::default();
        let mut schedule = Schedule::default();

        world.spawn((1,));

        schedule.add_stage("update");
        schedule.add_system_to_stage("update", traced_system.system());
        schedule.initialize(&mut world, &mut res);

        tracing::subscriber::with_default(subscriber, || {
            trace!("Example closure trace");

            // Emits trace messages as intended
            schedule.run(&mut world, &mut res);

            let mut executor = ParallelExecutor::default();
            // Only present in Bevy from Git
            // executor.initialize(&mut res);
            if res.get::<ComputeTaskPool>().is_none() {
                res.insert(ComputeTaskPool(TaskPool::default()));
            }

            trace!("Running system via executor");
            // Will emit no traces, presumably due to contextual shift
            executor.run(&mut schedule, &mut world, &mut res);
        });
    }
}
