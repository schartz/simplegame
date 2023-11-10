use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        // Add things here
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(jobless_people)
            .add_system(get_jobs);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (
            Person { name: "Tini".to_string() },
            Employed { job: Job::Pilot }
        )
    );

    commands.spawn(
        Person { name: "Tiny".to_string() }
    );

    commands.spawn((
        Person { name: "Tinny".to_string() },
        Employed { job: Job::Doctor }
    )
    );

    commands.spawn((
        Person { name: "Tino".to_string() },
        Employed { job: Job::Engineer }
    )
    );
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name)
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name)
    }
}

pub fn jobless_people(q: Query<&Person, Without<Employed>>) {
    for p in q.iter() {
        println!("{} is available to work.", p.name)
    }
}

pub fn get_jobs(q: Query<(&Person, &Employed)>) {
    for (p, j) in q.iter() {
        let job_name: &str = match j.job {
            Job::Doctor => "Doctor",
            Job::Pilot => "Pilot",
            Job::Engineer => "Engineer"
        };

        println!("{} is a {}", p.name, job_name)
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    Pilot,
    Engineer,
}


