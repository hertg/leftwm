use super::{Manager, Screen, Workspace};
use crate::config::Config;
use crate::display_servers::DisplayServer;
use crate::models::Tag;

impl<C: Config, SERVER: DisplayServer> Manager<C, SERVER> {
    /// Process a collection of events, and apply them changes to a manager.
    ///
    /// Returns `true` if changes need to be rendered.
    pub fn screen_create_handler(&mut self, screen: Screen) -> bool {
        let current_workspace_count = self.state.workspaces.len();

        let mut workspace = Workspace::new(
            screen.wsid,
            screen.bbox,
            self.state.layout_manager.new_layout(),
            screen
                .max_window_width
                .or_else(|| self.state.config.max_window_width()),
        );
        if workspace.id.is_none() {
            workspace.id = Some(
                self.state
                    .workspaces
                    .iter()
                    .map(|ws| ws.id.unwrap_or(-1))
                    .max()
                    .unwrap_or(-1)
                    + 1,
            );
        }
        if workspace.id.unwrap_or(0) as usize >= self.state.tags.len() {
            dbg!("Workspace ID needs to be less than or equal to the number of tags available.");
        }
        workspace.update_for_theme(&self.state.config);
        
        let visible_tags: Vec<&Tag> = self.state.tags.iter().filter(|tag| !tag.hidden).collect();
        let tag_count = visible_tags.len();
        if tag_count <= current_workspace_count {
            // there are no tags left to assign to this new screen/workspace,
            // we need to create another tag here, so every workspace has one
            let id = (current_workspace_count + 1) as u8;
            let label = id.to_string();
            let new_tag = Tag::new(id, &label, self.state.layout_manager.new_layout());
            self.state
                .tags
                .insert(current_workspace_count, new_tag);
        }
        let next_tag = self.state.tags[current_workspace_count].clone();
        self.focus_workspace(&workspace);
        self.focus_tag(&next_tag.label.as_str());
        workspace.show_tag(&next_tag);
        self.state.workspaces.push(workspace.clone());
        self.state.workspaces.sort_by(|a, b| a.id.cmp(&b.id));
        self.state.screens.push(screen);
        self.focus_workspace(&workspace);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_two_screens_should_tag_them_with_first_and_second_tags() {
        let mut manager = Manager::new_test(vec!["1".to_string(), "2".to_string()]);
        manager.screen_create_handler(Screen::default());
        manager.screen_create_handler(Screen::default());
        assert!(manager.state.workspaces[0].has_tag("1"));
        assert!(manager.state.workspaces[1].has_tag("2"));
    }

    #[test]
    fn should_be_able_to_add_screens_with_preexisting_tags() {
        let mut manager = Manager::new_test(vec![
            "web".to_string(),
            "console".to_string(),
            "code".to_string(),
        ]);
        manager.screen_create_handler(Screen::default());
        manager.screen_create_handler(Screen::default());
        assert!(manager.state.workspaces[0].has_tag("web"));
        assert!(manager.state.workspaces[1].has_tag("console"));
    }
}
