fn active_workspace_numbers(conn: &mut swayipc::Connection) -> Result<Vec<i32>, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    Ok(workspaces.iter().map(|w| w.num).collect())
}

fn get_tree(conn: &mut swayipc::Connection) -> Result<swayipc::Node, swayipc::Error> {
    let tree = conn.get_tree()?;
    Ok(tree)
}

fn analyze_node (tree: &swayipc::Node) {
    use swayipc::NodeType::*;
    for node in &tree.nodes{
        match node.node_type {
            Root => println!("Root node."),
            Workspace => match &node.name {
                Some(n) => println!("Workspace node {}.", n),
                None => println!("Unnamed workspace node."),
            },
            Con => println!("Container node."),
            Output => match &node.name {
                Some(n) => println!("Output node {}.", n),
                None => println!("Output node."),
            },
            Dockarea => println!("Dock area node."),
            FloatingCon => println!("Floating container node."),
            _ => println!("Unknows node type."),
        }
    }
}

fn main() -> Result<(), swayipc::Error> {

    let mut conn = swayipc::Connection::new()?;
    let active_ws = active_workspace_numbers(&mut conn)?;

    for ws in active_ws {
        println!("WS in use: {}", ws);
    }

    let tree = get_tree(&mut conn)?;
    // println!("{:?}", tree); //print out the tree to get a feel for the structure
    analyze_node(&tree);
    Ok(())
}
