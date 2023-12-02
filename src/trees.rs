use crate::utils::lines;
use std::cell::{Ref, RefCell};
use std::cmp;
use std::rc::Rc;

struct Tree {
    size: usize,
    max_left: Option<usize>,
    max_up: Option<usize>,
    max_right: Option<usize>,
    max_down: Option<usize>,
}

impl Tree {
    fn is_visible(&self) -> bool {
        for max_boundary in vec![self.max_left, self.max_up, self.max_right, self.max_down] {
            let is_visible = match max_boundary {
                Some(max_size) => self.size > max_size,
                None => true,
            };
            if is_visible {
                return true;
            }
        }
        false
    }

    fn from(
        size: usize,
        opt_left: Option<Rc<RefCell<Tree>>>,
        opt_up: Option<Rc<RefCell<Tree>>>,
    ) -> Tree {
        let max_left = Tree::get_max(opt_left, |left| left.max_left);
        let max_up = Tree::get_max(opt_up, |up| up.max_up);
        Tree {
            size: size,
            max_left: max_left,
            max_up: max_up,
            max_right: None,
            max_down: None,
        }
    }

    fn set_right_and_down(
        &mut self,
        opt_right: Option<Rc<RefCell<Tree>>>,
        opt_down: Option<Rc<RefCell<Tree>>>,
    ) {
        self.max_right = Tree::get_max(opt_right, |right| right.max_right);
        self.max_down = Tree::get_max(opt_down, |down| down.max_down);
    }

    fn get_max<F>(
        opt_delimiting_tree: Option<Rc<RefCell<Tree>>>,
        get_max_boundary: F,
    ) -> Option<usize>
    where
        F: Fn(Ref<Tree>) -> Option<usize>,
    {
        if let Some(ref_delimiting_tree) = opt_delimiting_tree {
            if let Ok(delimiting_tree) = ref_delimiting_tree.try_borrow() {
                let delimiting_tree_size = delimiting_tree.size;
                if let Some(prev_delimiting_tree) = get_max_boundary(delimiting_tree) {
                    return Some(cmp::max(prev_delimiting_tree, delimiting_tree_size));
                } else {
                    return Some(delimiting_tree_size);
                }
            }
        }
        None
    }
}

pub fn nb_visible_trees(tree_house_filename: &str) -> usize {
    let mut tree_house: Vec<Vec<Rc<RefCell<Tree>>>> = vec![];
    if let Ok(lines) = lines(tree_house_filename) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let mut current_line: Vec<Rc<RefCell<Tree>>> = vec![];
                let mut col_idx = 0;
                // When parsing we fill the size, plus the left and up max sizes as they can be deduced as the tree house is scanned
                // from the upper left. Max left is just the max between the previous tree max left and its the left tree size
                // if it exists and the same reasoning can be applied to the max up size
                for tree_size in line
                    .chars()
                    .map(|c| c.to_string())
                    .map(|c| usize::from_str_radix(&c, 10).unwrap())
                {
                    let opt_left = if col_idx > 0 {
                        Some(current_line[col_idx - 1].to_owned())
                    } else {
                        None
                    };
                    let opt_up = if tree_house.len() > 0 {
                        Some(tree_house[tree_house.len() - 1][col_idx].to_owned())
                    } else {
                        None
                    };
                    current_line.push(Rc::new(RefCell::new(Tree::from(
                        tree_size, opt_left, opt_up,
                    ))));
                    col_idx += 1;
                }
                tree_house.push(current_line);
            }
        }
    }

    let max_idx = tree_house.len() - 1;
    let mut visible_trees_count = 0;
    // As all the trees are known now we can do the reverse exploration to fill max right and max down
    // Like we did for left and up as they can be deduced when scanning from bottom right
    for line_idx in (0..=max_idx).rev() {
        let current_line = &tree_house[line_idx];
        for col_idx in (0..=max_idx).rev() {
            if let Ok(mut tree) = current_line[col_idx].try_borrow_mut() {
                let opt_right = if col_idx < max_idx {
                    Some(current_line[col_idx + 1].to_owned())
                } else {
                    None
                };
                let opt_down = if line_idx < max_idx {
                    Some(tree_house[line_idx + 1][col_idx].to_owned())
                } else {
                    None
                };
                tree.set_right_and_down(opt_right, opt_down);
                if tree.is_visible() {
                    visible_trees_count += 1;
                }
            }
        }
    }

    visible_trees_count
}
