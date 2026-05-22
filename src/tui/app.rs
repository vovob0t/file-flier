use std::cell::RefCell;
use std::rc::Rc;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;
use file_flier::sort_file_tree;
use file_flier::{config::Config, tools::FileNode};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::border::THICK;
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

#[derive(Debug)]
pub struct App {
    pub file_tree: Rc<RefCell<FileNode>>,
    pub curent_directory: Rc<RefCell<FileNode>>,
    pub nodes_history: Vec<Rc<RefCell<FileNode>>>,
    pub config: Config,
    pub should_exit: bool,
    pub file_tree_initialized: bool,
    pub loading_screen_initialize: bool,
}

impl App {
    pub fn new(cfg: Config) -> Self {
        // let file_tree = FileNode::default();
        let mut new_file_tree = Self {
            file_tree: Rc::new(RefCell::new(FileNode::default())),
            curent_directory: Rc::new(RefCell::new(FileNode::default())),
            nodes_history: vec![],
            config: cfg,
            should_exit: false,
            file_tree_initialized: false,
            loading_screen_initialize: false,
        };
        new_file_tree.curent_directory = Rc::clone(&new_file_tree.file_tree);
        new_file_tree
    }

    pub fn curent_file_node_mut(&mut self) -> &Rc<RefCell<FileNode>> {
        self.nodes_history.iter().last().unwrap()
    }
    pub fn curent_file_node(&self) -> &Rc<RefCell<FileNode>> {
        self.nodes_history.iter().last().unwrap()
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if !self.loading_screen_initialize {
                self.loading_screen_initialize = true;
                continue;
            }

            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.should_exit = true,
            KeyCode::Up | KeyCode::Char('k') => self.move_up(),
            KeyCode::Down | KeyCode::Char('j') => self.move_down(),
            KeyCode::Left | KeyCode::Esc => self.select_none(),
            KeyCode::Enter | KeyCode::Char('l') => self.change_curent_path_into_selected(),
            KeyCode::Backspace | KeyCode::Char('h') => self.change_to_previous_path(),
            KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.change_to_previous_path()
            }
            KeyCode::Char('g') => self.move_to_first(),
            KeyCode::Char('G') => self.move_to_last(),
            _ => {}
        }
    }

    fn change_to_previous_path(&mut self) {
        if self.nodes_history.len() > 1 {
            self.nodes_history.pop();
        }
    }

    fn change_curent_path_into_selected(&mut self) {
        let selected_entry_indx = self
            .curent_file_node_mut()
            .borrow()
            .state
            .borrow()
            .selected();

        if let Some(i) = selected_entry_indx {
            let i = Rc::clone(&self.curent_file_node_mut().borrow().children[i]);

            self.nodes_history.push(i);
            sort_file_tree(
                &mut self.nodes_history.iter().last().unwrap().borrow_mut(),
                &self.config.sort_type,
            );
        }
    }

    fn select_none(&mut self) {
        self.curent_file_node_mut()
            .borrow()
            .state
            .borrow_mut()
            .select(None);
    }

    fn move_up(&mut self) {
        self.curent_file_node_mut()
            .borrow()
            .state
            .borrow_mut()
            .select_previous();
    }
    fn move_down(&mut self) {
        self.curent_file_node_mut()
            .borrow()
            .state
            .borrow_mut()
            .select_next();
    }
    fn move_to_first(&mut self) {
        self.curent_file_node_mut()
            .borrow()
            .state
            .borrow_mut()
            .select_first();
    }
    fn move_to_last(&mut self) {
        self.curent_file_node_mut()
            .borrow()
            .state
            .borrow_mut()
            .select_last();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.file_tree_initialized {
            if !self.loading_screen_initialize {
                App::render_loading_screen(area, buf);
                return;
            }

            self.file_tree = Rc::new(RefCell::new(
                file_flier::create_dir_tree_from_path(self.config.path.as_ref()).unwrap(),
            ));

            sort_file_tree(&mut self.file_tree.borrow_mut(), &self.config.sort_type);
            self.curent_directory = Rc::clone(&self.file_tree);

            self.nodes_history.push(Rc::clone(&self.file_tree));

            self.file_tree_initialized = true;
            Widget::render(Clear, area, buf);

            // return;
        };

        let main_layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);

        let [header_area, content_area, footer_area] = area.layout(&main_layout);

        let content_layout = Layout::vertical([Constraint::Fill(1)]);
        let [list_area] = content_area.layout(&content_layout);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_dir_entries(list_area, buf);
    }
}

/// Rendering logic
impl App {
    fn render_loading_screen(area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Loading screen")
            .bg(NORMAL_ROW_BG)
            .border_set(THICK);

        Paragraph::new("Please, wait while FF evaluates file system")
            .centered()
            .block(block)
            .bold()
            .slow_blink()
            .render(area, buf);
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("File-flier - disk space analyzer")
            .centered()
            .bold()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, space bar to select and unselect, d to delete, →/enter to move into directory, ←/backspace to go back, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_dir_entries(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(
                Line::raw(format!(
                    "Curent directory - {}, size - {}",
                    self.curent_file_node().borrow().name,
                    self.curent_file_node().borrow().size,
                ))
                .left_aligned(),
            )
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let entries: Vec<ListItem> = self
            .curent_file_node()
            .borrow()
            .children
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let color = alternate_colors(i);
                let line = Line::styled(
                    format!(
                        "{} - {}",
                        entry
                            .borrow()
                            .name
                            .replace(&self.curent_file_node().borrow().name, "")
                            .replace("/", "")
                            + match entry.borrow().is_dir {
                                true => "/",
                                false => " ",
                            },
                        entry.borrow().size
                    ),
                    TEXT_FG_COLOR,
                );
                ListItem::new(line).bg(color)
            })
            .collect();

        let list = List::new(entries)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(
            list,
            area,
            buf,
            &mut self.curent_file_node_mut().borrow().state.borrow_mut(),
        );
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i.is_multiple_of(2) {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
