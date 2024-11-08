use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

use crate::d2api::d2ex::common::*;
use crate::d2api::d2consts::*;

#[derive(PartialEq)]
enum GameEvent {
    JoinGame,
    GameLoop,
    LeaveGame,
}

struct NetInfo {
    pre_recv_callbacks      : Vec<Box<dyn FnMut(D2GSCmd, *mut u8)>>,
    post_recv_callbacks     : Vec<Box<dyn FnMut(D2GSCmd, *mut u8)>>,
    send_callbacks          : Vec<Box<dyn FnMut(D2GSCmd, *mut u8)>>,
}

struct GameInfo {
    event_callbacks         : Vec<Box<dyn FnMut(GameEvent)>>,
    is_in_loop              : bool,

    stub_d2_sound_cleanup   : Option<extern "stdcall" fn()>,
}

struct D2ClientEx {
    net     : NetInfo,
    game    : GameInfo,
}

impl D2ClientEx {
    const fn new() -> Self {
        Self {
            game: GameInfo{
                event_callbacks         : vec![],
                is_in_loop              : false,
                stub_d2_sound_cleanup   : None,
            },
            net: NetInfo{
                pre_recv_callbacks      : vec![],
                post_recv_callbacks     : vec![],
                send_callbacks          : vec![],
            },
        }
    }

    #[allow(static_mut_refs)]
    pub fn get() -> &'static mut Self {
        static mut OBJ: D2ClientEx = D2ClientEx::new();

        unsafe {
            &mut OBJ
        }
    }
}

pub mod Net {
    use super::*;
    use D2Client::Net::D2GSHandler;

    impl NetInfo {
        fn on_pre_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(&mut self, cb: F) {
            self.pre_recv_callbacks.push(Box::new(cb));
        }

        fn on_post_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(&mut self, cb: F) {
            self.post_recv_callbacks.push(Box::new(cb));
        }

        fn call_gscmd_handler(&mut self, handler: D2GSHandler, payload: *mut u8) {
            let cmd: D2GSCmd = read_at(payload as usize);

            for cb in self.pre_recv_callbacks.iter_mut() {
                cb(cmd , payload);
            }

            handler(payload);

            for cb in self.post_recv_callbacks.iter_mut() {
                cb(cmd , payload);
            }
        }
    }

    pub fn on_pre_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(cb: F) {
        D2ClientEx::get().net.on_pre_recv(cb)
    }

    pub fn on_post_recv<F: FnMut(D2GSCmd, *mut u8) + 'static>(cb: F) {
        D2ClientEx::get().net.on_post_recv(cb)
    }

    pub fn send_packet<T>(payload: &T) -> usize {
        let slice = unsafe {
            std::slice::from_raw_parts(addr_of!(*payload) as *const u8, std::mem::size_of::<T>())
        };

        D2Client::Net::SendPacket(slice.as_ptr(), slice.len())
    }

    #[cfg(feature = "113c")]
    pub(super) fn call_gscmd_handler() {

        let handler: D2GSHandler;
        let payload: *mut u8;

        unsafe {
            asm!(
                "",
                out("eax") handler,
                out("edi") payload,
            );
        }

        D2ClientEx::get().net.call_gscmd_handler(handler, payload)
    }

}

pub mod Game {
    use super::*;

    impl GameInfo {
        fn on_join_game<F: FnMut() + 'static>(&mut self, mut cb: F) {
            self.on(move |event| {
                if event == GameEvent::JoinGame {
                    cb();
                }
            })
        }

