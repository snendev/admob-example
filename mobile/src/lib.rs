use bevy::window::WindowMode;
use bevy::{prelude::*, window::ApplicationLifetime};

use snendev_ad_example::MyGamePlugin;

#[bevy_main]
fn main() {
    println!("NEW APP HELLO");
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }),
        MyGamePlugin,
    ))
    .add_systems(Update, handle_application_lifetime)
    .add_systems(Startup, load_scene);

    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off)
        .add_state::<AdRewardState>()
        .add_systems(Startup, ad_button_ui)
        .add_systems(
            OnEnter(AdRewardState::AwaitingReward),
            start_ad_activity.map(|result| {
                result.unwrap();
            }),
        )
        .add_systems(
            Update,
            ad_button_interactions.run_if(in_state(AdRewardState::NoAd)),
        )
        .add_systems(
            Update,
            ad_button_interactions.run_if(in_state(AdRewardState::Rewarded)),
        )
        .add_systems(
            OnEnter(AdRewardState::NoAd),
            show_load_ad_ui,
        )
        .add_systems(
            OnEnter(AdRewardState::Rewarded),
            show_rewarded_ui,
        )
        .add_systems(
            Update,
            detect_ad_reward
                .map(|result| {
                    result.unwrap();
                })
                .run_if(in_state(AdRewardState::AwaitingReward)),
        );

    app.run();
}

#[derive(Resource)]
pub struct SceneLoaded;

#[derive(Resource)]
pub struct SceneSpawner(Handle<DynamicScene>);

fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load("scenes/level1.scn.ron"),
        ..default()
    });
}

#[derive(Component)]
struct AdButton;

fn ad_button_ui(mut commands: Commands) {
    commands
        .spawn((
            AdButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(180.),
                    height: Val::Px(65.),
                    margin: UiRect::all(Val::Percent(30.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
}

fn show_load_ad_ui(mut commands: Commands, ui_query: Query<Entity, With<AdButton>>) {
    let Ok(entity) = ui_query.get_single() else { return ;};
    commands.entity(entity).despawn_descendants();
    commands.entity(entity).with_children(|builder| {
        builder.spawn(TextBundle::from_section("Load Ad", TextStyle {
            font_size: 24.,
            color: Color::BLACK,
            ..Default::default()
        }));
    });
}

fn show_rewarded_ui(mut commands: Commands, ui_query: Query<Entity, With<AdButton>>) {
    let Ok(entity) = ui_query.get_single() else { return ;};
    commands.entity(entity).despawn_descendants();
    commands.entity(entity).with_children(|builder| {
        builder.spawn(TextBundle::from_section("Ad Redeemed!", TextStyle {
            font_size: 32.,
            color: Color::BLACK,
            ..Default::default()
        }));
    });
}

fn ad_button_interactions(
    mut state: ResMut<NextState<AdRewardState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<AdButton>)>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                state.set(AdRewardState::AwaitingReward);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(States)]
enum AdRewardState {
    #[default]
    NoAd,
    AwaitingReward,
    Rewarded,
}

// Pause audio when app goes into background and resume when it returns.
// This is handled by the OS on iOS, but not on Android.
fn handle_application_lifetime(
    mut lifetime_events: EventReader<ApplicationLifetime>,
    // music_controller: Query<&AudioSink>,
) {
    for _event in lifetime_events.read() {
        // match event {
        //     ApplicationLifetime::Suspended => music_controller.single().pause(),
        //     ApplicationLifetime::Resumed => music_controller.single().play(),
        //     ApplicationLifetime::Started => (),
        // }
    }
}

#[cfg(target_os = "android")]
fn start_ad_activity() -> anyhow::Result<()> {
    info!("Loading an ad!");
    let ctx = bevy::winit::ANDROID_APP
        .get()
        .expect("ANDROID_APP should be defined");
    let vm = unsafe {
        jni::JavaVM::from_raw(ctx.vm_as_ptr() as *mut *const jni::sys::JNIInvokeInterface_)
    }?;
    let activity = unsafe {
        jni::objects::JObject::from_raw(ctx.activity_as_ptr() as *mut jni::sys::_jobject)
    };
    let env = vm.attach_current_thread()?;

    env.call_method(activity, "startAdActivity", "()V", &[])?;
    Ok(())
}

// #[cfg(target_os = android)]
fn detect_ad_reward(mut commands: Commands, mut state: ResMut<NextState<AdRewardState>>) -> anyhow::Result<()> {
    info!("Detecting ad rewards!");
    #[cfg(target_os = "android")]
    let ctx = bevy::winit::ANDROID_APP
        .get()
        .expect("ANDROID_APP should be defined");
    #[cfg(target_os = "android")]
    let vm = unsafe {
        jni::JavaVM::from_raw(ctx.vm_as_ptr() as *mut *const jni::sys::JNIInvokeInterface_)
    }?;
    #[cfg(target_os = "android")]
    let activity = unsafe {
        jni::objects::JObject::from_raw(ctx.activity_as_ptr() as *mut jni::sys::_jobject)
    };
    #[cfg(not(target_os = "android"))]
    let vm: jni::JavaVM = ();
    #[cfg(not(target_os = "android"))]
    let activity = jni::objects::JObject::null();

    let env = vm.attach_current_thread()?;

    if env
        .call_method(activity, "didEarnReward", "()Z", &[])?
        .z()
        .is_ok_and(|is_true| is_true)
    {
        state.set(AdRewardState::Rewarded);
        info!("Ad reward received");
    } else if env
        .call_method(activity, "didCancel", "()Z", &[])?
        .z()
        .is_ok_and(|is_true| is_true)
    {
        state.set(AdRewardState::NoAd);
        info!("Ad canceled");
    }
    Ok(())
}
