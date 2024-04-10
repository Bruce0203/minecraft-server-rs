use criterion::{criterion_group, criterion_main, Criterion};
use minecraft_server::{
    io::prelude::{Encoder, ToIdentifier},
    protocol::v1_20_4::login::login_play::LoginPlay,
    server::game_mode::GameMode,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("encode data", |b| {
        b.iter(|| {
            let value = LoginPlay {
                entity_id: 0,
                is_hardcore: false,
                dimension_names: vec![],
                max_players: 1,
                view_distance: 01,
                simulation_distance: 1,
                reduce_debug_info: false,
                enable_respawn_screen: false,
                do_limited_crafting: false,
                dimension_type: "asdf".to_identifier(),
                dimension_name: "qwer".to_identifier(),
                hashed_seed: 123123412,
                game_mode: GameMode::Survival,
                previous_game_mode: Some(GameMode::Creative).into(),
                is_debug: true,
                is_flat: false,
                death_location: None,
                portal_cooldown: 123,
            }
            .encode();
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
