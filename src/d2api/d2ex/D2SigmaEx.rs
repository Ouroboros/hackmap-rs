use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

use D2Common::D2Unit;

use crate::d2api::d2ex::common::*;
use crate::d2api::d2consts::*;

#[repr(C, packed(4))]
struct FormatItemPropertiesContext {
    pub buf1                    : [u8; 0x100],          // 0x0000
    pub text                    : [u16; 0x4000],        // 0x0100
    pub text2                   : [u16; 0x4000],        // 0x8100
    pub text3                   : [[u16; 0x400]; 3],    // 0xC100
    pub client_unit_type_table  : PVOID,                // 0xD900
    pub unit                    : *mut D2Unit,          // 0xD904
    pub owner                   : *mut D2Unit,          // 0xD908
}

struct D2SigmaEx {
    is_getting_item_properties  : bool,
    // strip_color_code            : bool,
    item_properties             : String,
    AddCtrlPressedHintText      : Option<extern "fastcall" fn(usize)>,
    DrawItemProperties          : Option<extern "fastcall" fn(&mut FormatItemPropertiesContext, D2Font)>,
}

impl D2SigmaEx {
    const fn new() -> Self {
        Self {
            is_getting_item_properties  : false,
            // strip_color_code            : false,
            item_properties             : String::new(),
            AddCtrlPressedHintText      : None,
            DrawItemProperties          : None,
        }
    }

    #[allow(static_mut_refs)]
    pub fn get() -> &'static mut Self {
        static mut OBJ: D2SigmaEx = D2SigmaEx::new();

