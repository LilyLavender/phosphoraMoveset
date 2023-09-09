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

#[status_script(agent = "zelda", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn zelda_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
	PostureModule::update_rot_y_lr(fighter.module_accessor);
	fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), false.into());
	fighter.sub_shift_status_main(L2CValue::Ptr(zelda_speciallw_main_loop as *const () as _))
}

//#[status_script(agent = "zelda", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_MAIN_LOOP_STATUS)]
unsafe extern "C" fn zelda_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
		if fighter.global_table[0x17] == *SITUATION_KIND_GROUND {
			if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
				//goto LAB_7100008fac;
				if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
					KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
					GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
					if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
						MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
						WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
					} else {
						MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
					}
				} else {
					KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
					GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
					if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
						MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
						WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
					} else {
						MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
					}
				}
			}
		}
		if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
			//goto LAB_7100008fac;
			if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
				KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
				GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
				if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
					MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
					WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
				} else {
					MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
				}
			} else {
				if CancelModule::is_enable_cancel(fighter.module_accessor) {
					if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
						if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
							return 0.into();
						}
					}
					if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
						if fighter.sub_air_check_fall_common().get_bool() {
							return 0.into();
						}
					}
				}
				if MotionModule::is_end(fighter.module_accessor) {
					if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
						if fighter.global_table[0x16] != *SITUATION_KIND_AIR {
							return 0.into();
						}
						fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
					} else {
						fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
					}
				}
				KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
				GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
				if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
					MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
					WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
				} else {
					MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
				}
			}
		} 
	} else {
		//LAB_7100008fac:
		if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
			KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
			GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
			if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
			} else {
				MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
			}
		} else {
			KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
			GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
			if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
			} else {
				MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
			}
		}
		//LAB_7100009430:
	}
	if MotionModule::is_end(fighter.module_accessor) {
		if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
			fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
		} else {
			fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
		}
	}
	return 0.into();
}

#[status_script(agent = "zelda", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn zelda_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	StatusModule::init_settings(fighter.module_accessor, 
		smash::app::SituationKind(*SITUATION_KIND_NONE),  
		*FIGHTER_KINETIC_TYPE_UNIQ, 
		GROUND_CORRECT_KIND_KEEP.into(), 
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
		true, 
		FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(), 
		*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
		FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(), 
	0);
	FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, 
		false, 
		*FIGHTER_TREADED_KIND_NO_REAC, 
		false, 
		false, 
		false, 
		(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 
		FIGHTER_STATUS_ATTR_START_TURN.into(), 
		FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW.into(), 
	0);
	return 0.into();
}

pub fn install() {
	install_status_scripts!(
	  zelda_speciallw_main,
	  zelda_speciallw_pre,
	);
}
