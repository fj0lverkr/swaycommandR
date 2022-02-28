fn get_active_workspace_numbers(conn: &mut swayipc::Connection) -> Result<Vec<i32>, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    Ok(workspaces.iter().map(|w| w.num).collect())
}

fn get_workspace_name(conn: &mut swayipc::Connection, id: i32) -> Result<String, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    let mut name = String::new();
    for w in workspaces {
        if w.num == id {
            name = w.name;
        }
    }
    Ok(name)
}

fn get_tree(conn: &mut swayipc::Connection) -> Result<swayipc::Node, swayipc::Error> {
    let tree = conn.get_tree()?;
    Ok(tree)
}

fn analyze_node(tree: &swayipc::Node) {
    use swayipc::NodeType::*;
    for node in &tree.nodes {
        match node.node_type {
            Root => println!("Root node."),
            Workspace => {
                match &node.name {
                    Some(n) => println!("|-- Workspace '{}'.", n),
                    None => println!("|-- Unnamed workspace node."),
                }
                analyze_node(&node);
            }
            Con => match &node.name {
                Some(n) => println!("|--- Container '{} - {}'.",&node.id, n),
                None => println!("|--- Unnamed container '{}'.", &node.id)
            },
            Output => {
                match &node.name {
                    Some(n) => println!("|- Output '{}'.", n),
                    None => println!("|- Output node."),
                }
                analyze_node(&node);
            }
            Dockarea => println!("Dock area node."),
            FloatingCon => println!("Floating container node."),
            _ => println!("Unknows node type."),
        }
    }
}

fn main() -> Result<(), swayipc::Error> {
    let mut conn = swayipc::Connection::new()?;
    let active_ws_nums = get_active_workspace_numbers(&mut conn)?;


    for ws in active_ws_nums {
        let ws_name = get_workspace_name(&mut conn, ws)?;
        let command = format!("workspace --no-auto-back-and-forth {}", &ws_name);
        conn.run_command(command)?;
    }

    let tree = get_tree(&mut conn)?;
    // println!("{:?}", tree); //print out the tree to get a feel for the structure
    analyze_node(&tree);
    Ok(())
}
