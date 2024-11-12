use advent::prelude::*;

#[derive(HasParser, Clone)]
enum Direction {
    #[parse(string = "L")]
    Left,
    #[parse(string = "R")]
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct NodeName(String);

impl HasParser for NodeName {
    #[into_parser]
    fn parser() -> _ {
        many1(alpha_num()).map(Self)
    }
}

#[derive(HasParser)]
struct GraphInput {
    #[parse(after = " =")]
    start: NodeName,
    #[parse(before = "(", after = ",")]
    left: NodeName,
    #[parse(after = ")")]
    right: NodeName,
}

#[derive(HasParser)]
#[parse(sep_by = "\n\n")]
struct Input {
    instructions: Vec<Direction>,
    graph: List<GraphInput, TermWith<NewLine>>,
}

struct Node {
    #[allow(dead_code)]
    name: NodeName,
    left: Option<NodeName>,
    right: Option<NodeName>,
}

impl Node {
    fn new(name: NodeName) -> Self {
        Self {
            name,
            left: None,
            right: None,
        }
    }
}

fn build_graph(input: &Input) -> HashMap<NodeName, Node> {
    let mut graph = HashMap::new();
    for i in &input.graph {
        let node = graph
            .entry(i.start.clone())
            .or_insert(Node::new(i.start.clone()));
        node.left = Some(i.left.clone());
        node.right = Some(i.right.clone());
        if !graph.contains_key(&i.left) {
            graph.insert(i.left.clone(), Node::new(i.left.clone()));
        }
        if !graph.contains_key(&i.right) {
            graph.insert(i.right.clone(), Node::new(i.right.clone()));
        }
    }
    graph
}

#[part_one]
fn part_one(input: Input) -> u128 {
    let graph = build_graph(&input);
    let mut cur = NodeName("AAA".into());
    if !graph.contains_key(&cur) {
        return 0;
    }
    let mut num_steps = 0;
    for i in input.instructions.into_iter().cycle() {
        num_steps += 1;
        match i {
            Direction::Left => {
                cur = graph.get(&cur).unwrap().left.clone().unwrap();
            }
            Direction::Right => {
                cur = graph.get(&cur).unwrap().right.clone().unwrap();
            }
        }
        if cur == NodeName("ZZZ".into()) {
            break;
        }
    }
    num_steps
}

#[derive(Debug)]
struct Search {
    cur: NodeName,
    seen: HashMap<(NodeName, u128), u128>,
    path: Vec<NodeName>,
    looped: bool,
    loop_offset: u128,
}

impl Search {
    fn new(cur: NodeName) -> Self {
        let mut seen = HashMap::new();
        seen.insert((cur.clone(), 0), 0);
        Self {
            cur,
            seen,
            path: vec![],
            looped: false,
            loop_offset: 0,
        }
    }

    fn travel(&mut self, n: u128, next: NodeName, i_n: u128) {
        if self.looped {
            self.loop_offset += 1;
            return;
        }

        if let Some(nn) = self.seen.get(&(next.clone(), i_n)) {
            self.looped = true;
            let length = n - nn;
            self.path = self.path[(self.path.len() - length as usize)..].to_vec();
        } else {
            self.seen.insert((next.clone(), i_n), n);
            self.path.push(next.clone());
            self.cur = next;
        }
    }

    fn offsets_to_z(&self) -> (u128, u128) {
        let iter = self
            .path
            .iter()
            .skip(self.loop_offset as usize)
            .chain(self.path.iter().take(self.loop_offset as usize));
        let mut nums = vec![];
        for (n, i) in iter.enumerate() {
            if i.0.ends_with("Z") {
                nums.push(n as u128);
            }
        }
        assert_eq!(nums.len(), 1);
        (nums[0], self.path.len() as u128)
    }
}

fn find_turns(first: (u128, u128), mut rest: Vec<(u128, u128)>, out: &mut u128) {
    if rest.is_empty() {
        return;
    }
    let next = rest.remove(0);

    let mut advance = 0;
    let mut t = *out % next.1;
    while t != next.0 {
        t = (t + first.1) % next.1;
        advance += first.1;
    }

    *out += advance;

    let super_wheel = (0, num::integer::lcm(first.1, next.1));
    find_turns(super_wheel, rest, out);
}

#[part_two]
fn part_two(input: Input) -> u128 {
    let graph = build_graph(&input);
    let mut curs: Vec<_> = graph
        .keys()
        .filter_map(|n| n.0.ends_with("A").then(|| Search::new(n.clone())))
        .collect();

    let instr_len = input.instructions.len();
    let mut num_steps = 0;
    for (n, i) in input.instructions.into_iter().cycle().enumerate() {
        num_steps += 1;
        for c in &mut curs {
            let nn = graph.get(&c.cur).unwrap();
            match i {
                Direction::Left => {
                    c.travel(num_steps, nn.left.clone().unwrap(), (n % instr_len) as u128);
                }
                Direction::Right => {
                    c.travel(
                        num_steps,
                        nn.right.clone().unwrap(),
                        (n % instr_len) as u128,
                    );
                }
            }
        }
        if curs.iter().all(|c| c.looped) {
            break;
        }
    }
    let mut nums: Vec<_> = curs.iter().map(|c| c.offsets_to_z()).collect();
    nums.sort();
    let first = nums.remove(0);

    // turn the wheels so the first wheel is at the end
    let initial_step = first.0;
    for n in &mut nums {
        n.0 -= initial_step;
    }

    let mut v = 0;
    find_turns((0, first.1), nums, &mut v);

    num_steps + initial_step + v
}

harness!(part_1: 22199, part_2: 13334102464297);
