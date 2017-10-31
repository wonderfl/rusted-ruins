
use super::Game;
use super::animation::*;
use common::objholder::AnimImgIdx;
use common::gamedata::chara::CharaId;
use rand::{thread_rng, Rng};

pub fn attack_neighbor(game: &mut Game, attacker: CharaId, target: CharaId) {
    // Damage calculation
    let damage = {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        let mut attack_power = attacker.params.str as f64;
        let defence_power = target.params.vit as f64 / 2.0;
        attack_power *= thread_rng().gen_range(0.5, 1.0);

        let damage = attack_power - defence_power;
        if damage < 0.0 { 0 }else{ damage as i32 }
    };
    // Damage processing
    super::chara::damage(game, target, damage);
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("attack"; attacker=attacker.name, target=target.name, damage=damage);
    }
    // Animation pushing
    game.anim_queue.push_back(Animation::img_onetile(
        AnimImgIdx(0), game.gd.get_current_map().chara_pos(target).unwrap()));
}

