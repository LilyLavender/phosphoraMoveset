use super::*;

unsafe extern "C" fn zelda_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_phosphora(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
	PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
	PostureModule::update_rot_y_lr(fighter.module_accessor);
	fighter.sub_shift_status_main(L2CValue::Ptr(zelda_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn zelda_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
	if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    let is_end = MotionModule::is_end(fighter.module_accessor);
    if is_end {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    
    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    if is_changing || fighter.global_table[0x17].get_i32() != fighter.global_table[0x16].get_i32() {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE)
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_MOT_CHANGE)
			} else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

	return 0.into();
}

unsafe extern "C" fn zelda_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_phosphora(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    fighter.sub_status_pre_SpecialNCommon();
	StatusModule::init_settings(
		fighter.module_accessor, 
		smash::app::SituationKind(*SITUATION_KIND_NONE),  
		*FIGHTER_KINETIC_TYPE_UNIQ, 
		GROUND_CORRECT_KIND_KEEP.into(), 
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
		true, 
		FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(), 
		*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
		FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(), 
		0
	);
	FighterStatusModuleImpl::set_fighter_status_data(
		fighter.module_accessor, 
		false, 
		*FIGHTER_TREADED_KIND_NO_REAC, 
		false, 
		false, 
		false, 
		(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 
		FIGHTER_STATUS_ATTR_START_TURN.into(), 
		FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N.into(), 
		0
	);
	return 0.into();
}

unsafe extern "C" fn zelda_game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
        if WorkModule::get_int(agent.module_accessor, FIGHTER_ZELDA_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN) == 0 {
            WorkModule::set_int(agent.module_accessor, 300, FIGHTER_ZELDA_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN);
            WorkModule::set_int(agent.module_accessor, DEIN_TYPE_DIFFUSION, FIGHTER_ZELDA_INSTANCE_WORK_ID_INT_DEIN_TYPE);
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ZELDA_GENERATE_ARTICLE_DEIN, false, -1);
        }
	}
	frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
		StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END.into(), true.into());
    }
}

pub fn install() {
    Agent::new("zelda")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_main)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, zelda_specialn_pre)
        .game_acmd("game_specialn_phosphora", zelda_game_specialn, Default)
        .game_acmd("game_specialairn_phosphora", zelda_game_specialn, Default)
        .install();
}