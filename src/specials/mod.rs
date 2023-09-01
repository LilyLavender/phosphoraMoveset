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
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

const FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT : i32 = 0x200000eb;
const FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_ALLOW_EFFECT : i32 = 0x200000ec;
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;

#[acmd_script( agent = "zelda", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 2.0, 361, 90, 0, 80, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 1.4, 361, 90, 0, 60, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.25);
}

#[acmd_script( agent = "zelda", script = "game_specialhistart", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 91, 30, 0, 118, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "zelda", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 2.4, 55, 94, 0, 80, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 1.6, 55, 90, 0, 60, 16.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL);
    }
}

#[acmd_script( agent = "zelda", script = "game_specialairhistart", category = ACMD_GAME, low_priority )]
unsafe fn zelda_game_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 1.2, 80, 100, 0, 60, 10.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "zelda", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME )]
unsafe fn zelda_game_speciallw(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(14.2), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
		WorkModule::off_flag(agent.module_accessor, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
		AttackModule::clear_all(agent.module_accessor);
	}
}

#[acmd_script( agent = "zelda", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT )]
unsafe fn zelda_effect_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
		macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
	}
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_phantom_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true)
	}
	frame(agent.lua_state_agent, 6.0);
	for _ in 1..3 {
		if macros::is_excute(agent) {
			if WorkModule::is_flag(agent.module_accessor, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_ALLOW_EFFECT) {
				macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("top"), 0, 16, 10, 0, 0, 0, 0.45, true)
			}
		}
		wait(agent.lua_state_agent, 1.0);
	}
}

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool, fighter: &mut L2CAgentBase) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
	let defender_kind = sv_battle_object::kind(defender_id);
    if attacker_kind == *FIGHTER_KIND_ZELDA && WorkModule::is_flag(attacker_boma, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT) {
        if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
			WorkModule::on_flag(attacker_boma, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_ALLOW_EFFECT); //allow effects
			if (defender_kind == *FIGHTER_KIND_LINK) { //Link
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_LINKARROW), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_KOOPAJR) { //Bowser Jr
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_MECHAKOOPA), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_KROOL) { //K Rool
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_KROOLCROWN), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_ROBOT) { //ROB
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_ROBOTGYRO), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_SIMON) { //Simon
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_SIMONHOLYWATER), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_RICHTER) { //Richter
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_RICHTERHOLYWATER), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_SNAKE) { //Snake
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_SNAKEGRENADE), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_TOONLINK) { //Toon Link
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_TOONLINKBOMB), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_YOUNGLINK) { //Young Link
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_YOUNGLINKBOMB), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_MURABITO) { //Villager
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_PEACH) { //Peach
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_PEACHDAIKON), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_DAISY) { //Daisy
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_DAISYDAIKON), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_DIDDY) { //Diddy
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_BANANA), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_BUDDY) { //Banjo
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_BUDDYBOMB), 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_PACMAN) { //PAC-Man
				let pacitem = smash::app::sv_math::rand(hash40("fighter"), 8);
				let mut pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANBOSS);
				if pacitem == 0 {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANAPPLE)
				} else if pacitem == 1 {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANBELL)
				} else if pacitem == 2 {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANBOSS)
				} else if pacitem == 3 {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANCHERRY)
				} else if pacitem == 4 {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANKEY)
				} else if pacitem == 5 {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANMELON)
				} else if pacitem == 6 {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANORANGE)
				} else {
					pacitemstring = smash::app::ItemKind(*ITEM_KIND_PACMANSTRAWBERRY)
				}
				ItemModule::have_item(attacker_boma, pacitemstring, 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_REFLET) { //Robin
				let refitem = smash::app::sv_math::rand(hash40("fighter"), 2);
				let mut refitemstring = smash::app::ItemKind(*ITEM_KIND_THUNDERSWORD);
				if refitem == 0 {
					refitemstring = smash::app::ItemKind(*ITEM_KIND_THUNDERSWORD)
				} else {
					refitemstring = smash::app::ItemKind(*ITEM_KIND_BOOK)
				}
				ItemModule::have_item(attacker_boma, refitemstring, 0, 0, false, false);
			} else if (defender_kind == *FIGHTER_KIND_ROCKMAN) { //Mega Man
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_METALBLADE), 0, 0, false, false);
			} else {											//None
				let foodtype = smash::app::sv_math::rand(hash40("fighter"), 36);
				ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_TABEMONO), foodtype, 0, false, false);
			}
        }
    } else {
		WorkModule::off_flag(attacker_boma, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_ALLOW_EFFECT); //don't allow effects
	}
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again, fighter)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    smashline::install_acmd_scripts!(
		zelda_game_specialairhi,
		zelda_game_specialairhistart,
		zelda_game_specialhi,
		zelda_game_specialhistart,
		zelda_game_speciallw,
		zelda_effect_speciallw
    );
	skyline::install_hook!(notify_log_event_collision_hit_replace);
}
