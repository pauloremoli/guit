use ratatui::{prelude::*, widgets::*};

use crate::app::{App, Section};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Min(8), Constraint::Length(5)]).split(f.size());
    draw_main_panes(f, app, chunks[0]);
    draw_hot_keys(f, chunks[1]);
}

fn draw_right_panes(f: &mut Frame, app: &mut App, area: Rect) {
    let commit_message_style = Style::default().fg(Color::White);
    let author_style = Style::default().fg(Color::LightBlue);
    let hash_style = Style::default().fg(Color::Yellow);

    let logs: Vec<ListItem> = app
        .reflog
        .items
        .iter()
        .map(|commit| {
            let content = vec![text::Line::from(vec![
                Span::styled(&commit.0, hash_style),
                Span::raw(" "),
                Span::styled(&commit.1, author_style),
                Span::raw(" "),
                Span::styled(&commit.2, commit_message_style),
            ])];
            ListItem::new(content)
        })
        .collect();
    let logs = List::new(logs)
        .block(
            Block::bordered()
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                })
                .title("Reflog")
                .border_style(match &app.active_section {
                    Section::COMMITS => Style::new().light_blue(),
                    _ => Style::new(),
                }),
        )
        .highlight_style(Style::new().bg(Color::Green).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(logs, area, &mut app.reflog.state);
}

fn draw_left_panes(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(40),
        Constraint::Percentage(20),
    ])
    .split(area);

    draw_status(app, f, &chunks[0]);
    draw_commits(app, f, &chunks[1]);
    draw_branches(app, f, &chunks[2]);
}

fn get_initials(name: &str) -> String {
    name.split(" ")
        .into_iter()
        .map(|item| item[..1].to_uppercase().to_owned())
        .collect()
}

fn draw_commits(app: &mut App, f: &mut Frame, area: &Rect) {
    let commit_message_style = Style::default().fg(Color::White);
    let author_style = Style::default().fg(Color::LightBlue);
    let hash_style = Style::default().fg(Color::Yellow);

    let logs: Vec<ListItem> = app
        .commits
        .items
        .iter()
        .map(|commit| {
            let content = vec![text::Line::from(vec![
                Span::styled(commit.id().to_string()[..7].to_owned(), hash_style),
                Span::raw(" "),
                Span::styled(
                    get_initials(commit.author().name().unwrap_or("")),
                    author_style,
                ),
                Span::raw(" "),
                Span::styled(
                    commit.summary().unwrap_or("").to_string(),
                    commit_message_style,
                ),
            ])];
            ListItem::new(content)
        })
        .collect();
    let logs = List::new(logs)
        .block(
            Block::bordered()
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                })
                .title("Commits")
                .border_style(match &app.active_section {
                    Section::COMMITS => Style::new().light_blue(),
                    _ => Style::new(),
                }),
        )
        .highlight_style(Style::new().bg(Color::Green).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(logs, *area, &mut app.commits.state);
}

fn draw_branches(app: &mut App, f: &mut Frame, area: &Rect) {
    let branches: Vec<ListItem> = app
        .branches
        .items
        .iter()
        .map(|branch| ListItem::new(text::Line::from(vec![Span::raw(branch)])))
        .collect();
    let logs = List::new(branches)
        .block(
            Block::bordered()
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                })
                .title("Branches")
                .border_style(match &app.active_section {
                    Section::BRANCHES(_) => Style::new().light_blue(),
                    _ => Style::new(),
                }),
        )
        .highlight_style(Style::default().light_green().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(logs, *area, &mut app.branches.state);
}

fn draw_status(app: &mut App, f: &mut Frame, area: &Rect) {
    let status: Vec<ListItem> = app
        .status
        .items
        .iter()
        .map(|status| ListItem::new(vec![text::Line::from(Span::raw(status))]))
        .collect();
    let items_list = List::new(status)
        .block(
            Block::bordered()
                .title("Status")
                .border_style(match &app.active_section {
                    Section::STATUS => Style::new().light_blue(),
                    _ => Style::new(),
                })
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        )
        .highlight_style(Style::default().light_green().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(items_list, *area, &mut app.status.state);
}

fn draw_main_panes(f: &mut Frame, app: &mut App, area: Rect) {
    let constraints = vec![Constraint::Percentage(30), Constraint::Percentage(70)];

    let chunks = Layout::horizontal(constraints).split(area);
    {
        draw_left_panes(f, app, chunks[0]);
        draw_right_panes(f, app, chunks[1]);
    }
}

fn draw_hot_keys(f: &mut Frame, area: Rect) {
    let padding = Span::raw("    ");
    let text = vec![text::Line::from(vec![
        Span::styled("Q", Style::default().fg(Color::Green)),
        Span::raw(" - quit"),
        padding.clone(),
        Span::styled("?", Style::default().fg(Color::Green)),
        Span::raw(" - list all hot-keys"),
        padding.clone(),
        Span::styled("Up", Style::default().fg(Color::Green)),
        Span::raw(" - move up in the active pane"),
        padding.clone(),
        Span::styled("Down", Style::default().fg(Color::Green)),
        Span::raw(" - move down in the active pane"),
        padding.clone(),
        Span::styled("TAB", Style::default().fg(Color::Green)),
        Span::raw(" - to change active pane"),
    ])];
    let block = Block::bordered()
        .padding(Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 0,
        })
        .title(Span::styled(
            " Hot-Keys ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
