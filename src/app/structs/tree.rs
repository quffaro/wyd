// shamelessly stolen from Ben Lovy
// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
    fn size(&self) -> usize {
        self.arena.len()
    }
    fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, node| acc + node.children.len())
    }

    fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
    fn depth_to_target(&self, idx: usize, target: &T) -> Option<usize> {
        // are we here?  If so, Some(0)
        if target == &self.arena[idx].val {
            return Some(0);
        }

        // If not, try all children
        for p in &self.arena[idx].children {
            if let Some(x) = self.depth_to_target(*p, &target) {
                return Some(1 + x);
            }
        }
        // If it cant be found, return None
        None
    }
    fn distance_between(&mut self, from: T, target: T) -> usize {
        // If it's not in the tree, this will add a new unconnected node
        // the final function will still return None
        let start_node = self.node(from);
        let mut ret = 0;
        // Start traversal
        let mut trav = &self.arena[start_node];
        // Explore all children, then hop up one
        while let Some(inner) = trav.parent {
            if let Some(x) = self.depth_to_target(inner, &target) {
                ret += x;
                break;
            }
            trav = &self.arena[inner];
            ret += 1;
        }
        // don't go all the way to target, just orbit
        if ret > 0 {
            ret - 1
        } else {
            ret
        }
    }
}
