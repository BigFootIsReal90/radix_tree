pub mod radix_tree {

    use std::{cmp, collections::HashMap, fmt::Debug};

    #[derive(Debug)]
    pub enum StreamRes<T> {
        One(T),
        Two((T, T)),
    }

    // trait Insertion<T> {
    //     fn insertion(&mut self, node: T);
    // }
    // impl Insertion<(String, String)> for HashMap<String, String> {
    //     fn insertion(&mut self, node: (String, String)) {
    //         self.insert(node.0, node.1);
    //     }
    // }
    // impl Insertion<String> for String {
    //     fn insertion(&mut self, node: String) {
    //         self.insert_str(self.len() - 1, &node);
    //     }
    // }

    #[derive(Clone, Debug)]
    pub struct RadixTreeNode<T> {
        pub children: HashMap<String, RadixTreeNode<T>>,
        pub value: Option<T>,
        pub end: bool,
    }
    #[derive(Debug, Clone)]
    pub struct RadixTree<T> {
        pub root: RadixTreeNode<T>,
    }

    impl RadixTree<HashMap<String, String>> {
        pub fn new(root: RadixTreeNode<HashMap<String, String>>) -> Self {
            RadixTree { root: root }
        }

        //Insert node in the radix tree
        pub fn insert(&mut self, mut new_string: String, val: HashMap<String, String>) {
            println!("inside insert function");
            //let prev = node;
            let mut node = &mut self.root;
            let mut key = "".to_string();
            while new_string.len() > 0 {
                println!("{:?} inside while", new_string);
                let mut found = false;
                //println!("{:?}", node.children.iter());
                for (egde_label, _) in node.children.iter() {
                    println!("inside for");
                    let common = Self::common_prefix(&new_string, egde_label);
                    if common.len() > 0 {
                        found = true;
                        key = egde_label.clone();
                        break;
                    }
                }

                if found {
                    let common = Self::common_prefix(&key, &new_string);
                    println!("{:?} common", common);
                    let mut remaining: String = "".to_string();
                    for (i, c) in new_string.char_indices() {
                        if i >= common.chars().count() {
                            remaining.push_str(&c.to_string());
                        }
                    }
                    println!("{:?} remaining", remaining);

                    if common == key {
                        new_string = remaining;
                        let curr_node = node.children.get_mut(&key).unwrap();
                        node = curr_node;
                    } else {
                        // split
                        Self::split_edge(&mut node, &common, remaining, &key, val);
                        //new_string = "".to_string();
                        break;
                    }

                    // for (edge_label, child) in node.children.iter() {
                } else {
                    println!("{:?} inside else", new_string);
                    node.children.insert(
                        new_string.clone(),
                        RadixTreeNode {
                            children: HashMap::new(),
                            value: Some(val),
                            end: true,
                        },
                    );
                    //new_string = "".to_string();
                    break;
                }
            }
            println!("successfully inserted");
        }
        // if found {
        //     Self::split_edge(node, &common, remaining, edge_label, val.clone());
        //     new_string = "".to_string();
        //     break;
        // }
        // if !found {
        //     let mut new_node1 = RadixTreeNode {
        //         children: HashMap::new(),
        //         value: Some(val),
        //         end: false,
        //     };
        //     self.root.children.insert(new_string, new_node1);
        //     new_string = "".to_string();
        //     break;
        // }

        //Find out the common prefix bewteen two string
        fn common_prefix(s: &String, d: &String) -> String {
            println!("inside common prefix");
            let mut s_iter = s.chars();
            let mut d_iter = d.chars();
            let mut common = String::new();

            loop {
                let temp_s_iter_next = s_iter.next();
                let s_iter_next;
                match temp_s_iter_next {
                    Some(val) => s_iter_next = val,
                    None => break,
                }
                let temp_d_iter_next = d_iter.next();
                let d_iter_next;
                match temp_d_iter_next {
                    Some(val) => d_iter_next = val,
                    None => break,
                }
                if s_iter_next == d_iter_next {
                    common.push(s_iter_next);
                } else {
                    break;
                }
            }
            return common;
        }

        //Split a RadixTree node
        fn split_edge(
            node: &mut RadixTreeNode<HashMap<String, String>>,
            common: &String,
            remaining: String,
            edge_label: &String,
            val: HashMap<String, String>,
        ) {
            //println!("{:?},{:?} inside split egde", edge_label, node);
            let temp = node.children.remove(edge_label).unwrap();

            let mut common2 = "".to_string();
            for (i, c) in edge_label.char_indices() {
                if i >= common.chars().count() {
                    common2.push_str(&c.to_string());
                }
            }

            let mut new_node1 = RadixTreeNode {
                children: HashMap::new(),
                value: None,
                end: false,
            };
            let new_node2 = RadixTreeNode {
                children: HashMap::new(),
                value: Some(val),
                end: true,
            };

            new_node1.children.insert(remaining.to_owned(), new_node2);

            // for (key, value) in temp.children.drain() {
            //     new_node1.children.insert(key, value);
            // }
            node.children.insert(common.to_owned(), new_node1);
            node.children
                .get_mut(common)
                .unwrap()
                .children
                .insert(common2, temp);
            //node.children.insert(common2, temp);
        }

        pub fn search_in_range<'a>(
            curr: &'a RadixTreeNode<HashMap<String, String>>,
            start: &str,
            end: &str,
            curr_str: String,
            res: &'a mut Vec<StreamRes<String>>,
            start_len: usize,
            end_len: usize,
        ) {
            println!(
                "inside search_in_range start:{:?} end:{:?} curr_str:{:?}",
                start, end, curr_str
            );
            if curr.end {
                println!(
                    "inside end condition,s len {:?}, e len {:?}, curr string len {:?}",
                    start_len,
                    end_len,
                    curr_str.len()
                );
                if curr_str.len() >= start_len && curr_str.len() <= end_len {
                    println!("inside end and len condition");
                    res.push(StreamRes::One(curr_str.clone()));
                    res.extend(
                        curr.value
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|x| StreamRes::Two((x.0.to_string(), x.1.to_string())))
                            .collect::<Vec<StreamRes<String>>>(),
                    );
                }
            }
            for (child, _) in curr.children.iter() {
                //let count = child.char().count();
                println!("child={:?}", child,);
                let (mut child_greater_than_start, mut child_smaller_than_end) =
                    (cmp::Ordering::Equal, cmp::Ordering::Equal);
                let mut start_iter = start.chars();
                let child_iter = child.chars();
                let (s, e);
                for c1 in child_iter {
                    let temp = start_iter.next();
                    let c2;
                    match temp {
                        Some(c) => c2 = c,
                        None => {
                            break;
                        }
                    }
                    if c1 == c2 {
                        continue;
                    } else if c1 > c2 {
                        child_greater_than_start = cmp::Ordering::Greater;
                        break;
                    } else {
                        child_greater_than_start = cmp::Ordering::Less;
                        break;
                    }
                }
                let k1;
                match child_greater_than_start {
                    cmp::Ordering::Equal => {
                        k1 = start_iter.collect::<String>();
                        s = k1.as_str();
                    }
                    cmp::Ordering::Greater => {
                        s = "000000000000";
                    }
                    cmp::Ordering::Less => {
                        continue;
                    }
                }

                let mut end_iter = end.chars();
                let child_iter = child.chars();
                for c1 in child_iter {
                    let c2 = end_iter.next().unwrap();
                    if c1 == c2 {
                        continue;
                    } else if c1 < c2 {
                        child_smaller_than_end = cmp::Ordering::Less;
                        break;
                    } else {
                        child_smaller_than_end = cmp::Ordering::Greater;
                        break;
                    }
                }
                let k2;
                match child_smaller_than_end {
                    cmp::Ordering::Equal => {
                        k2 = end_iter.collect::<String>();
                        e = k2.as_str();
                    }
                    cmp::Ordering::Less => {
                        e = "999999999999";
                    }
                    cmp::Ordering::Greater => {
                        continue;
                    }
                }

                RadixTree::search_in_range(
                    &curr.children[child],
                    s,
                    e,
                    curr_str.clone() + child.as_str(),
                    res,
                    start_len,
                    end_len,
                );

                // } else if end.get(0..count).unwrap() > child.as_str() {
                //     continue;
                // }
                // let (mut s, mut e) = (start, end);
                // if start.get(0..count).unwrap() == child {
                //     s = start.get(count..child.len()).unwrap();
                // } else {
                //     s = "0000000000";
                // }
                // if end.get(0..count).unwrap() == child {
                //     e = end.get(count..child.len()).unwrap();
                // } else {
                //     e = "9999999999";
                // }

                // RadixTree::search_in_range(
                //     &curr.children[child],
                //     s,
                //     e,
                //     curr_str.clone() + child,
                //     res,
                // );

                // if is_greater_or_equal(child, &start) && is_greater_or_equal(&end, child) {
                //     Self::search_in_range(
                //         &curr.children[child],
                //         &start,
                //         &end,
                //         curr_str.clone() + child,
                //         res,
                //     );
            }
        }
    }

    pub fn is_greater_or_equal(s1: &str, s2: &str) -> bool {
        let mut s1_iter = s1.chars();
        let mut s2_iter = s2.chars();
        let count1 = s1_iter.clone().count();
        let count2 = s2_iter.clone().count();
        let count = cmp::min(count1, count2);
        for _ in 0..count {
            let (c1, c2) = (s1_iter.next(), s2_iter.next());
            if dbg!(c1) == dbg!(c2) {
                continue;
            } else if dbg!(c1) > dbg!(c2) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    //Search for a specfic string
    // fn search(self, s: String) {
    //     let s_iter = s.chars();
    //     let node = self.root;
    //     let remaining = s;
    //     while remaining.len() > 0 {
    //         let found = false;
    //         for (edge_label, child) in node.children.iter() {
    //             let temp = s_iter.next().unwrap();
    //             let start: char = edge_label.chars().next().unwrap();
    //             let flag = True;
    //             if temp == start {
    //                 while let Some(c) = start.next() {
    //                     if c == temp.next() {
    //                         continue;
    //                     } else {
    //                         flag = False;
    //                         break;
    //                     }
    //                 }
    //                 if flag {
    //                     s_iter = temp;
    //                     node = node.children.get(edge_label);
    //                     remaining = break;
    //                 } else {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     if !found {
    //         break;
    //     }

    #[cfg(test)]
    mod tests {
        use crate::radix_tree::{self, RadixTree, RadixTreeNode};
        use std::collections::HashMap;

        fn it_works() {
            let value = radix_tree::is_greater_or_equal("0-3", "0-");
            println!("inside test");
            assert_eq!(true, value);
        }

        #[test]
        fn check_search_in_range() {
            let mut rt = RadixTree::new(RadixTreeNode {
                children: HashMap::new(),
                value: None,
                end: false,
            });
            rt.insert(
                "0-1".to_string(),
                HashMap::from([("foo".to_string(), "bar".to_string())]),
            );
            rt.insert(
                "0-2".to_string(),
                HashMap::from([("baz".to_string(), "foo".to_string())]),
            );
            rt.insert(
                "0-3".to_string(),
                HashMap::from([("bar".to_string(), "baz".to_string())]),
            );
            let mut res: Vec<radix_tree::StreamRes<String>> = Vec::new();

            RadixTree::search_in_range(&rt.root, "0-1", "0-3", "".to_string(), &mut res, 3, 3);
            println!("{:#?},{:?}", rt, res);
        }
    }
}
