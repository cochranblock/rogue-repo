// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! rogue-runner: 1000 levels, procedural, offline. f95=mulberry32 f96=generate_level f117=zone_for_level

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub const c90: u32 = 1000;
pub const c91: f32 = 0.6;
pub const c92: f32 = -12.0;
pub const c93: f32 = 32.0;
pub const c94: f32 = 24.0;
pub const c95: f32 = 0.85;

/// Action = abstract input (frontend maps keys/touch to this)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    None,
    Jump,
    Start,
}

/// f117 = zone_for_level — levels 1–1000 map to 20 zones (50 levels each)
pub fn f117(level: u32) -> u32 {
    level.saturating_sub(1) / 50
}

/// f95 = mulberry32 — deterministic PRNG for level generation
pub fn f95(seed: u32) -> impl FnMut() -> f32 {
    let mut s = seed as i32;
    move || {
        s = s.wrapping_add(0x6D2B79F5);
        ((s as u32) as f32) / 4294967296.0
    }
}

/// t95 = Obstacle
#[derive(Clone)]
pub struct t95 {
    pub x: f32,
    pub h: f32,
    pub w: f32,
}

/// t96 = LevelData
#[derive(Clone)]
pub struct t96 {
    pub speed: f32,
    pub obstacles: Vec<t95>,
}

/// f96 = generate_level — deterministic level from seed(level_num)
pub fn f96(level_num: u32) -> t96 {
    let mut rng = f95(level_num);
    let speed = 3.0 + level_num as f32 * 0.08;
    let count = 5 + (level_num / 50) as usize;
    let mut obstacles = Vec::with_capacity(count);
    for i in 0..count {
        obstacles.push(t95 {
            x: 400.0 + (i as f32) * 220.0 + rng() * 120.0,
            h: 30.0 + rng() * 35.0,
            w: 24.0 + rng() * 16.0,
        });
    }
    t96 { speed, obstacles }
}

#[cfg(target_arch = "wasm32")]
fn storage_get(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    storage.get_item(key).ok().flatten()
}

#[cfg(target_arch = "wasm32")]
fn storage_set(key: &str, value: &str) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item(key, value);
    }
}

/// t88 = GameState. s96=run_frame s97=is_jumping (animation state)
#[derive(Clone)]
pub struct t88 {
    pub s88: String,
    pub s89: u32,
    pub s90: u32,
    pub s91: f32,
    pub s92: f32,
    pub s93: Option<t96>,
    pub s94: usize,
    pub s95: u32,
    pub s96: u32,
    pub s97: bool,
}

impl Default for t88 {
    fn default() -> Self {
        Self {
            s88: "menu".to_string(),
            s89: 1,
            s90: 0,
            s91: 0.0,
            s92: 0.0,
            s93: None,
            s94: 0,
            s95: 1,
            s96: 0,
            s97: false,
        }
    }
}

impl t88 {
    /// f105 = load_progress
    pub fn f105(&mut self) {
        self.s95 = 1;
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(l) = std::fs::read_to_string("rogue_runner_level.txt") {
            if let Ok(n) = l.trim().parse::<u32>() {
                self.s95 = n.min(c90);
            }
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(s) = storage_get("rogue_runner_level") {
            if let Ok(n) = s.parse::<u32>() {
                self.s95 = n.min(c90);
            }
        }
    }

    /// f106 = save_progress
    pub fn f106(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = std::fs::write("rogue_runner_level.txt", self.s89.to_string());
        #[cfg(target_arch = "wasm32")]
        storage_set("rogue_runner_level", &self.s89.to_string());
    }

    /// f107 = start_game (takes screen_h for ground_y)
    fn f107(&mut self, screen_h: f32) {
        self.s89 = self.s95;
        self.s93 = Some(f96(self.s89));
        self.s94 = 0;
        let gy = screen_h * c95 - c93;
        self.s91 = gy;
        self.s92 = 0.0;
        self.s88 = "play".to_string();
        self.s96 = 0;
        self.s97 = false;
    }

    /// f108 = jump
    fn f108(&mut self, screen_h: f32) {
        if self.s88 != "play" {
            return;
        }
        let gy = screen_h * c95 - c93;
        if self.s91 >= gy - 2.0 {
            self.s92 = c92;
            self.s97 = true;
        }
    }

    /// f109 = game_over
    fn f109(&mut self) {
        self.s88 = "gameover".to_string();
        self.f106();
    }

    /// f110 = level_complete
    fn f110(&mut self) {
        self.s89 = (self.s89 + 1).min(c90);
        self.s90 += self.s89;
        self.f106();
        self.s93 = Some(f96(self.s89));
        self.s94 = 0;
    }

    /// f111 = update — pure logic, no I/O. Accepts Action and dt.
    pub fn f111(&mut self, action: Action, dt: f32, screen_w: f32, screen_h: f32) {
        if action == Action::Start && (self.s88 == "menu" || self.s88 == "gameover") {
            self.f107(screen_h);
            return;
        }
        if action == Action::Jump {
            self.f108(screen_h);
        }

        if self.s88 != "play" {
            return;
        }

        let ld = self.s93.as_mut().unwrap();
        let gy = screen_h * c95 - c93;

        self.s92 += c91;
        self.s91 += self.s92 * (dt * 60.0 / 16.0);
        if self.s91 > gy {
            self.s91 = gy;
            self.s92 = 0.0;
            self.s97 = false;
        } else if self.s92 < 0.0 {
            self.s97 = true;
        }

        if self.s91 >= gy - 2.0 && self.s92 >= 0.0 {
            self.s96 = self.s96.wrapping_add(1);
        }

        let px = screen_w * 0.2;
        let pl = px;
        let pr = px + c94;
        let pt = self.s91;
        let pb = self.s91 + c93;

        let spd = ld.speed * (dt * 60.0 / 16.0);
        for (i, o) in ld.obstacles.iter_mut().enumerate() {
            o.x -= spd;
            if o.x + o.w < pl && i == self.s94 {
                self.s94 += 1;
                self.s90 += 10;
            }
            if o.x < pr && o.x + o.w > pl {
                let ot = screen_h * c95 - o.h;
                let ob = screen_h * c95;
                if pb > ot && pt < ob {
                    self.f109();
                    return;
                }
            }
        }
        if self.s94 >= ld.obstacles.len() {
            self.f110();
        }
    }
}
