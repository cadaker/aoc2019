use std::hash::Hash;

#[derive(Clone)]
pub struct Prio<T: Ord + Eq + Hash + Clone, PrioT: Ord + Eq + Clone> {
    by_prio: std::collections::BTreeSet<(PrioT, T)>,
    prio_of: std::collections::HashMap<T, PrioT>,
}

impl<T: Ord + Eq + Clone + Hash, PrioT: Ord + Eq + Clone> Prio<T, PrioT> {
    pub fn new() -> Self {
        use std::collections::{BTreeSet, HashMap};
        Prio { by_prio: BTreeSet::new(), prio_of: HashMap::new() }
    }

    pub fn update(&mut self, item: T, prio: PrioT) {
        match self.prio_of.get(&item).cloned() {
            None => (),
            Some(old_prio) => {
                self.by_prio.remove(&(old_prio, item.clone()));
            },
        };
        self.prio_of.insert(item.clone(), prio.clone());
        self.by_prio.insert((prio, item));
    }

    pub fn is_empty(&self) -> bool {
        self.by_prio.is_empty()
    }

    pub fn prio_for(&self, item: &T) -> Option<PrioT> {
        self.prio_of.get(item).cloned()
    }


    pub fn pop(&mut self) -> Option<(T, PrioT)> {
        let popped = self.by_prio.iter().next().cloned();
        match popped {
            None => None,
            Some(entry) => {
                self.by_prio.remove(&entry);
                self.prio_of.remove(&entry.1);
                Some((entry.1, entry.0))
            },
        }
    }
}


pub trait Dijkstra {
    type Node;

    fn reachable(&mut self, node: &Self::Node) -> Vec<(Self::Node, usize)>;

    fn target(&mut self, node: &Self::Node) -> bool;
}

pub fn dijkstra<Handler: Dijkstra>(handler: &mut Handler, start_node: Handler::Node)
                                   -> Option<(Handler::Node, usize)>
    where Handler::Node: Ord + Eq + std::hash::Hash + Clone
{
    let mut prio = Prio::<Handler::Node, usize>::new();
    let mut finished = std::collections::HashSet::<Handler::Node>::new();

    prio.update(start_node, 0);

    while !prio.is_empty() {
        let (node, dist) = prio.pop().unwrap();
        finished.insert(node.clone());

        if handler.target(&node) {
            return Some((node, dist))
        }

        for (n, extra_dist) in handler.reachable(&node) {
            let existing_prio = prio.prio_for(&n);
            if !finished.contains(&n) &&
                (existing_prio.is_none() || dist + extra_dist < existing_prio.unwrap())
            {
                prio.update(n, dist + extra_dist);
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prio_ordered() {
        let mut prio = Prio::<i32, usize>::new();
        prio.update(-5, 10);
        prio.update(2, 5);
        prio.update(10, 7);
        prio.update(-6, 4);
        assert_eq!(prio.pop(), Some((-6, 4)));
        assert_eq!(prio.pop(), Some((2, 5)));
        assert_eq!(prio.pop(), Some((10, 7)));
        assert_eq!(prio.pop(), Some((-5, 10)));
        assert_eq!(prio.pop(), None);
    }

    #[test]
    fn update_prios() {
        let mut prio = Prio::<i32, usize>::new();
        prio.update(1, 0);
        prio.update(2, 1);
        let mut prio2 = prio.clone();

        assert_eq!(prio.pop(), Some((1, 0)));
        assert_eq!(prio.pop(), Some((2, 1)));

        prio2.update(1, 2);
        assert_eq!(prio2.pop(), Some((2, 1)));
        assert_eq!(prio2.pop(), Some((1, 2)));
    }
}
