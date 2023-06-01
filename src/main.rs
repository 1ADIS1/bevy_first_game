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
        app.add_startup_system(setup).add_systems((
            print_names,
            print_people_with_jobs,
            print_people_ready_for_hire,
            print_person_job,
        ));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::DOCTOR },
    ));
    commands.spawn(Person {
        name: "John".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Person name is {0}", person.name);
    }
}

pub fn print_people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{0} has a job.", person.name);
    }
}

pub fn print_people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{0} is ready for hire.", person.name);
    }
}

pub fn print_person_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        println!("{} is {:?}", person.name, employed.job);
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
    DOCTOR,
}
