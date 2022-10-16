struct RenderOpts {
    screen_width: usize,    
}

struct IntTreeNode {
    start: i32,
    end: i32,
    depth: i32,
    left: Option<Box<IntTreeNode>>,
    right: Option<Box<IntTreeNode>>,
}

impl IntTreeNode {
    fn new(start: i32, end: i32, depth: i32) -> Self {
        IntTreeNode { start, end, depth, left: None, right: None }
    }

    fn create(start: i32, end: i32, depth: i32) -> Option<Box<IntTreeNode>> {
        Some(Box::new(IntTreeNode::new(start, end, depth)))
    }

    fn insert(node: &mut Option<Box<IntTreeNode>>, start: i32, end: i32, curr_depth: i32, max_depth: &mut i32) {
        if start < end {
            if let Some(node) = node {
                if node.start == start && node.end == end {
                    node.depth += 1;
                } else if node.start >= end {
                    IntTreeNode::insert(&mut node.left, start, end, curr_depth, max_depth)
                } else if node.end <= start {
                    IntTreeNode::insert(&mut node.right, start, end, curr_depth, max_depth)
                } else {
                    if node.start >= start {
                        IntTreeNode::insert(&mut node.left, start, node.start, curr_depth, max_depth)
                    } else {
                        IntTreeNode::insert(&mut node.left, node.start, start, node.depth, max_depth)
                    };
        
                    if node.end < end {
                        IntTreeNode::insert(&mut node.right, node.end, end, curr_depth, max_depth);
                    } else {
                        IntTreeNode::insert(&mut node.right, end, node.end, node.depth, max_depth);
                    };
                    
                    node.start = node.start.max(start);
                    node.end = node.end.min(end);
                    node.depth += 1;
                }
            } else {
                *node = IntTreeNode::create(start, end, curr_depth);
            }

            *max_depth = node.as_ref().unwrap().depth.max(*max_depth);
        }
    }

    fn println(node: &Option<Box<IntTreeNode>>, indent: usize, buffer: &mut Vec<i32>) {
        if let Some(node) = node {
            for ix in node.start..node.end {
                buffer[ix as usize] += 1;
            }

            IntTreeNode::println(&node.left, indent + 2, buffer);
            IntTreeNode::println(&node.right, indent + 2, buffer);
        }
    }

    fn print_nodes(node: &Option<Box<IntTreeNode>>) {
        if let Some(node) = node {
            IntTreeNode::print_nodes(&node.left);
            print!("{}-{}({});", node.start, node.end, node.depth);
            IntTreeNode::print_nodes(&node.right);
        }
    }

    fn render(&self, opts: &RenderOpts) -> String {
        format!("<th><div>{} {} {}</div></th>", self.start, self.depth, self.end)
    }

    fn to_html(node: &Option<Box<IntTreeNode>>, opts: &RenderOpts) -> String {
        if let Some(node) = node {
            format!("{}{}{}", IntTreeNode::to_html(&node.left, opts), node.render(opts), IntTreeNode::to_html(&node.right, opts))
        } else {
            "".to_string()
        }
    }
}

struct MyCalendarThree {
    tree: Option<Box<IntTreeNode>>,
    depth: i32,
    start: i32,
    end: i32
}

impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree { tree: None, depth: 0, start: 0, end: 0 }
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        IntTreeNode::insert(&mut self.tree, start, end, 1, &mut self.depth);
        self.start = self.start.min(start);
        self.end = self.start.max(end);
        self.depth
    }

    fn render_opts(&self) -> RenderOpts {
        RenderOpts { screen_width: 0 }
    }

    fn to_html(&self, opts: &RenderOpts) -> String {
        if self.tree.is_some() {
            format!("<html><body><table><tr>{}</tr></table></body></html>", IntTreeNode::to_html(&self.tree, opts))
        } else {
            "<div>empty</div>".to_string()
        }
    }
}

fn main() {
    /*
    let mut calendar = MyCalendarThree::new();

    for b in vec![(10,20),(50,60),(10,40),(5,15),(5,10),(25,55)].iter() {
        println!("{}", calendar.book(b.0, b.1));
        // IntTreeNode::println(&calendar.tree, 0);
    }

    let mut calendar = MyCalendarThree::new();

    fn remap(i: i32) -> char {
        match i {
            0 => '_',
            1 => '1',
            2 => '2',
            _ => '*'
        }
    }
    
    for b in vec![(24,40),(43,50),(27,43),(5,21),(30,40),(14,29),(3,19),(3,14),(25,39),(6,19)].iter() {
        println!("{}", calendar.book(b.0, b.1));
        //let mut buffer = vec![0; 80];
        //print!("{} - {}: ", b.0, b.1);
        //IntTreeNode::println(&calendar.tree, 0, &mut buffer);
        //println!();
        //println!("{}", buffer.iter().map(|i| remap(*i)).collect::<String>());
    }
    */

    let mut calendar = MyCalendarThree::new();

    // for b in vec![(97,100),(51,65),(27,46),(90,100),(20,32),(15,28),(60,73),(77,91),(67,85),(58,72),(74,93),(73,83),(71,87),(97,100),(14,31),(26,37),(66,76),(52,67),(24,43),(6,23),(94,100),(33,44),(30,46),(6,20),(71,87),(49,59),(38,55),(4,17),(46,61),(13,31),(94,100),(47,65),(9,25),(4,20),(2,17),(28,42),(26,38),(72,83),(43,61),(18,35)] {
    for b in vec![(97,100),(51,65),(27,46),(90,100),(20,32)] {
        println!("{}", calendar.book(b.0, b.1));
        println!("{}", calendar.to_html(&RenderOpts { screen_width: 80 }));
    }
}
