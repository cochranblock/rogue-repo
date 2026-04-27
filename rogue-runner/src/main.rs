// Unlicense — public domain — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! rogue-runner: f0=main. Frontend: input, render, assets. Backend: t88 in lib.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]

use macroquad::prelude::*;
use rogue_runner::{f117, t88, t35};

fn window_conf() -> Conf {
    Conf {
        window_title: "Rogue Runner".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

fn asset_path(zone: u32, name: &str) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        format!(
            "{}/assets/zones/{:02}/{}",
            env!("CARGO_MANIFEST_DIR"),
            zone.min(19),
            name
        )
    }
    #[cfg(target_arch = "wasm32")]
    {
        format!(
            "/assets/apps/rogue-runner-wasm/zones/{:02}/{}",
            zone.min(19),
            name
        )
    }
}

fn player_path(name: &str) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        format!("{}/assets/player/{}", env!("CARGO_MANIFEST_DIR"), name)
    }
    #[cfg(target_arch = "wasm32")]
    {
        format!("/assets/apps/rogue-runner-wasm/player/{}", name)
    }
}

struct ZoneAssets {
    bg: Option<Texture2D>,
    ground: Option<Texture2D>,
    obstacles: Option<Texture2D>,
}

struct PlayerAssets {
    run: Option<Texture2D>,
    jump: Option<Texture2D>,
}

async fn load_zone(zone: u32) -> ZoneAssets {
    let bg = load_texture(&asset_path(zone, "bg.png")).await.ok();
    let ground = load_texture(&asset_path(zone, "ground.png")).await.ok();
    let obstacles = load_texture(&asset_path(zone, "obstacles.png")).await.ok();
    for tex in [&bg, &ground, &obstacles].into_iter().flatten() {
        tex.set_filter(FilterMode::Nearest);
    }
    ZoneAssets { bg, ground, obstacles }
}

async fn load_player() -> PlayerAssets {
    let run = load_texture(&player_path("run.png")).await.ok();
    let jump = load_texture(&player_path("jump.png")).await.ok();
    for tex in [&run, &jump].into_iter().flatten() {
        tex.set_filter(FilterMode::Nearest);
    }
    PlayerAssets { run, jump }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut st = t88::default();
    st.f105();

    let mut zone_cache: Option<(u32, ZoneAssets)> = None;
    let player = load_player().await;

    loop {
        let action = if st.s88 == "menu" || st.s88 == "gameover" {
            if is_key_pressed(KeyCode::Space)
                || is_key_pressed(KeyCode::Up)
                || is_mouse_button_pressed(MouseButton::Left)
            {
                t35::Start
            } else {
                t35::None
            }
        } else if is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Up)
            || is_mouse_button_pressed(MouseButton::Left)
        {
            t35::Jump
        } else {
            t35::None
        };

        let dt = get_frame_time();
        let sw = screen_width();
        let sh = screen_height();
        st.f111(action, dt, sw, sh);

        let zone_assets = if st.s88 == "play" {
            let zone = f117(st.s89);
            if zone_cache.as_ref().map(|(z, _)| *z != zone).unwrap_or(true) {
                zone_cache = Some((zone, load_zone(zone).await));
            }
            zone_cache.as_ref().map(|(_, a)| a)
        } else {
            None
        };

        f112(&st, zone_assets, &player);
        next_frame().await;
    }
}

fn f112(
    st: &t88,
    zone_assets: Option<&ZoneAssets>,
    player: &PlayerAssets,
) {
    use rogue_runner::{c93, c94, c95};

    if st.s88 == "menu" || st.s88 == "gameover" {
        clear_background(BLACK);
        let msg = if st.s88 == "gameover" {
            format!("Game Over. Level {}. Space or Up to retry.", st.s89)
        } else if st.s95 > 1 {
            format!(
                "Resume from Level {}. 1000 levels. Space, Up arrow, or click.",
                st.s95
            )
        } else {
            "1000 levels. Jump to survive. Space, Up arrow, or click.".to_string()
        };
        draw_text(
            "Rogue Runner",
            screen_width() / 2.0 - 80.0,
            screen_height() / 2.0 - 40.0,
            32.0,
            SKYBLUE,
        );
        draw_text(
            &msg,
            screen_width() / 2.0 - 150.0,
            screen_height() / 2.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Space, Up arrow, or click to Play",
            screen_width() / 2.0 - 140.0,
            screen_height() / 2.0 + 40.0,
            18.0,
            SKYBLUE,
        );
        return;
    }

    let gy = screen_height() * c95;
    let sw = screen_width();
    let sh = screen_height();

    if let Some(za) = zone_assets {
        if let Some(ref bg) = za.bg {
            draw_texture_ex(
                bg,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(sw, sh)),
                    ..Default::default()
                },
            );
        } else {
            clear_background(BLACK);
        }
    } else {
        clear_background(BLACK);
    }

    if let Some(za) = zone_assets {
        if let Some(ref ground) = za.ground {
            let gw = ground.width();
            let _gh = ground.height();
            let mut x = 0.0f32;
            while x < sw {
                draw_texture_ex(
                    ground,
                    x,
                    gy,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(gw, sh - gy)),
                        ..Default::default()
                    },
                );
                x += gw;
            }
        } else {
            draw_rectangle(0.0, gy, sw, sh - gy, DARKGRAY);
        }
    } else {
        draw_rectangle(0.0, gy, sw, sh - gy, DARKGRAY);
    }

    let px = sw * 0.2;
    let run_frames = 4u32;
    let frame_w = 24.0f32;

    if let Some(ref run) = player.run {
        if st.s97 {
            if let Some(ref jump) = player.jump {
                draw_texture(jump, px, st.s91, WHITE);
            } else {
                draw_rectangle(px, st.s91, c94, c93, SKYBLUE);
            }
        } else {
            let frame = (st.s96 / 4) % run_frames;
            let src_x = frame as f32 * frame_w;
            draw_texture_ex(
                run,
                px,
                st.s91,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(src_x, 0.0, frame_w, 32.0)),
                    dest_size: Some(Vec2::new(c94, c93)),
                    ..Default::default()
                },
            );
        }
    } else {
        draw_rectangle(px, st.s91, c94, c93, SKYBLUE);
    }

    if let Some(ref ld) = st.s93 {
        let obs_tex = zone_assets.and_then(|za| za.obstacles.as_ref());
        let obs_cell = 64.0f32;
        for (i, o) in ld.obstacles.iter().enumerate() {
            if o.x + o.w > 0.0 {
                if let Some(tex) = obs_tex {
                    let idx = i % 4;
                    let src_x = (idx % 4) as f32 * obs_cell;
                    let src_y = (idx / 4) as f32 * obs_cell;
                    draw_texture_ex(
                        tex,
                        o.x,
                        gy - o.h,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(src_x, src_y, obs_cell, obs_cell)),
                            dest_size: Some(Vec2::new(o.w, o.h)),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_rectangle(o.x, gy - o.h, o.w, o.h, ORANGE);
                }
            }
        }
    }

    draw_text(
        &format!("Level {} · {}", st.s89, st.s90),
        12.0,
        28.0,
        20.0,
        SKYBLUE,
    );
}
