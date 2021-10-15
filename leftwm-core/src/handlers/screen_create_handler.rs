use super::{Screen, Workspace};
use crate::config::Config;
use crate::models::Tag;
use crate::state::State;

impl<C: Config> State<C> {
    /// Process a collection of events, and apply them changes to a manager.
    ///
    /// Returns `true` if changes need to be rendered.
    pub fn screen_create_handler(&mut self, screen: Screen) -> bool {
        let current_workspace_count = self.workspaces.len();

        let mut workspace = Workspace::new(
            screen.wsid,
            screen.bbox,
            self.layout_manager.new_layout(),
            screen
                .max_window_width
                .or_else(|| self.config.max_window_width()),
        );
        if workspace.id.is_none() {
            workspace.id = Some(
                self.workspaces
                    .iter()
                    .map(|ws| ws.id.unwrap_or(-1))
                    .max()
                    .unwrap_or(-1)
                    + 1,
            );
        }
        if workspace.id.unwrap_or(0) as usize >= self.tags.len() {
            dbg!("Workspace ID needs to be less than or equal to the number of tags available.");
        }
        workspace.update_for_theme(&self.config);
        
        let visible_tags: Vec<&Tag> = self.tags.iter().filter(|tag| !tag.hidden).collect();
        let tag_count = visible_tags.len();
        if tag_count <= current_workspace_count {
            // there are no tags left to assign to this new screen/workspace,
            // we need to create another tag here, so every workspace has one
            let id = (current_workspace_count + 1) as u8;
            let label = id.to_string();
            let new_tag = Tag::new(id, &label, self.layout_manager.new_layout());
            self.tags.insert(current_workspace_count, new_tag);
        }
        let next_tag = self.tags[current_workspace_count].clone();
        self.focus_workspace(&workspace);
        self.focus_tag(&next_tag.label.as_str());
        workspace.show_tag(&next_tag);
        self.workspaces.push(workspace.clone());
        self.workspaces.sort_by(|a, b| a.id.cmp(&b.id));
        self.screens.push(screen);
        self.focus_workspace(&workspace);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Manager;

    #[test]
    fn creating_two_screens_should_tag_them_with_first_and_second_tags() {
        let mut manager = Manager::new_test(vec!["1".to_string(), "2".to_string()]);
        let state = &mut manager.state;
        state.screen_create_handler(Screen::default());
        state.screen_create_handler(Screen::default());
        assert!(state.workspaces[0].has_tag("1"));
        assert!(state.workspaces[1].has_tag("2"));
    }

    #[test]
    fn should_be_able_to_add_screens_with_preexisting_tags() {
        let mut manager = Manager::new_test(vec![
            "web".to_string(),
            "console".to_string(),
            "code".to_string(),
        ]);
        let state = &mut manager.state;
        state.screen_create_handler(Screen::default());
        state.screen_create_handler(Screen::default());
        assert!(state.workspaces[0].has_tag("web"));
        assert!(state.workspaces[1].has_tag("console"));
    }
}
