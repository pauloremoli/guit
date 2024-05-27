use git2::{BranchType, Repository};
use ratatui::widgets::ListState;
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub enum Section {
    STATUS,
    COMMITS,
    BRANCHES(BranchType),
    REFLOG,
}

pub struct App<'a> {
    pub repo: &'a Repository,
    pub should_quit: bool,
    pub branches: StatefulList<String>,
    pub status: StatefulList<String>,
    pub commits: StatefulList<git2::Commit<'a>>,
    pub reflog: StatefulList<(String, String, String)>,
    pub active_section: Section,
}

impl<'a> App<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        let commits = crate::repository::commits(&repo, None);
        let branches = crate::repository::branches(&repo, BranchType::Local);
        let reflog = crate::repository::reflog(&repo, None);
        App {
            repo,
            should_quit: false,
            branches: StatefulList::with_items(branches),
            status: StatefulList::with_items(vec![
                "Item1".to_owned(),
                "Item2".to_owned(),
                "Item3".to_owned(),
                "Item4".to_owned(),
                "Item5".to_owned(),
            ]),
            commits: StatefulList::with_items(commits.expect("Could not load commits")),
            active_section: Section::STATUS,
            reflog: StatefulList::with_items(reflog),
        }
    }

    pub fn on_up(&mut self) {
        match &self.active_section {
            Section::STATUS => self.status.previous(),
            Section::COMMITS => self.commits.previous(),
            Section::BRANCHES(_) => self.branches.previous(),
            Section::REFLOG => self.reflog.previous(),
        }
    }

    pub fn on_down(&mut self) {
        match &self.active_section {
            Section::STATUS => self.status.next(),
            Section::COMMITS => self.commits.next(),
            Section::BRANCHES(_) => self.branches.next(),
            Section::REFLOG => self.reflog.next(),
        }
    }

    pub fn on_right(&mut self) {
        match &self.active_section {
            Section::BRANCHES(_) => todo!("Change to remote branch is not yet implemented"),
            _ => (),
        }
    }

    pub fn on_left(&mut self) {}

    pub fn on_tab(&mut self) {
        self.active_section = match self.active_section {
            Section::STATUS => Section::COMMITS,
            Section::COMMITS => Section::BRANCHES(BranchType::Local),
            Section::BRANCHES(_) => Section::REFLOG,
            Section::REFLOG => Section::STATUS,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Add here any update that needs to show progress or redraw based on tick
    }
}
