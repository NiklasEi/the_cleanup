use bevy::prelude::*;

use crate::digger::{Digger, DiggerState};
use crate::map::Map;
use crate::GameState;

pub struct BasePlugin;

#[derive(SystemLabel, Eq, PartialEq, Hash, Clone, Debug)]
pub enum BaseSystemLabels {
    CheckPlayerPosition,
}

impl Plugin for BasePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Base { active: false }).add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(
                    check_player_position
                        .system()
                        .label(BaseSystemLabels::CheckPlayerPosition),
                )
                .with_system(
                    fuel_up
                        .system()
                        .after(BaseSystemLabels::CheckPlayerPosition),
                ),
        );
    }
}

pub struct Base {
    pub active: bool,
}

fn check_player_position(
    digger: Query<&Transform, With<Digger>>,
    map: Res<Map>,
    mut base: ResMut<Base>,
) {
    if let Ok(transform) = digger.single() {
        base.active = Vec2::new(transform.translation.x, transform.translation.y)
            .distance(map.base)
            <= map.tile_size;
    }
}

fn fuel_up(base: Res<Base>, mut digger_state: ResMut<DiggerState>) {
    if base.active {
        let to_fuel = digger_state.fuel_max - digger_state.fuel;
        if to_fuel > digger_state.money {
            digger_state.fuel += digger_state.money;
            digger_state.money = 0.;
        } else {
            digger_state.money -= to_fuel;
            digger_state.fuel = digger_state.fuel_max;
        }
    }
}
