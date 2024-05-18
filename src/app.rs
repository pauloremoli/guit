use std::{collections::HashMap, path::PathBuf};

use anyhow::{Error, Result};
use git2::{BranchType, Commit, Repository};
use ratatui::widgets::ListState;

pub fn commits(repo: &Repository, limit: Option<usize>) -> Vec<(String, String, String)> {
    let limit = limit.unwrap_or(20);
    let mut result = Vec::new();

    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return result,
    };

    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(_) => return result,
    };

    revwalk.push(head.target().unwrap()).unwrap();
    revwalk.set_sorting(git2::Sort::TIME).unwrap();

    for oid in revwalk.take(limit) {
        if let Ok(oid) = oid {
            if let Ok(commit) = repo.find_commit(oid) {
                let commit_hash = commit.id().to_string()[..7].to_owned();
                let author = commit.author().name().unwrap_or("").to_string();
                let message = commit.summary().unwrap_or("").to_string();

                result.push((commit_hash, author, message));
            }
        }
    }

    result
}

pub fn reflog(repo: &Repository, limit: Option<usize>) -> Vec<(String, String, String)> {
    let limit = limit.unwrap_or(100);
    let mut result = Vec::new();

    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(_) => return result,
    };

    revwalk.set_sorting(git2::Sort::TIME).unwrap();

    for oid in revwalk.take(limit) {
        if let Ok(oid) = oid {
            if let Ok(commit) = repo.find_commit(oid) {
                let commit_hash = commit.id().to_string()[..7].to_owned();
                let author = commit.author().name().unwrap_or("").to_string();
                let message = commit.summary().unwrap_or("").to_string();

                result.push((commit_hash, author, message));
            }
        }
    }

    result
}

pub fn branches(repo: &Repository, branch_type: BranchType) -> Vec<String> {
    let branches = repo
        .branches(Some(branch_type))
        .expect("Could not list branches from repo");
    branches
        .filter_map(|branch| match branch {
            Ok((branch, _)) => match branch.name() {
                Ok(Some(value)) => Some(value.to_owned()),
                _ => None,
            },

            _ => None,
        })
        .collect::<Vec<String>>()
}

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

pub struct App {
    pub repo: Repository,
    pub should_quit: bool,
    pub branches: StatefulList<String>,
    pub status: StatefulList<String>,
    pub commits: StatefulList<(String, String, String)>,
    pub reflog: StatefulList<(String, String, String)>,
    pub active_section: Section,
}

impl App {
    pub fn new(repo_path: &PathBuf) -> Self {
        let repo = match Repository::init(repo_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };

        let commits = commits(&repo, None);
        let branches = branches(&repo, BranchType::Local);
        let reflog = reflog(&repo, None);
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
            commits: StatefulList::with_items(commits),
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
