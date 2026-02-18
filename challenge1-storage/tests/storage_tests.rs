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

#[test]
fn converts_borsh_to_json_and_preserves_data() {
    let person = Person {
        name: "Andre".to_string(),
        age: 30,
    };
    let mut borsh_storage = Storage::<Person, Borsh>::new(Borsh);
    borsh_storage
        .save(&person)
        .expect("borsh setup save should succeed");

    let json_storage = borsh_storage
        .convert_to(Json)
        .expect("conversion borsh->json should succeed");
    let loaded = json_storage.load().expect("json load should succeed");

    assert!(json_storage.has_data());
    assert_eq!(loaded, person);
}

#[test]
fn converts_json_to_wincode_and_preserves_data() {
    let person = Person {
        name: "Andre".to_string(),
        age: 30,
    };
    let mut json_storage = Storage::<Person, Json>::new(Json);
    json_storage
        .save(&person)
        .expect("json setup save should succeed");

    let wincode_storage = json_storage
        .convert_to(Wincode)
        .expect("conversion json->wincode should succeed");
    let loaded = wincode_storage.load().expect("wincode load should succeed");

    assert!(wincode_storage.has_data());
    assert_eq!(loaded, person);
}

#[test]
fn converts_wincode_to_borsh_and_preserves_data() {
    let person = Person {
        name: "Andre".to_string(),
        age: 30,
    };
    let mut wincode_storage = Storage::<Person, Wincode>::new(Wincode);
    wincode_storage
        .save(&person)
        .expect("wincode setup save should succeed");

    let borsh_storage = wincode_storage
        .convert_to(Borsh)
        .expect("conversion wincode->borsh should succeed");
    let loaded = borsh_storage.load().expect("borsh load should succeed");

    assert!(borsh_storage.has_data());
    assert_eq!(loaded, person);
}
