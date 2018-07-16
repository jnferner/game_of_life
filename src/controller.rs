use crate::generation_calculator::Change;
use crate::grid::Position;
use crate::grid_info::GridInfo;
use crate::interactive_game::InteractiveGame;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

pub struct ClickableController {
    game: Box<InteractiveGame>,
    grid_info: GridInfo,
}

impl ClickableController {
    pub fn new(game: Box<InteractiveGame>, grid_info: GridInfo) -> Self {
        ClickableController { game, grid_info }
    }

    pub fn on_click(&mut self, x: u32, y: u32) {
        let cell_position = self.get_cell_location_from_coordinates(x, y);
        if let Some((x, y)) = cell_position {
            let position = Position { x, y };
            self.game.toggle_cell(&position);
        }
    }

    pub fn on_timer(&mut self) {
        self.game.next_generation();
    }

    fn get_cell_location_from_coordinates(&self, x: u32, y: u32) -> Option<(u32, u32)> {
        if x > self.grid_info.width || y > self.grid_info.height {
            None
        } else {
            let cell_width = self.grid_info.width / self.grid_info.columns;
            let cell_height = self.grid_info.height / self.grid_info.rows;
            let cell_x = x / cell_width;
            let cell_y = y / cell_height;
            Some((cell_x, cell_y))
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::generation_calculator::Change;
    use crate::grid::Position;
    use crate::grid_info::GridInfo;
    use crate::interactive_game::InteractiveGameMock;

    use mockers::matchers::ANY;
    use mockers::{Scenario, Sequence};

    fn create_mock() -> (Scenario, InteractiveGameMock, GridInfo) {
        let scenario = Scenario::new();
        let game = scenario.create_mock_for::<InteractiveGame>();
        let grid_info = GridInfo {
            width: 10,
            height: 8,
            rows: 4,
            columns: 5,
        };
        (scenario, game, grid_info)
    }

    #[test]
    fn calls_next_gen_on_timer() {
        let (scenario, game, grid_info) = create_mock();
        scenario.expect(game.next_generation_call().and_return(()));
        let mut controller = ClickableController::new(Box::new(game), grid_info);
        controller.on_timer();
    }

    #[test]
    fn ignores_out_of_bounds_clicks() {
        let (_scenario, game, grid_info) = create_mock();
        let mut controller = ClickableController::new(Box::new(game), grid_info);
        controller.on_click(11, 9);
        controller.on_click(10, 9);
        controller.on_click(11, 8);
    }

    #[test]
    fn toggles_cell_on_click() {
        let (scenario, game, grid_info) = create_mock();
        const POSITION: Position = Position { x: 1, y: 2 };
        scenario.expect(game.toggle_cell_call(&POSITION).and_return(()));
        let mut controller = ClickableController::new(Box::new(game), grid_info);
        controller.on_click(2, 5);
    }
}
