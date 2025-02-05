use super::*;

unsafe extern "C" fn zelda_frame(fighter: &mut L2CFighterCommon) {
	if is_phosphora(fighter.module_accessor) {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
		
		// Up smash cancel helper
		if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI4 
		&& status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
			WorkModule::off_flag(fighter.module_accessor, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_ATTACK_HI_4_CANCEL);
		}
		
		// Handle projectile cooldowns
		projectile_cooldown_helper(fighter, FIGHTER_ZELDA_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN);
		projectile_cooldown_helper(fighter, FIGHTER_ZELDA_INSTANCE_WORK_ID_INT_SPECIAL_S_COOLDOWN);
		projectile_cooldown_helper(fighter, FIGHTER_ZELDA_INSTANCE_WORK_ID_INT_SPECIAL_LW_COOLDOWN);
    }
}

unsafe extern "C" fn projectile_cooldown_helper(fighter: &mut L2CFighterCommon, cooldown_const: i32) {
	let cooldown_curr = WorkModule::get_int(fighter.module_accessor, cooldown_const);
	if cooldown_curr != 0 {
		WorkModule::dec_int(fighter.module_accessor, cooldown_const);
	} 
	if cooldown_curr == 1 {
		macros::EFFECT(fighter, Hash40::new("sys_damage_aura"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
		macros::PLAY_SE(fighter, Hash40::new("se_common_elec_l_damage"));
	}
}

pub fn install() {
    Agent::new("zelda")
        .on_line(Main, zelda_frame)
        .install();
}
