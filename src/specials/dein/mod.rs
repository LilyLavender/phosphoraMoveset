use super::*;

unsafe extern "C" fn zelda_dein_move_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	let w_boma = weapon.module_accessor;
	let owner_id = WorkModule::get_int(w_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
	let o_boma = &mut *sv_battle_object::module_accessor(owner_id as u32);
	if !is_phosphora(o_boma) {
		return smashline::original_status(Exec, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE)(weapon);
    }
	let d_boma = get_boma_of_closest_player((*(w_boma)).battle_object_id as usize);
	// Do nothing if projectile is reflected
	if WorkModule::is_flag(w_boma, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLAG_REFLECT) { return 0.into(); }
	// Get projectile info
	let homing_frame = WorkModule::get_float(w_boma, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_HOMING_FRAME);
	let speed_limit = WorkModule::get_float(w_boma, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_SPEED_LIMIT);
	let accel = WorkModule::get_float(w_boma, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_ACCEL);
	let status_frame = weapon.global_table[0xe].get_f32();

	// Declare x and y speeds
	let init_speed_x = WorkModule::get_float(w_boma, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_X);
	let init_speed_y = WorkModule::get_float(w_boma, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_Y);
	let energy_type = KineticModule::get_energy(w_boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
	let speed_start_x: f32 = if status_frame != 1.0 { lua_bind::KineticEnergy::get_speed_x(energy_type) } else { init_speed_x };
	let speed_start_y: f32 = if status_frame != 1.0 { lua_bind::KineticEnergy::get_speed_y(energy_type) } else { init_speed_y };
	let speed_start = (speed_start_x.powf(2.0) + speed_start_y.powf(2.0)).powf(0.5);
	let mut speed_x = speed_start_x;
	let mut speed_y = speed_start_y;
	
	if homing_frame <= status_frame || d_boma.is_null() { // Normal movement
		// Calculate direction
		let dir_x = if speed_start != 0.0 { speed_x / speed_start } else { 0.0 };
		let dir_y = if speed_start != 0.0 { speed_y / speed_start } else { 0.0 };

		// Calculate componential acceleration
		let accel_x = accel * dir_x;
		let accel_y = accel * dir_y;

		// Add acceleration to current speeds
		speed_x = speed_x + accel_x;
		speed_y = speed_y + accel_y;
	} else { // Homing movement
		// Get positions
		let w_pos_x = PostureModule::pos_x(w_boma);
		let w_pos_y = PostureModule::pos_y(w_boma);
		let d_pos_x = PostureModule::pos_x(d_boma);
		let d_pox_y = PostureModule::pos_y(d_boma) + (WorkModule::get_param_float(d_boma, hash40("height"), 0) / 2.0);
		
		// Compute x and y components to be added to new speed
		let c = ((w_pos_x - d_pos_x).powf(2.0) + (w_pos_y - d_pox_y).powf(2.0)).powf(0.5);
		let accel_x = if accel != 0.0 { (d_pos_x - w_pos_x) / c * accel } else { (d_pos_x - w_pos_x) / c };
		let accel_y = if accel != 0.0 { (d_pox_y - w_pos_y) / c * accel } else { (d_pox_y - w_pos_y) / c };

		// Add acceleration to current speeds
		speed_x = if accel != 0.0 { speed_x + accel_x } else { speed_start * accel_x };
		speed_y = if accel != 0.0 { speed_y + accel_y } else { speed_start * accel_y };
	}

	// Limit speed
	let final_total_speed = (speed_x.powf(2.0) + speed_y.powf(2.0)).powf(0.5);
	if final_total_speed > speed_limit {
		let scale = speed_limit / final_total_speed;
		speed_x = speed_x * scale;
		speed_y = speed_y * scale;
	}
	// Set speed
	weapon.clear_lua_stack();
	weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
	weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
	weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
	sv_kinetic_energy::set_speed(weapon.lua_state_agent);
	return 0.into();
}

// Helper function to get the boma of the player closest to the player with the id passed in
unsafe extern "C" fn get_boma_of_closest_player(actor_id: usize) -> *mut smash::app::BattleObjectModuleAccessor {
    // Decs
	#[derive(Debug)]
    struct BomaData {
        boma: *mut smash::app::BattleObjectModuleAccessor,
        x_pos: f32,
        y_pos: f32,
        distance: f32,
    }
    let fighter_num = lua_bind::FighterManager::total_fighter_num(singletons::FighterManager());
    let mut bomas: Vec<BomaData> = Vec::with_capacity(fighter_num as usize);

    // Init
    for i in 0..fighter_num {
        let boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
        let x_pos = PostureModule::pos_x(boma);
        let y_pos = PostureModule::pos_y(boma);
        bomas.push(BomaData {
            boma,
            x_pos,
            y_pos,
            distance: -1.0,
        });
    }
	
    // Get actor info
    let actor_boma = sv_battle_object::module_accessor(actor_id as u32);
    let actor_x = PostureModule::pos_x(actor_boma);
    let actor_y = PostureModule::pos_y(actor_boma);
    let facing = PostureModule::lr(actor_boma);

    // Calculate distances
    for (i, curr) in bomas.iter_mut().enumerate() {
        if i == actor_id
        || (utility::get_category(&mut *actor_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON // Ignore actor's owner
        && i == WorkModule::get_int(actor_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as usize) { 
            curr.distance = -1.0;
            continue;
        }
        curr.distance = ((curr.x_pos - actor_x).powi(2) + (curr.y_pos - actor_y).powi(2)).sqrt();
    }

    // Return closest
    bomas
        .iter()
        .filter(|d| d.distance >= 0.0)
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
		.map(|fighter| fighter.boma)
		.unwrap()
}

unsafe extern "C" fn zelda_dein_move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	let w_boma = weapon.module_accessor;
	let o_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(w_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if !is_phosphora(o_boma) {
        return smashline::original_status(Main, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE)(weapon);
    }
	let dein_type = WorkModule::get_int(o_boma, FIGHTER_ZELDA_INSTANCE_WORK_ID_INT_DEIN_TYPE); // This code takes the dein_type from phosphora and applies it to the projectile. Do not change
	let facing = PostureModule::lr(w_boma);
	if dein_type == DEIN_TYPE_DIFFUSION {
		WorkModule::set_float(w_boma, 60.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
		WorkModule::set_float(w_boma, -1.0, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_HOMING_FRAME);
		WorkModule::set_float(w_boma, 2.2, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_SPEED_LIMIT);
		WorkModule::set_float(w_boma, 0.1, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_ACCEL);
		let mut speed_x = 1.0;
		let mut speed_y = 0.0;
		let stick_y = ControlModule::get_stick_y(o_boma);
		if stick_y > 0.5 {
			speed_x = 0.866;
			speed_y = 0.5;
		} else if stick_y < -0.5 {
			speed_x = 0.866;
			speed_y = -0.5;
		}
		WorkModule::set_float(w_boma, speed_x * facing, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_X);
		WorkModule::set_float(w_boma, speed_y, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_Y);
	} else if dein_type == DEIN_TYPE_SPEAD {
		WorkModule::set_float(w_boma, 50.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
		WorkModule::set_float(w_boma, 2.0, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_HOMING_FRAME);
		WorkModule::set_float(w_boma, 3.6, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_SPEED_LIMIT);
		WorkModule::set_float(w_boma, 0.0, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_ACCEL);
		WorkModule::set_float(w_boma, 3.6 * facing, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_X);
		WorkModule::set_float(w_boma, 0.0, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_Y);
	} else if dein_type == DEIN_TYPE_BIG {
		WorkModule::set_float(w_boma, 180.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
		WorkModule::set_float(w_boma, 30.0, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_HOMING_FRAME);
		WorkModule::set_float(w_boma, 3.6, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_SPEED_LIMIT);
		WorkModule::set_float(w_boma, 0.033, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLOAT_ACCEL);
		WorkModule::set_float(w_boma, 0.0, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_X);
		WorkModule::set_float(w_boma, 0.0, WEAPON_ZELDA_DEIN_STATUS_WORK_ID_INIT_SPEED_Y);
	}
	WorkModule::set_int(w_boma, dein_type, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_INT_TYPE);
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
	weapon.fastshift(L2CValue::Ptr(zelda_dein_move_main_loop as *const () as _))
}

unsafe extern "C" fn zelda_dein_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
	let w_boma = weapon.module_accessor;
	if WorkModule::get_float(w_boma, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE) > 0.0 {
		WorkModule::add_float(w_boma, -1.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
		return 0.into();
	}
	weapon.change_status(WEAPON_ZELDA_DEIN_STATUS_KIND_TAME.into(), false.into());
	return 0.into();
}

unsafe extern "C" fn zelda_dein_game_move(agent: &mut L2CAgentBase) {
	let dein_type = WorkModule::get_int(agent.module_accessor, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_INT_TYPE);
	if dein_type == DEIN_TYPE_DIFFUSION {
		if macros::is_excute(agent) {
			macros::ATTACK_FP(agent, 0, 0, Hash40::new("top"), 6.0, 60, 80, 0, 40, 3.8, 0.0, 0.0, 0.0, Hash40::new("collision_attr_elec"), 0.0, 1.0, 1.0, false, false, 0, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *COLLISION_SITUATION_MASK_GA, false, *ATTACK_REGION_NONE, *COLLISION_CATEGORY_MASK_ALL, false, *COLLISION_PART_MASK_ALL, false, true, true, true, 0, false, false, *ATTACK_LR_CHECK_SPEED, false, false, false, false, false, *COLLISION_SHAPE_TYPE_SPHERE);
		}
	} else if dein_type == DEIN_TYPE_SPEAD {
		if macros::is_excute(agent) {
			macros::ATTACK_FP(agent, 0, 0, Hash40::new("top"), 5.0, 60, 40, 0, 20, 2.0, 0.0, 0.0, 0.0, Hash40::new("collision_attr_elec"), 0.0, 1.0, 1.0, false, false, 0, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *COLLISION_SITUATION_MASK_GA, false, *ATTACK_REGION_NONE, *COLLISION_CATEGORY_MASK_ALL, false, *COLLISION_PART_MASK_ALL, false, true, true, true, 0, false, false, *ATTACK_LR_CHECK_SPEED, false, false, false, false, false, *COLLISION_SHAPE_TYPE_SPHERE);
		}
	} else if dein_type == DEIN_TYPE_BIG {
		if macros::is_excute(agent) {
			macros::ATTACK_FP(agent, 0, 0, Hash40::new("top"), 8.0, 45, 100, 0, 30, 7.8, 0.0, 0.0, 0.0, Hash40::new("collision_attr_elec"), 0.0, 1.0, 1.4, false, false, 0, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *COLLISION_SITUATION_MASK_GA, false, *ATTACK_REGION_NONE, *COLLISION_CATEGORY_MASK_ALL, false, *COLLISION_PART_MASK_ALL, false, true, true, true, 0, false, false, *ATTACK_LR_CHECK_SPEED, false, false, false, false, false, *COLLISION_SHAPE_TYPE_SPHERE);
		}
	}
}

unsafe extern "C" fn zelda_dein_effect_move(agent: &mut L2CAgentBase) {
	let dein_type = WorkModule::get_int(agent.module_accessor, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_INT_TYPE);
	if dein_type == DEIN_TYPE_DIFFUSION {
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("eleka_diffusion_shot"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1.0, false);
		}
	} else if dein_type == DEIN_TYPE_SPEAD {
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("eleka_spead_shot"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1.0, false);
		}
	} else if dein_type == DEIN_TYPE_BIG {
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("eleka_big_shot"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1.0, false);
		}
	}
}

unsafe extern "C" fn zelda_dein_s_game_move(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn zelda_dein_s_effect_move(agent: &mut L2CAgentBase) { 
}

unsafe extern "C" fn zelda_dein_s_sound_move(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn zelda_dein_move_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
	let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
	let o_boma = &mut *sv_battle_object::module_accessor(owner_id as u32);
	if !is_phosphora(o_boma) {
        return smashline::original_status(Pre, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE)(weapon);
    }
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 
        0
    );
    return 0.into();
}

pub fn install() {
    Agent::new("zelda_dein")
		.status(Exec, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, zelda_dein_move_exec)
        .status(Main, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, zelda_dein_move_main)
		.status(Pre, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, zelda_dein_move_pre)
        .game_acmd("game_move_phosphora", zelda_dein_game_move, Default)
		.effect_acmd("effect_move_phosphora", zelda_dein_effect_move, Default)
		.install();
    Agent::new("zelda_dein_s")
        .game_acmd("game_move_phosphora", zelda_dein_s_game_move, Default)
		.effect_acmd("effect_move_phosphora", zelda_dein_s_effect_move, Default)
        .sound_acmd("sound_move_phosphora", zelda_dein_s_sound_move, Default)
		.install();
}