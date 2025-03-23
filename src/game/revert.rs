use bevy::prelude::*;

type Move = UVec2;

#[derive(Event)]
pub struct Undo;

#[derive(Event)]
pub struct ReverteAction {
    pub action: Action,
}

#[derive(Event)]
pub struct AddMoveAndClose(pub Move);

#[derive(Event)]
pub struct AddMove(pub Move);

#[derive(Event)]
pub struct MoveDone;

#[derive(Resource, Default, Debug, Clone)]
pub struct Action {
    pub moves: Vec<Move>,
}

#[derive(Resource, Default)]
struct History(Vec<Action>);

impl History {
    fn pop(&mut self) -> Action {
        self.0.pop().unwrap_or_default()
    }

    fn undo(_trigger: Trigger<Undo>, mut commands: Commands, mut history: ResMut<Self>) {
        let m = history.pop();
        info!("Revert {:?}", m);
        commands.trigger(ReverteAction { action: m });
    }

    fn add_move_and_close(
        trigger: Trigger<AddMoveAndClose>,
        mut action: ResMut<Action>,
        mut history: ResMut<Self>,
    ) {
        let m = trigger.0;
        info!("Add move: {:?}", m);
        action.moves.push(m);
        history.0.push(action.clone());
        action.moves.clear();
    }

    fn add_move(trigger: Trigger<AddMove>, mut action: ResMut<Action>) {
        let m = trigger.0;
        info!("Add move: {:?}", m);
        action.moves.push(m);
    }

    fn close_action(
        _trigger: Trigger<MoveDone>,
        mut action: ResMut<Action>,
        mut history: ResMut<Self>,
    ) {
        info!("Closed Action");
        history.0.push(action.clone());
        action.moves.clear();
    }
}

pub struct UndoPlugin;

impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<History>();
        app.init_resource::<Action>();
        app.add_observer(History::add_move);
        app.add_observer(History::close_action);
        app.add_observer(History::undo);
        app.add_observer(History::add_move_and_close);
    }
}
