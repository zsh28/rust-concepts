use challenge1_storage::{Borsh, Json, Person, Storage, Wincode};

#[test]
fn saves_and_loads_with_borsh() {
    let person = Person {
        name: "Andre".to_string(),
        age: 30,
    };
    let mut storage = Storage::<Person, Borsh>::new(Borsh);

    storage.save(&person).expect("borsh save should succeed");
    let loaded = storage.load().expect("borsh load should succeed");

    assert!(storage.has_data());
    assert_eq!(loaded, person);
}

#[test]
fn saves_and_loads_with_wincode() {
    let person = Person {
        name: "Andre".to_string(),
        age: 30,
    };
    let mut storage = Storage::<Person, Wincode>::new(Wincode);

    storage.save(&person).expect("wincode save should succeed");
    let loaded = storage.load().expect("wincode load should succeed");

    assert!(storage.has_data());
    assert_eq!(loaded, person);
}

#[test]
fn saves_and_loads_with_json() {
    let person = Person {
        name: "Andre".to_string(),
        age: 30,
    };
    let mut storage = Storage::<Person, Json>::new(Json);

    storage.save(&person).expect("json save should succeed");
    let loaded = storage.load().expect("json load should succeed");

    assert!(storage.has_data());
    assert_eq!(loaded, person);
}