        fn on_game_loop<F: FnMut() + 'static>(&mut self, mut cb: F) {
            self.on(move |event| {
                if event == GameEvent::GameLoop {
                    cb();
                }
            })
        }

        fn on_leave_game<F: FnMut() + 'static>(&mut self, mut cb: F) {
            self.on(move |event| {
                if event == GameEvent::LeaveGame {
                    cb();
                }
            })
        }

        fn on<F: FnMut(GameEvent) + 'static>(&mut self, cb: F) {
            self.event_callbacks.push(Box::new(cb));
        }

        pub(super) fn join_game_pre_recv_callback(&mut self, cmd: D2GSCmd, _payload: *mut u8) {
            if self.is_in_loop {
                return;
            }

            match cmd {
                D2GSCmd::GAME_LOADING => {
                    self.is_in_loop = true;

                    for cb in self.event_callbacks.iter_mut() {
                        cb(GameEvent::JoinGame);
                    }
                },
                _ => {},
            }
        }

        fn run_game_loop(&mut self) -> i32 {
            if self.is_in_loop {
                for cb in self.event_callbacks.iter_mut() {
                    cb(GameEvent::GameLoop);
                }
            }

            D2Gfx::Window::GetState()
        }

        fn d2_sound_cleanup(&mut self) {
            self.is_in_loop = false;

            self.stub_d2_sound_cleanup.unwrap()();

            for cb in self.event_callbacks.iter_mut() {
                cb(GameEvent::LeaveGame);
            }
        }
    }

    pub fn on_join_game<F: FnMut() + 'static>(cb: F) {
        D2ClientEx::get().game.on_join_game(cb);
    }

    pub fn on_game_loop<F: FnMut() + 'static>(cb: F) {
        D2ClientEx::get().game.on_game_loop(cb);
    }

    pub fn on_leave_game<F: FnMut() + 'static>(cb: F) {
        D2ClientEx::get().game.on_leave_game(cb);
    }

    pub(super) fn d2_sound_cleanup() {
        D2ClientEx::get().game.d2_sound_cleanup()
    }

    pub(super) fn run_game_loop() -> i32 {
        D2ClientEx::get().game.run_game_loop()
    }

}
pub mod Inventory {
    use D2Common::D2Unit;

    use super::*;

    pub fn get_free_position_for_item(item: &D2Unit, inv_page: D2ItemInvPage) -> Option<(i32, i32)> {
        let player = D2Client::Units::GetClientPlayer()?;

        D2Common::Inventory::GetFreePosition(
            ptr_to_ref(player.pInventory)?,
            item,
            D2Common::Units::GetInventoryRecordId(player, inv_page, D2Client::Game::IsLodGame()),
            D2ItemInvPage::Cube,
        )
    }
}

pub mod Utils {
    use super::*;
    use D2Common::D2Unit;
    use crate::D2CommonEx;

    pub fn send_cursor_item_to_cube(cursor_item: &D2Unit, cube: &D2Unit) {
        let payload = D2Common::SCMD_PACKET_2A_ITEM_TO_CUBE{
            nHeader     : D2ClientCmd::ITEM_TO_CUBE as u8,
            dwItemGUID  : cursor_item.dwUnitId,
            dwCubeGUID  : cube.dwUnitId,
        };

        super::Net::send_packet(&payload);
    }

    pub fn cursor_item_to_cube() -> Option<()> {
        let cursor_item = D2CommonEx::Inventory::get_player_cursor_item()?;
        let cube = get_cube_from_inv()?;

        super::Inventory::get_free_position_for_item(cursor_item, D2ItemInvPage::Cube)?;

        send_cursor_item_to_cube(cursor_item, cube);

        None
    }

    pub fn get_cube_from_inv() -> Option<&'static D2Unit> {
        let player = D2Client::Units::GetClientPlayer()?;

        let mut opt_item = D2Common::Inventory::GetFirstItem(ptr_to_ref(player.pInventory)?);

        while let Some(item) = opt_item {
            loop {
                if D2Common::Inventory::UnitIsItem(item) == FALSE {
                    break;
                }

                match D2Common::Items::GetInvPage(item) {
                    D2ItemInvPage::Inventory | D2ItemInvPage::Stash => {},
                    _ => break,
                }

                if D2Common::Items::GetBaseCode(item) != D2ItemCodes::Cube {
                    break;
                }

                return Some(item);
            }
            opt_item = D2Common::Inventory::GetNextItem(item);
        }

        None
    }
}

pub(super) fn init(_modules: &D2Modules) -> Result<(), HookError> {
    let cli = D2ClientEx::get();

    inline_hook_call::<()>(0, D2Client::AddressTable.Game.Call_D2GFX_GetWindow, Game::run_game_loop as usize, None, None)?;
    inline_hook_call(0, D2Client::AddressTable.Game.Call_D2SoundCleanup, Game::d2_sound_cleanup as usize, Some(&mut cli.game.stub_d2_sound_cleanup), None)?;
    inline_hook_call::<()>(0, D2Client::AddressTable.Net.Call_GSCmdHandler, Net::call_gscmd_handler as usize, None, None)?;

    Net::on_pre_recv(move |cmd, payload| {
        cli.game.join_game_pre_recv_callback(cmd, payload);
    });

    Ok(())
}
