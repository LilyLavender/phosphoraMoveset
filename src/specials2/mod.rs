use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
		lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*
};
use smash::lib::L2CValue;
use std::f32::consts::PI;

static mut n_cooldown: [f32; 8] = [180.0; 8];
static mut s_cooldown: [f32; 8] = [180.0; 8];

const FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_REPLACE : i32 = 0x200000fe;
const WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLAG_LOCK_TYPE : i32 = 0x20000018;
const FIGHTER_ZELDA_WORK_FLAG_SPECIAL_N_UNAVAILABLE : i32 = 0x200000d1;
const FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_UNAVAILABLE : i32 = 0x200000d2;

const FIGHTER_ZELDA_WORK_FLAG_ATTACK_HI_4_CANCEL : i32 = 0x200000ff;

#[status_script(agent = "zelda_dein", status = WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn zelda_dein_move_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLAG_REFLECT) {
		if WorkModule::is_flag(weapon.module_accessor, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLAG_LOCK_TYPE) {
			// Declare x and y speeds
			let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
			let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
			let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);
			
			// Declare defender boma (only works in 2p)
			let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
			let mut bomaInt = 1;
			if WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == 1 {
				bomaInt = 0;
			}
			let DEFboma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(bomaInt));
			
			// Get positions
			let b1x = PostureModule::pos_x(weapon.module_accessor);
			let b1y = PostureModule::pos_y(weapon.module_accessor);
			let b2x = PostureModule::pos_x(DEFboma);
			let b2y = PostureModule::pos_y(DEFboma) + (WorkModule::get_param_float(DEFboma, hash40("height"), 0) / 2.0);
			
			// Compute x and y components to be added to new speed
			let c = ((b1x - b2x).powf(2.0) + (b1y - b2y).powf(2.0)).powf(0.5);
			let xcomp = (b2x - b1x) / c;
			let ycomp = (b2y - b1y) / c;
			
			// Add speed until max speed is reached
			let accel = 0.06; 
			speed_x = speed_x + accel;
			speed_y = speed_y + accel;
			let speed_max = 4.8; 
			if speed_max < speed_x {
				speed_x = speed_max;
			}
			if speed_max < speed_y {
				speed_y = speed_max;
			}
			
			let mut dein_speed = WorkModule::get_float(weapon.module_accessor, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_SPEED);
			let accel = 0.06; 
			dein_speed = dein_speed + accel;
			let speed_max = 4.8; 
			if speed_max < dein_speed {
				dein_speed = speed_max;
			}
			WorkModule::set_float(weapon.module_accessor, dein_speed, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_SPEED);
			
			// Multiply original speed by x and y components
			let speed_x_final = (dein_speed + 2.0 * speed_x) / 3.0 * xcomp;
			let speed_y_final = (dein_speed + 2.0 * speed_y) / 3.0 * ycomp;
			
			// Set speed
			weapon.agent.clear_lua_stack();
			weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
			weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x_final));
			weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y_final));
			sv_kinetic_energy::set_speed(weapon.lua_state_agent);
		} else {
			let facing = PostureModule::lr(weapon.module_accessor);
			
			let mut dein_speed = WorkModule::get_float(weapon.module_accessor, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_SPEED);
			let accel = 0.10; 
			dein_speed = dein_speed + accel;
			let speed_max = 2.2; 
			if speed_max < dein_speed {
				dein_speed = speed_max;
			}
			WorkModule::set_float(weapon.module_accessor, dein_speed, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_SPEED);
			
			weapon.agent.clear_lua_stack();
			weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
			weapon.agent.push_lua_stack(&mut L2CValue::new_num(dein_speed * facing));
			weapon.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
			sv_kinetic_energy::set_speed(weapon.lua_state_agent);
		}
	}
	return 0.into();
}

