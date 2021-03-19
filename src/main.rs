/*
This game uses an ECS approach, which stands for Entity Component System.
This means everything with physical form is an entitiy that implements one or more components to create a larger whole.
The ECS is provided from the specs library, and it is a central control element.
*/

use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;


/*Derive is a library short-hand for implementing the desired interface for that struct. So position is a component (building block) for entities such as players*/
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)] 
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct LeftMover {

}
 
#[derive(Component, Debug)]
struct Player {

}

/*A world is an instruction from the library Specs that can register components. Can be considered like a constructor*/
struct State {
    ecs: World
}

struct LeftWalker {

}

fn main() -> rltk::BError {
    use rltk::RltkBuilder; /*This is only used in main so just include in this scope*/
    let context = RltkBuilder::simple80x50() /*Build the window*/
        .with_title("KazooGame") /*Title of the window*/
        .build()?; /*Build the window with the options so far. ? is an operator the lets rust know this can fail, and should return early if an error occurs*/
    let mut gs = State {
        ecs: World::new() /*gs is the GameState. It instantiates a new world*/
    };
    gs.ecs.register::<Position>(); /*Register all the components that an entity can have*/
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs /*This should make sense by itself. An entity is created with the desired traits, such as position and it is a player*/
        .create_entity() 
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build(); /*Build the entity*/

    for i in 0..10 { /*Create 10 entities with these relevant traits*/
        gs.ecs
        .create_entity()
        .with(Position { x: i * 7, y: 20 })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(LeftMover{})
        .build();
    }

    rltk::main_loop(context, gs)/*main_loop comes from the library*/
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>(); /*Gain write access to the entity's position*/
    let mut players = ecs.write_storage::<Player>(); /*Gain write access to the entity's player component*/

    for (_player, pos) in (&mut players, &mut positions).join() { /*Return only entities with player components*/
        pos.x = min(79 , max(0, pos.x + delta_x)); /*Move relevant entities (only the player) inside the bounds of the screen*/
        pos.y = min(49, max(0, pos.y + delta_y)); 
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {/*Match is like a switch in rust. This match matches whether or not any key was pressed*/
        None => {} /*Nothing is pressed*/
        Some(key) => match key { /*If something is pressed, match again on which key was actually pressed*/
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs), /*When pressing a relevant key, move the entitiy to the relevant position*/
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs), /*Left, right, up, down are by default bound to WASD by the library*/
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {} /*Any other button presses are ignored*/
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {/*Tick is a special function from the rltk library. This function is run once every "tick" or frame*/
        ctx.cls();/*Clear the screen*/

        player_input(self, ctx); /*Call the player input function*/
        self.run_systems();/*Move LeftWalkers to the left on every tick*/

        let positions = self.ecs.read_storage::<Position>(); /*Gain read only access from the ECS to the container used to store position components*/
        let renderables = self.ecs.read_storage::<Renderable>(); /*Same for renderables*/

        for (pos, render) in (&positions, &renderables).join() { /*A for loop that loops over all entities that have the relevant traits, in this case those that have a position and are renderable*/
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph); /*Sets these properties of relevant entities to be rendered on the screen*/
        }
    }
}

/*Rust lifetimes are used but i barely understand them*/
impl<'a> System<'a> for LeftWalker { /*This implements logic for LeftWalker entities, which in this case is to move left at all times*/
    type SystemData = (ReadStorage<'a, LeftMover>, 
                        WriteStorage<'a, Position>);/*Gain access to their properties*/

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join() { /*For any of the entities that has the relevant traits (is a LeftWalker and has a position), do X*/
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; } /*X in this case is move them to the left, wrapping around if they hit the wall*/
        }
    }
}


impl State {
    fn run_systems(&mut self) { /*The function can mutate itself*/
        let mut lw = LeftWalker{}; /*Create instance of the LeftWalker struct*/
        lw.run_now(&self.ecs); /*Makes entities with the LeftWalker component run left with a call to the ECS*/
        self.ecs.maintain(); /*If actions are queued up, execute them*/
    }
}