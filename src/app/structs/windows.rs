// https://applied-math-coding.medium.com/
// use std::cell::RefCell;
// use std::rc::Rc;
use super::{ListItems, ListNav, ListState};
use ratatui::style::Color;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone)]
pub struct Window {
    pub base: BaseWindow,
    pub popup: Popup,
    pub status: WindowStatus,
    pub mode: Mode,
}

impl Window {
    pub fn new(needs_config: bool) -> Window {
        Window {
            base: BaseWindow::Project,
            popup: if needs_config {
                Popup::Config
            } else {
                Popup::None
            },
            status: WindowStatus::NotLoaded,
            mode: Mode::Insert,
        }
    }
    pub fn mode_color(&self) -> Color {
        match self.mode {
            Mode::Insert => Color::Yellow,
            Mode::Normal => Color::Green,
        }
    }
    pub fn base_focus_color(&self, window: BaseWindow) -> Color {
        match self {
            Window {
                popup: Popup::None,
                base: _,
                ..
            } => Color::Yellow,
            _ => Color::White,
        }
    }
    fn to_project() -> Window {
        Window {
            base: BaseWindow::Project,
            popup: Popup::None,
            status: WindowStatus::NotLoaded,
            mode: Mode::Insert,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, EnumString, EnumIter)]
pub enum BaseWindow {
    Project,
    Todo,
    Description,
}

impl fmt::Display for BaseWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Project => write!(f, "PROJECTS"),
            Self::Todo => write!(f, "TODO"),
            Self::Description => write!(f, "DESC"),
        }
    }
}

impl ListItems<BaseWindow> {
    fn new() -> ListItems<BaseWindow> {
        ListItems {
            items: BaseWindow::iter().collect(),
            state: ListState::default(),
        }
    }
    pub fn current(&mut self) -> Option<&BaseWindow> {
        let idx = self.get_state_selected().unwrap();
        self.items.iter().nth(idx)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Popup {
    None,
    SearchGitConfigs,
    AddTodo,
    EditCat,
    NewCat,
    EditDesc,
    Help,
    Config,
}

impl fmt::Display for Popup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::None => write!(f, "NO POPUP"),
            Self::SearchGitConfigs => write!(f, "SEARCH PROJECTS"),
            Self::AddTodo => write!(f, "ADD TODO"),
            Self::EditCat => write!(f, "EDIT CATEGORY"),
            Self::NewCat => write!(f, "NEW CATEGORY"),
            Self::EditDesc => write!(f, "EDIT DESCRIPTION"),
            Self::Help => write!(f, "HELP"),
            Self::Config => write!(f, "CONFIG"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowStatus {
    Loaded,
    NotLoaded,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
        }
    }
}
// #[derive(PartialEq)]
// struct TreeNode {
//     pub value: Option<Window>,
//     pub children: Vec<Rc<RefCell<TreeNode>>>,
//     pub parent: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     pub fn new() -> TreeNode {
//         return TreeNode {
//             value: None,
//             children: vec![],
//             parent: None,
//         };
//     }

//     pub fn init() -> Vec<TreeNode> {
//         return TreeNode {
//             value: Some(Window::Project)

//     pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
//         self.children.push(new_node);
//     }

//     pub fn print(&self) -> String {
//         if let Some(value) = self.value {
//             return value.to_string();
//         } else {
//             return String::from("[")
//                 + &self
//                     .children
//                     .iter()
//                     .map(|tn| tn.borrow().print())
//                     .collect::<Vec<String>>()
//                     .join(",")
//                 + "]";
//         }
//     }
// }

// fn init_tree(s: String) -> Rc<RefCell<TreeNode>> {
//     let root = Rc::new(RefCell::new(TreeNode::new()));
//     let mut current = Rc::clone(&root);
//     let chars = s.chars().collect::<Vec<char>>();
//     for (_, c) in chars
//         .iter()
//         .enumerate()
//         .filter(|(idx, _)| *idx > 0 && *idx + 1 < chars.len())
//     {
//         if *c == '[' || c.is_numeric() {
//             let child = Rc::new(RefCell::new(TreeNode::new()));
//             current.borrow_mut().children.push(Rc::clone(&child));
//             {
//                 let mut mut_child = child.borrow_mut();
//                 mut_child.parent = Some(Rc::clone(&current));
//                 if c.is_numeric() {
//                     mut_child.value = c.to_digit(10);
//                 }
//             }
//             current = child;
//         } else if *c == ',' || *c == ']' {
//             let current_clone = Rc::clone(&current);
//             current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
//         } else {
//             panic!("Unknown character: {}", c);
//         }
//     }
//     return root;
// }