#[status_script(agent = "zelda_dein", status = WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn zelda_dein_move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	if !WorkModule::is_flag(owner_boma, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_REPLACE) {
		WorkModule::on_flag(owner_boma, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_UNAVAILABLE);
		WorkModule::on_flag(weapon.module_accessor, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLAG_LOCK_TYPE);
		WorkModule::set_float(weapon.module_accessor, 120.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
	} else {
		WorkModule::on_flag(owner_boma, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_N_UNAVAILABLE);
		WorkModule::off_flag(weapon.module_accessor, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLAG_LOCK_TYPE);
		WorkModule::set_float(weapon.module_accessor, 40.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
	}
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
	weapon.fastshift(L2CValue::Ptr(zelda_dein_move_main_loop as *const () as _))
}

unsafe extern "C" fn zelda_dein_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
	if WorkModule::get_float(weapon.module_accessor, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE) > 0.0 {
		WorkModule::add_float(weapon.module_accessor, -1.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
		if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
			return 0.into();
		}
		return 0.into();
	}
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	weapon.change_status(WEAPON_ZELDA_DEIN_STATUS_KIND_TAME.into(), false.into());
	return 0.into();
}

#[acmd_script( agent = "zelda", scripts = [ "game_specialsstart", "game_specialairsstart" ], category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		if !WorkModule::is_flag(agent.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_REPLACE) {
			if !WorkModule::is_flag(agent.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_UNAVAILABLE) {
				// This flag allows the projectile to be summoned
				WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_1);
			}
		} else {
			if !WorkModule::is_flag(agent.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_N_UNAVAILABLE) {
				// This flag allows the projectile to be summoned
				WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_1);
			}
		}
	}
	frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
		StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }
}

#[acmd_script( agent = "zelda", scripts = [ "game_specialsend", "game_specialairsend" ], category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_2);
    }
}

#[acmd_script( agent = "zelda_dein", script = "game_move", category = ACMD_GAME, low_priority )]
unsafe fn zelda_dein_game_move(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
		if WorkModule::is_flag(agent.module_accessor, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLAG_LOCK_TYPE) {
			macros::ATTACK_FP(agent, 0, 0, Hash40::new("top"), 10.0, 45, 100, 0, 30, 7.8, 0.0, 0.0, 0.0, Hash40::new("collision_attr_elec"), 0.0, 1.0, 1.4, false, false, 0, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *COLLISION_SITUATION_MASK_GA, false, *ATTACK_REGION_NONE, *COLLISION_CATEGORY_MASK_ALL, false, *COLLISION_PART_MASK_ALL, false, true, true, true, 0, false, false, *ATTACK_LR_CHECK_SPEED, false, false, false, false, false, *COLLISION_SHAPE_TYPE_SPHERE);
		} else {
			macros::ATTACK_FP(agent, 0, 0, Hash40::new("top"), 7.0, 361, 50, 0, 60, 5.8, 0.0, 0.0, 0.0, Hash40::new("collision_attr_elec"), 0.0, 1.0, 1.4, false, false, 0, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *COLLISION_SITUATION_MASK_GA, false, *ATTACK_REGION_NONE, *COLLISION_CATEGORY_MASK_ALL, false, *COLLISION_PART_MASK_ALL, false, true, true, true, 0, false, false, *ATTACK_LR_CHECK_SPEED, false, false, false, false, false, *COLLISION_SHAPE_TYPE_SPHERE);
		}
	}
}

#[acmd_script( agent = "zelda_dein", script = "effect_move", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_dein_effect_move(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
		if WorkModule::is_flag(agent.module_accessor, WEAPON_ZELDA_DEIN_INSTANCE_WORK_ID_FLAG_LOCK_TYPE) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_din_bullet"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 2.0, false);
			macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.8, false);
		} else {
			macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_din_bullet"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1.5, false);
			macros::LAST_EFFECT_SET_COLOR(agent, 1.2, 0.7, 0.0);
		}
	}	
}

#[acmd_script( agent = "zelda", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_specialn(agent: &mut L2CAgentBase) {
	WorkModule::on_flag(agent.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_REPLACE);
	StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
}

