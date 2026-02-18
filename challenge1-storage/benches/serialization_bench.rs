use challenge1_storage::{Borsh, Json, Person, Serializer, Storage, Wincode};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn sample_person() -> Person {
    sample_person_with_name_len("Andre Benchmark Payload".len() * 16)
}

fn sample_person_with_name_len(name_bytes: usize) -> Person {
    Person {
        name: "a".repeat(name_bytes),
        age: 30,
    }
}

fn bench_serialize(c: &mut Criterion) {
    let person = sample_person();
    let borsh = Borsh;
    let wincode = Wincode;
    let json = Json;

    let mut group = c.benchmark_group("serialize_person");
    group.bench_function("borsh", |b| {
        b.iter(|| black_box(borsh.to_bytes(black_box(&person)).expect("borsh serialize")))
    });
    group.bench_function("wincode", |b| {
        b.iter(|| {
            black_box(
                wincode
                    .to_bytes(black_box(&person))
                    .expect("wincode serialize"),
            )
        })
    });
    group.bench_function("json", |b| {
        b.iter(|| black_box(json.to_bytes(black_box(&person)).expect("json serialize")))
    });
    group.finish();
}

fn bench_deserialize(c: &mut Criterion) {
    let person = sample_person();
    let borsh = Borsh;
    let wincode = Wincode;
    let json = Json;

    let borsh_bytes = borsh.to_bytes(&person).expect("borsh bytes");
    let wincode_bytes = wincode.to_bytes(&person).expect("wincode bytes");
    let json_bytes = json.to_bytes(&person).expect("json bytes");

    let mut group = c.benchmark_group("deserialize_person");
    group.bench_function("borsh", |b| {
        b.iter(|| {
            black_box(
                borsh
                    .from_bytes::<Person>(black_box(&borsh_bytes))
                    .expect("borsh deserialize"),
            )
        })
    });
    group.bench_function("wincode", |b| {
        b.iter(|| {
            black_box(
                wincode
                    .from_bytes::<Person>(black_box(&wincode_bytes))
                    .expect("wincode deserialize"),
            )
        })
    });
    group.bench_function("json", |b| {
        b.iter(|| {
            black_box(
                json.from_bytes::<Person>(black_box(&json_bytes))
                    .expect("json deserialize"),
            )
        })
    });
    group.finish();
}

fn bench_storage_save_load(c: &mut Criterion) {
    let person = sample_person();

    let mut save_group = c.benchmark_group("storage_save_person");
    let mut borsh_storage = Storage::<Person, Borsh>::new(Borsh);
    save_group.bench_function("borsh", |b| {
        b.iter(|| borsh_storage.save(black_box(&person)).expect("borsh save"))
    });
    let mut wincode_storage = Storage::<Person, Wincode>::new(Wincode);
    save_group.bench_function("wincode", |b| {
        b.iter(|| {
            wincode_storage
                .save(black_box(&person))
                .expect("wincode save")
        })
    });
    let mut json_storage = Storage::<Person, Json>::new(Json);
    save_group.bench_function("json", |b| {
        b.iter(|| json_storage.save(black_box(&person)).expect("json save"))
    });
    save_group.finish();

    let mut load_group = c.benchmark_group("storage_load_person");
    let mut borsh_storage = Storage::<Person, Borsh>::new(Borsh);
    borsh_storage.save(&person).expect("borsh setup save");
    load_group.bench_function("borsh", |b| {
        b.iter(|| black_box(borsh_storage.load().expect("borsh load")))
    });

    let mut wincode_storage = Storage::<Person, Wincode>::new(Wincode);
    wincode_storage.save(&person).expect("wincode setup save");
    load_group.bench_function("wincode", |b| {
        b.iter(|| black_box(wincode_storage.load().expect("wincode load")))
    });

    let mut json_storage = Storage::<Person, Json>::new(Json);
    json_storage.save(&person).expect("json setup save");
    load_group.bench_function("json", |b| {
        b.iter(|| black_box(json_storage.load().expect("json load")))
    });
    load_group.finish();
}

fn bench_storage_convert(c: &mut Criterion) {
    let payload_sizes = [
        ("1kb", 1024usize),
        ("64kb", 64 * 1024usize),
        ("1mb", 1024 * 1024usize),
    ];

    for (label, bytes) in payload_sizes {
        let person = sample_person_with_name_len(bytes);

        let mut borsh_storage = Storage::<Person, Borsh>::new(Borsh);
        borsh_storage
            .save(&person)
            .expect("borsh setup save for convert");

        let mut wincode_storage = Storage::<Person, Wincode>::new(Wincode);
        wincode_storage
            .save(&person)
            .expect("wincode setup save for convert");

        let mut json_storage = Storage::<Person, Json>::new(Json);
        json_storage
            .save(&person)
            .expect("json setup save for convert");

        let mut group = c.benchmark_group(format!("storage_convert_{label}"));
        group.throughput(Throughput::Bytes(bytes as u64));

        group.bench_function("borsh_to_json", |b| {
            b.iter(|| {
                black_box(
                    borsh_storage
                        .convert_to(Json)
                        .expect("convert borsh->json should succeed"),
                )
            })
        });

        group.bench_function("json_to_wincode", |b| {
            b.iter(|| {
                black_box(
                    json_storage
                        .convert_to(Wincode)
                        .expect("convert json->wincode should succeed"),
                )
            })
        });

        group.bench_function("wincode_to_borsh", |b| {
            b.iter(|| {
                black_box(
                    wincode_storage
                        .convert_to(Borsh)
                        .expect("convert wincode->borsh should succeed"),
                )
            })
        });

        group.finish();
    }
}

criterion_group!(
    benches,
    bench_serialize,
    bench_deserialize,
    bench_storage_save_load,
    bench_storage_convert
);
criterion_main!(benches);