        unsafe {
            &mut OBJ
        }
    }

    fn get_item_properties(&mut self, unit: &D2Common::D2Unit, get_name_only: bool, strip_color_code: bool) -> String {
        let player = match D2Client::Units::GetClientPlayer() {
            None => return String::new(),
            Some(p) => p,
        };

        self.is_getting_item_properties = true;

        let mut ctx = D2Sigma::GetItemPropertiesContext::new();

        D2Sigma::ItemText::GetItemPropertiesInit(&mut ctx, player, unit, null_mut());

        D2Sigma::ItemText::GetItemProperties29(&mut ctx);
        D2Sigma::ItemText::GetName(&mut ctx);

        if get_name_only == false {
            D2Sigma::ItemText::GetItemProperties27(&mut ctx);
            D2Sigma::ItemText::GetItemProperties26(&mut ctx);
            D2Sigma::ItemText::GetItemProperties25(&mut ctx);
            D2Sigma::ItemText::GetItemProperties24(&mut ctx);
            D2Sigma::ItemText::GetItemProperties23(&mut ctx);
            D2Sigma::ItemText::GetItemProperties22(&mut ctx);
            D2Sigma::ItemText::GetItemProperties21(&mut ctx);
            D2Sigma::ItemText::GetItemProperties20(&mut ctx);
            D2Sigma::ItemText::GetItemProperties19(&mut ctx);
            D2Sigma::ItemText::GetItemProperties18(&mut ctx);
            D2Sigma::ItemText::GetItemProperties17(&mut ctx);
            D2Sigma::ItemText::GetItemProperties16(&mut ctx);
            D2Sigma::ItemText::GetItemProperties15(&mut ctx);
            D2Sigma::ItemText::GetItemProperties14(&mut ctx);
            D2Sigma::ItemText::GetItemProperties13(&mut ctx);
            D2Sigma::ItemText::GetItemProperties12(&mut ctx);
            D2Sigma::ItemText::GetItemProperties11(&mut ctx);
            D2Sigma::ItemText::GetItemProperties10(&mut ctx);
            D2Sigma::ItemText::GetItemProperties9(&mut ctx);
            D2Sigma::ItemText::GetItemProperties8(&mut ctx);
            D2Sigma::ItemText::GetItemProperties7(&mut ctx);
            D2Sigma::ItemText::GetItemProperties6(&mut ctx);
            D2Sigma::ItemText::GetItemProperties5(&mut ctx);
            D2Sigma::ItemText::GetItemProperties4(&mut ctx);
            D2Sigma::ItemText::GetItemProperties3(&mut ctx);
            D2Sigma::ItemText::GetItemProperties2(&mut ctx);
            D2Sigma::ItemText::GetItemProperties1(&mut ctx);
        }

        self.is_getting_item_properties = false;

        let text = (ctx.text.as_ptr() as PCWSTR).to_string();

        if strip_color_code == false {
            return text;
        }

        let chars: Vec<char> = text.chars().collect();
        let mut new_text = String::new();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];
            if ch == 'ÿ' && i + 1 < chars.len() && chars[i + 1] == 'c' {
                i += 3;
            } else {
                new_text.push(ch);
                i += 1;
            }
        }

        return new_text;

        // self.is_getting_item_properties = true;
        // self.strip_color_code = strip_color_code;

        // if let Some(player) = D2Client::Units::GetClientPlayer() {
        //     D2Sigma::Units::DisplayItemProperties(player, unit);
        // }

        // self.is_getting_item_properties = false;
        // self.strip_color_code = false;

        // std::mem::take(&mut self.item_properties)
    }

    // fn on_get_item_properties(&mut self, text: &str) {
    //     let mut text: Vec<&str> = text.split("\n").collect();
    //     text = text.into_iter().rev().collect();

    //     let text = text.join("\n");

    //     if self.strip_color_code == false {
    //         self.item_properties = text;
    //         return;
    //     }

    //     let chars: Vec<char> = text.chars().collect();
    //     let mut new_text = String::new();
    //     let mut i = 0;

    //     while i < chars.len() {
    //         let ch = chars[i];
    //         if ch == 'ÿ' && i + 1 < chars.len() && chars[i + 1] == 'c' {
    //             i += 3;
    //         } else {
    //             new_text.push(ch);
    //             i += 1;
    //         }
    //     }

    //     self.item_properties = new_text;
    // }

    // extern "fastcall" fn draw_item_properties(text: &mut FormatItemPropertiesContext, font: D2Font) {
    //     let sigma = D2SigmaEx::get();

    //     if sigma.is_getting_item_properties {
    //         sigma.on_get_item_properties(&(text.text.as_ptr() as PCWSTR).to_string());
    //         return;
    //     }

    //     sigma.DrawItemProperties.unwrap()(text, font)
    // }

    // extern "fastcall" fn add_ctrl_pressed_hint_text(this: usize) {
    //     let sigma = D2SigmaEx::get();

    //     if sigma.is_getting_item_properties == false {
    //         sigma.AddCtrlPressedHintText.unwrap()(this)
    //     }
    // }

}

pub mod Items {
    use super::*;

    pub fn get_item_name(unit: &D2Unit) -> String {
        D2SigmaEx::get().get_item_properties(unit, true, false)

        // let buffer: [u16; 0x1000] = [0; 0x1000];

        // D2Sigma::Items::GetItemName(unit, buffer.as_ptr(), 0);

        // String::from_utf16_lossy(buffer.as_slice())
    }

    pub fn get_item_properties(unit: &D2Unit, strip_color_code: bool) -> String {
        D2SigmaEx::get().get_item_properties(unit, false, strip_color_code)
    }

    pub fn is_getting_item_properties() -> bool {
        D2SigmaEx::get().is_getting_item_properties
    }
}

pub(super) fn init(_modules: &D2Modules) -> Result<(), HookError> {
    // let sigma = D2SigmaEx::get();

    // inline_hook_jmp(0, D2Sigma::AddressTable.UI.DrawItemProperties, D2SigmaEx::draw_item_properties as usize, Some(&mut sigma.DrawItemProperties), None)?;
    // inline_hook_jmp(0, D2Sigma::AddressTable.Items.AddCtrlPressedHintText, D2SigmaEx::add_ctrl_pressed_hint_text as usize, Some(&mut sigma.AddCtrlPressedHintText), None)?;

    Ok(())
}