#[acmd_script( agent = "zelda_dein_s", script = "game_move", category = ACMD_GAME, low_priority )]
unsafe fn zelda_dein_s_game_move(agent: &mut L2CAgentBase) {
    /*if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 52, 97, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 65, 40, 0, 55, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 2, 0.05, 200, 1, 0, 0, 12, 60);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }*/
}

#[acmd_script( agent = "zelda_dein_s", script = "effect_move", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_dein_s_effect_move(agent: &mut L2CAgentBase) {
    /*if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("zelda_din_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }*/
}

#[acmd_script( agent = "zelda_dein_s", script = "sound_move", category = ACMD_SOUND, low_priority )]
unsafe fn zelda_dein_s_sound_move(agent: &mut L2CAgentBase) {
    /*frame(agent.lua_state_agent, 0.0);
    WorkModule::get_float(agent.module_accessor, *WEAPON_ZELDA_DEIN_S_INSTANCE_WORK_ID_FLOAT_RATE);
    if(0x10db70(527766696, 0.475)){
		if macros::is_excute(agent) {
			macros::PLAY_SE_REMAIN(agent, Hash40::new("se_zelda_special_s02"));
		} else {
			WorkModule::get_float(agent.module_accessor, *WEAPON_ZELDA_DEIN_S_INSTANCE_WORK_ID_FLOAT_RATE);
			if(0x10db70(527766696, 0.975)){
				if macros::is_excute(agent) {
					macros::PLAY_SE_REMAIN(agent, Hash40::new("se_zelda_special_s03"));
				} else {
					if macros::is_excute(agent) {
						macros::PLAY_SE_REMAIN(agent, Hash40::new("se_zelda_special_s04"));
					}
				}
			}
		}
	}*/
}

#[fighter_frame(agent = FIGHTER_KIND_ZELDA)]
fn zelda_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		
		// Slow time upon parry
		if  MotionModule::motion_kind(fighter.module_accessor) == hash40("just_shield_off") &&
			MotionModule::frame(fighter.module_accessor) < 2.0 {
			macros::SLOW_OPPONENT(fighter, 3.0, 80.0);
			macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
		}
		
		// Up smash cancel helper
		if  MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_hi_4") &&
			MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_hi") {
			WorkModule::off_flag(fighter.module_accessor, FIGHTER_ZELDA_WORK_FLAG_ATTACK_HI_4_CANCEL);
		}
		
		// Neutral b helper
		if  MotionModule::motion_kind(fighter.module_accessor) != hash40("special_n") && 
			MotionModule::motion_kind(fighter.module_accessor) != hash40("special_air_n") && 
			MotionModule::motion_kind(fighter.module_accessor) != hash40("special_s_start") && 
			MotionModule::motion_kind(fighter.module_accessor) != hash40("special_air_s_start") {
			WorkModule::off_flag(fighter.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_REPLACE);
		}
		
		// Projectile cooldown helper
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_UNAVAILABLE) {
			s_cooldown[entry_id] = s_cooldown[entry_id] - 1.0;
			if s_cooldown[entry_id] <= 0.0 {
				s_cooldown[entry_id] = 180.0;
				WorkModule::off_flag(fighter.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_S_UNAVAILABLE);
			}
		}
		if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_N_UNAVAILABLE) {
			n_cooldown[entry_id] = n_cooldown[entry_id] - 1.0;
			if n_cooldown[entry_id] <= 0.0 {
				n_cooldown[entry_id] = 180.0;
				WorkModule::off_flag(fighter.module_accessor, FIGHTER_ZELDA_WORK_FLAG_SPECIAL_N_UNAVAILABLE);
			}
		}
		
		
	}
}

pub fn install() {
	smashline::install_agent_frames!(
        zelda_frame,
    );
	install_status_scripts!(
        zelda_dein_move_exec,
		zelda_dein_move_main,
    );
	smashline::install_acmd_scripts!(
		zelda_dein_game_move,
		zelda_dein_effect_move,
		zelda_game_specialsstart,
		zelda_game_specialsend,
		zelda_game_specialn,
		zelda_dein_s_game_move,
		zelda_dein_s_effect_move,
		zelda_dein_s_sound_move,
    );
}