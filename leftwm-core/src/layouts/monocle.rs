use crate::models::Window;
use crate::models::Workspace;

use super::Layout;
use super::LayoutModifiers;
use super::Xyhw;

// use crate::models::WindowState;

/// Layout which gives only one window with the full desktop realestate. A monocle mode.
pub fn update(workspace: &Workspace, windows: &mut Vec<&mut Window>) {
    let window_count = windows.len();

    if window_count == 0 {
        return;
    }

    let workspace_width = workspace.width_limited(1);
    let workspace_x = workspace.x_limited(1);
    let mut iter = windows.iter_mut();

    //maximize primary window
    {
        if let Some(monowin) = iter.next() {
            monowin.set_height(workspace.height());
            monowin.set_width(workspace_width);
            monowin.set_x(workspace_x);
            monowin.set_y(workspace.y());

            monowin.set_visible(true);
        }
    }

    //hide all other windows
    {
        if window_count > 1 {
            for w in iter {
                w.set_height(workspace.height());
                w.set_width(workspace_width);
                w.set_x(workspace_x);
                w.set_y(workspace.y());

                w.set_visible(false);
            }
        }
    }
}

struct Monocle;

impl Layout for Monocle {
    fn calculate(window_count: u8, modifiers: LayoutModifiers) -> Vec<Option<Xyhw>> {
        let mut vec: Vec<Option<Xyhw>> = Vec::new();
        vec.push(Some(modifiers.container));
        for i in 1..window_count {
            vec.push(None);
        }
        vec
    }
}

mod test {
    use crate::{layouts::{Layout, LayoutModifiers}, models::XyhwBuilder};

    #[test]
    fn monocle_should_return_only_one_size() {
        let calculated = super::Monocle::calculate(3, LayoutModifiers {
            container: XyhwBuilder {
                ..Default::default()
            }.into(),
            master_width_percentage: 50,
            master_window_count: 1
        });

        assert_eq!(calculated.len(), 3);
        assert!(calculated.get(0).unwrap().is_some());
        assert!(calculated.get(1).unwrap().is_none());
        assert!(calculated.get(2).unwrap().is_none());
    }
}
