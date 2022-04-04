use std::{fmt::{Display, Pointer}, io::BufRead, fs};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum NodeValue {
    Value(usize),
    Tuple(Box<NodeValue>, Box<NodeValue>)
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeValue::Value(v) => v.fmt(f),
            NodeValue::Tuple(l, r) => f.write_fmt(format_args!("[{},{}]", l, r)),
        }
    }
}

impl NodeValue {
    fn add_left(self, value: usize) -> NodeValue {
        match self {
            NodeValue::Value(v) => NodeValue::Value(v + value),
            NodeValue::Tuple(l, r) => NodeValue::Tuple(Box::new((*l).add_left(value)), r)
        }
    }
    fn add_right(self, value: usize) -> NodeValue {
        match self {
            NodeValue::Value(v) => NodeValue::Value(v + value),
            NodeValue::Tuple(l, r) => NodeValue::Tuple(l, Box::new((*r).add_right(value)))
        }
    }
    fn add(self, other: NodeValue) -> NodeValue {
        NodeValue::Tuple(Box::new(self), Box::new(other))
    }
    fn mag(&self) -> usize {
        match self {
            Self::Value(v) => *v,
            NodeValue::Tuple(l, r) => 3 * l.mag() + 2 * r.mag()
        }
    }
}

pub fn puzzle1() {
    let input = fs::read_to_string("input-day18.txt").expect("error reading input file");
    let mut lines = input.lines();
    let mut nodes = parse_node(&mut lines.next().unwrap().chars().collect());
    //nodes = reduce(nodes);

    for line in lines {
        let new_nodes = parse_node(&mut line.chars().collect());
        nodes = nodes.add(new_nodes);
        nodes = reduce(nodes);
    }
    
    println!("{}", nodes);
    let magnitude = nodes.mag();
    println!("magnitude is {}", magnitude);
    assert_eq!(3884, magnitude);
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day18.txt").expect("error reading input file");
    let lines = input.lines();
    let nodes: Vec<NodeValue> = lines.map(|l| parse_node(&mut l.chars().collect())).collect();

    let mut highest = 0;
    for node1 in nodes.iter() {
        for node2 in nodes.iter().filter(|n| *n != node1) {
            let n = node1.clone().add(node2.clone());
            let n = reduce(n);
            let mag = n.mag();
            if mag > highest {
                highest = mag;
            }
        }
    }

    println!("highest magnitued is {}", highest);
    assert_eq!(4595, highest);
}

fn reduce(mut node: NodeValue) -> NodeValue{
    loop {
        // check for nested first:
        let (exploded, _, _, has_exploded) = explode(node, 1);
        node = exploded;
        if has_exploded {
            continue;
        }

        let (split, has_split) = split(node);
        node = split;
        if has_split {
            continue;
        }

        // reduce completed
        return node;
    }
}

fn split(node: NodeValue) -> (NodeValue, bool) {
    match node {
        NodeValue::Value(v) if v >= 10 => {
            (NodeValue::Tuple(Box::new(NodeValue::Value(v/2)), Box::new(NodeValue::Value((v+1)/2))), true)
        },
        NodeValue::Tuple(l, r) => {
            let new_l = split(*l);
            let new_r = if new_l.1 { (*r, false) } else { split(*r) }; //make sure to stop if left side has already been split
            (NodeValue::Tuple(Box::new(new_l.0), Box::new(new_r.0)), new_l.1 || new_r.1)
        }
        _ => (node, false)
    }
}

// I'm so sorry
fn explode(node: NodeValue, depth: usize) -> (NodeValue, Option<usize>, Option<usize>, bool) {
    match node {
        NodeValue::Value(_) => (node, None, None, false),
        NodeValue::Tuple(l, r) if depth < 4 => {
            let new_l = explode(*l, depth + 1);
            if new_l.3 { // has changed
                let mut new_r = *r;
                if let Some(add_right) = new_l.2 {
                    new_r = new_r.add_left(add_right);
                }
                let new_node = NodeValue::Tuple(Box::new(new_l.0), Box::new(new_r));
                (new_node, new_l.1, None, true)
            } else {
                let new_r = explode(*r, depth + 1);
                if new_r.3 { // has changed
                    let mut new_l = new_l.0;
                    if let Some(add_left) = new_r.1 {
                        new_l = new_l.add_right(add_left);
                    }
                    let new_node = NodeValue::Tuple(Box::new(new_l), Box::new(new_r.0));
                    (new_node, None, new_r.2, true)
                } else {
                    (NodeValue::Tuple(Box::new(new_l.0), Box::new(new_r.0)), None, None, false)
                }
            }

        },
        NodeValue::Tuple(l, r) => {
            let left_result = try_explode(&l);
            if let (new_node, add_left, Some(add_right)) = left_result {
                (NodeValue::Tuple(Box::new(new_node), Box::new(r.add_left(add_right))), add_left, None, true)
            } else {
                let right_result = try_explode(&r);
                if let (new_node, Some(add_left), add_right) = right_result {
                    (NodeValue::Tuple(Box::new(l.add_right(add_left)), Box::new(new_node)), None, add_right, true)
                } else {
                    (NodeValue::Tuple(l, r), None, None, false)
                }
            }
        }
    }
}

fn try_explode(node: &NodeValue) -> (NodeValue, Option<usize>, Option<usize>) {
    match node {
        NodeValue::Tuple(l, r) => {
            if let NodeValue::Value(lv) = **l {
                if let NodeValue::Value(rv) = **r {
                    return (NodeValue::Value(0), Some(lv), Some(rv));
                }
            }
            (node.clone(), None, None)
        },
        NodeValue::Value(v) => (NodeValue::Value(*v), None, None)
    }
}

fn parse_node(input: &mut Vec<char>) -> NodeValue {
    let c = input.remove(0);
    match c {
        '[' => {
            let l = parse_node(input);
            let r = parse_node(input);
            NodeValue::Tuple(Box::new(l), Box::new(r))
        },
        ',' => parse_node(input),
        ']' => parse_node(input),
        _ => {
            let mut chars = vec![c];
            loop{
                let n = input.remove(0);
                if n == ',' || n == ']' {
                    let value = String::from_iter(chars).parse::<usize>().unwrap();
                    return NodeValue::Value(value);
                } else {
                    chars.push(n);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_reduce(){
        assert_eq!(explode(parse_node(&mut "[[[[[9,8],1],2],3],4]".chars().collect()), 1).0.to_string(), "[[[[0,9],2],3],4]");
        assert_eq!(explode(parse_node(&mut "[7,[6,[5,[4,[3,2]]]]]".chars().collect()), 1).0.to_string(), "[7,[6,[5,[7,0]]]]");
        assert_eq!(explode(parse_node(&mut "[[6,[5,[4,[3,2]]]],1]".chars().collect()), 1).0.to_string(), "[[6,[5,[7,0]]],3]");
        assert_eq!(explode(parse_node(&mut "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars().collect()), 1).0.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(explode(parse_node(&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars().collect()), 1).0.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        assert_eq!(explode(parse_node(&mut "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]".chars().collect()), 1).0.to_string(), "[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]");
    }
}