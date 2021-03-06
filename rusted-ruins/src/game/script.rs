//! Script engine implementation

use common::gamedata::*;
use common::gobj;
use common::script::*;

use crate::game::eval_expr::EvalExpr;
use crate::game::InfoGetter;

pub struct ScriptEngine {
    script: &'static Script,
    pos: ScriptPos,
    cid: Option<CharaId>,
    talking: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExecResult {
    Talk(Option<CharaId>, TalkText, bool),
    ShopBuy(CharaId),
    ShopSell,
    Quest,
    Quit,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TalkText {
    pub text_id: &'static str,
    pub choices: Option<&'static [(String, String)]>,
}

/// Unwrap Value as bool
macro_rules! as_bool {
    ($v:expr) => {{
        match $v {
            Value::Bool(v) => v,
            Value::RefUnknownVar => false,
            _ => {
                return ExecResult::Quit;
            }
        }
    }};
}

/// Unwrap Value as int
macro_rules! as_int {
    ($v:expr) => {{
        match $v {
            Value::Int(v) => v,
            _ => {
                return ExecResult::Quit;
            }
        }
    }};
}

/// Jump to the given section and continue loop.
macro_rules! jump {
    ($s:expr, $section:expr) => {{
        match $section.as_ref() {
            QUIT_SECTION => return ExecResult::Quit,
            CONTINUE_SECTION => {
                $s.pos.advance();
                continue;
            }
            _ => (),
        }
        $s.pos.set_section($section);
        continue;
    }};
}

/// Unwrap or return with warning message.
macro_rules! ur {
    ($a:expr, $e:expr) => {{
        if let Some(a) = $a {
            a
        } else {
            warn!(concat!("script error: ", $e));
            return ExecResult::Quit;
        }
    }};
}

impl ScriptEngine {
    pub fn new(id: &str, cid: Option<CharaId>) -> ScriptEngine {
        let script_obj: &ScriptObject = gobj::get_by_id(id);
        ScriptEngine {
            script: &script_obj.script,
            pos: ScriptPos {
                section: "start".to_owned(),
                i: 0,
            },
            cid,
            talking: false,
        }
    }

    pub fn exec(&mut self, gd: &mut GameData) -> ExecResult {
        let result = loop {
            let instruction = if let Some(instruction) = self.script.get(&self.pos) {
                instruction
            } else {
                break ExecResult::Quit;
            };

            match instruction {
                Instruction::Jump(section) => {
                    jump!(self, section);
                }
                Instruction::JumpIf(section, expr) => {
                    if as_bool!(expr.eval(gd)) {
                        jump!(self, section);
                    }
                }
                Instruction::Talk(text_id, choices) => {
                    let need_open_talk_dialog = if self.talking {
                        false
                    } else {
                        self.talking = true;
                        true
                    };

                    let choices = if choices.is_empty() {
                        None
                    } else {
                        Some(choices.as_ref())
                    };
                    return ExecResult::Talk(
                        self.cid,
                        TalkText { text_id, choices },
                        need_open_talk_dialog,
                    );
                }
                Instruction::GSet(name, v) => {
                    let v = v.eval(gd);
                    gd.vars.set_global_var(name, v);
                }
                Instruction::ReceiveMoney(v) => {
                    let v = v.eval(gd);
                    gd.player.add_money(as_int!(v) as i64);
                }
                Instruction::RemoveItem(item_id) => {
                    let il = ur!(gd.player_item_location(item_id), "cannot find item");
                    gd.remove_item(il, 1);
                }
                Instruction::Special(SpecialInstruction::ShopBuy) => {
                    break ExecResult::ShopBuy(ur!(self.cid, "cid is needed"));
                }
                Instruction::Special(SpecialInstruction::ShopSell) => {
                    break ExecResult::ShopSell;
                }
                Instruction::Special(SpecialInstruction::GetDungeonLocation) => {
                    let mid = gd.get_current_mapid();
                    super::region::gen_dungeon_max(gd, mid.rid());
                }
                Instruction::Special(SpecialInstruction::QuestWindow) => {
                    super::quest::update_town_quest(gd);
                    break ExecResult::Quest;
                }
                Instruction::Special(SpecialInstruction::ReceiveQuestRewards) => {
                    let result = super::quest::receive_rewards(gd);
                    gd.vars.set_last_result(Value::Bool(result))
                }
            }
            self.pos.advance();
        };

        self.pos.advance();
        result
    }

    pub fn continue_talk(&mut self, gd: &mut GameData, choice: Option<u32>) -> ExecResult {
        match self.script.get(&self.pos).expect("instruction not found") {
            Instruction::Talk(_, choices) => {
                if let Some(c) = choice {
                    let next_section = &choices[c as usize].1;
                    if next_section == QUIT_SECTION {
                        return ExecResult::Quit;
                    }
                    if next_section == CONTINUE_SECTION {
                        self.pos.advance();
                    } else {
                        self.pos.set_section(next_section);
                    }
                } else {
                    assert!(choices.is_empty());
                    self.pos.advance();
                }
                self.exec(gd)
            }
            _ => unreachable!(),
        }
    }
}
