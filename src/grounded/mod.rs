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

const FIGHTER_ZELDA_WORK_FLAG_ATTACK_HI_4_CANCEL : i32 = 0x200000ff;

#[acmd_script( agent = "zelda", script = "game_attack11", category = ACMD_GAME )]
unsafe fn zelda_game_attack11(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 7.0);
	if macros::is_excute(agent) {
		if PostureModule::lr(agent.module_accessor) > 0.0 {
			if ControlModule::get_stick_y(agent.module_accessor) > 0.707 {
				ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
			} else {
				if ControlModule::get_stick_y(agent.module_accessor) < -0.707 {
				ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW);
				} else {
					if ControlModule::get_stick_x(agent.module_accessor) > 0.707 {
						ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
					} else {
						if ControlModule::get_stick_x(agent.module_accessor) < -0.707 {
							ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
						} else {
							ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
						}
					}
				}
			}
			StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
		} else {
			if ControlModule::get_stick_y(agent.module_accessor) > 0.707 {
				ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
			} else {
				if ControlModule::get_stick_y(agent.module_accessor) < -0.707 {
				ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW);
				} else {
					if ControlModule::get_stick_x(agent.module_accessor) < -0.707 {
						ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
					} else {
						if ControlModule::get_stick_x(agent.module_accessor) > 0.707 {
							ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
						} else {
							ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
						}
					}
				}
			}
			StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
		}
	}
}

#[acmd_script( agent = "zelda", script = "effect_attack11", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attack11(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "zelda", script = "game_attacks3hi", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_attacks3hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 88, 126, 0, 45, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 88, 126, 0, 45, 4.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 10.0, 75, 126, 0, 45, 5.0, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 10.0, 92, 120, 0, 40, 4.0, 6.0, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "zelda", script = "effect_attacks3hi", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attacks3hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 10, 1, 0, 0, 90, 1, true, *EF_FLIP_YZ);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "zelda", script = "game_attacks3lw", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_attacks3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 74, 126, 0, 45, 5.0, 0.0, 0.0, 9.0, Some(0.0), Some(6.5), Some(9.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 1.1);
}

#[acmd_script( agent = "zelda", script = "effect_attacks3lw", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attacks3lw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_flash"), Hash40::new("havel"), 0, 1, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 4, 9, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "zelda", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 5.0, 6.5);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 85, 55, 0, 70, 2.8, 0.0, 4.5, 1.0, Some(0.0), Some(4.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 78, 55, 0, 70, 2.8, 0.0, 4.5, 1.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
}

#[acmd_script( agent = "zelda", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2.5, 2, -5, 10, 5, 1.2, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "zelda", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 65, 130, 0, 50, 3.0, 0.0, 7.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 65, 130, 0, 50, 4.0, 0.0, 7.0, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 65, 130, 0, 50, 3.0, 0.0, 7.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, 65, 130, 0, 50, 4.0, 0.0, 7.0, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 2.9);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "zelda", script = "effect_attackdash", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("trans"), 0, 8, 10, 0, 90, 0, 0.55, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "zelda", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 110, 0, 37, 5.7, 0.0, 9.1, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 110, 0, 37, 6.0, 0.0, 9.1, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
	wait(agent.lua_state_agent, 3.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}

#[acmd_script( agent = "zelda", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 10, 16, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "zelda", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
	frame(agent.lua_state_agent, 8.0);
	if ControlModule::get_stick_y(agent.module_accessor) > 0.707 {
		WorkModule::on_flag(agent.module_accessor, FIGHTER_ZELDA_WORK_FLAG_ATTACK_HI_4_CANCEL);
		PostureModule::add_pos_2d(agent.module_accessor, &Vector2f{ x: 0.0, y: 3.0 });
		KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.8, z: 0.0 });
		ControlModule::set_attack_air_kind(agent.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
		StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
	}
    frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.25, 89, 90, 0, 42, 5.8, 0.0, 18.0, 0.0, Some(0.0), Some(24.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
	} 
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "zelda", script = "effect_attackhi4", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 18, 0, 0, 0, 0, 0.4, true);
		macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 24, 0, 0, 0, 0, 0.4, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 18, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.15, 0, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 1);
    }
}

#[acmd_script( agent = "zelda", script = "effect_attackhi4charge", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attackhi4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 4, 0, 0, 0, true);
}

#[acmd_script( agent = "zelda", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 60, 90, 0, 0, 4.0, 0.0, 3.0, 13.0, Some(0.0), Some(3.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 60, 85, 0, 0, 7.0, 0.0, 3.0, 17.0, Some(0.0), Some(3.0), Some(10.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "zelda", script = "effect_attacklw4", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 4.0, 15.0, 0, 0, 0, 0.7, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "zelda", script = "effect_attacklw4charge", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_effect_attacklw4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("szero_pl_hold"), Hash40::new("haver"), 0, 1.3, 3, 0, 0, 0, 0.75, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
}

pub fn install() {
    smashline::install_acmd_scripts!(
		zelda_game_attack11,
		zelda_effect_attack11,
		zelda_game_attacks3hi,
		zelda_effect_attacks3hi,
		zelda_game_attacks3lw,
		zelda_effect_attacks3lw,
		zelda_game_attacklw3,
		zelda_effect_attacklw3,
		zelda_game_attackdash,
		zelda_effect_attackdash,
		zelda_game_attacks4,
		zelda_effect_attacks4,
		zelda_game_attackhi4,
		zelda_effect_attackhi4,
		zelda_effect_attackhi4charge,
		zelda_game_attacklw4,
		zelda_effect_attacklw4,
		zelda_effect_attacklw4charge,
    );
}