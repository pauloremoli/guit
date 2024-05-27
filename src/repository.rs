use anyhow::Result;
use git2::{BranchType, Commit, Repository, Statuses};

pub fn status<'a>(repo: &'a Repository) -> Result<Statuses<'_>> {
    Ok(repo.statuses(None)?)
}

pub fn commits<'a>(repo: &'a Repository, limit: Option<usize>) -> Result<Vec<Commit<'_>>> {
    let limit = limit.unwrap_or(20);

    let head = repo.head()?;
    let mut revwalk = repo.revwalk()?;

    revwalk.push(head.target().unwrap()).unwrap();
    revwalk.set_sorting(git2::Sort::TIME).unwrap();

    revwalk
        .take(limit)
        .into_iter()
        .map(|oid| -> Result<Commit<'_>> { repo.find_commit(oid?).map_err(|err| err.into()) })
        .collect()
}

pub fn reflog(repo: &Repository, limit: Option<usize>) -> Vec<(String, String, String)> {
    let limit = limit.unwrap_or(100);
    let mut result = Vec::new();

    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(_) => return result,
    };

    let head = match repo.head() {
        Ok(head) => head,
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
