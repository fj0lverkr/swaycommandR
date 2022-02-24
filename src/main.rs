fn next_workspace_number(conn: &mut swayipc::Connection) -> Result<i32, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    let mut ids: Vec<i32> = workspaces.iter().map(|w| w.num).collect();
    ids.sort_unstable();
    let len = ids.len() as i32;
    Ok(ids
        .into_iter()
        .enumerate()
        .find(|&(idx, workspace_num)| idx as i32 + 1 != workspace_num)
        .map_or(len + 1, |(idx, _)| idx as i32 + 1))
}

fn active_workspace_numbers(conn: &mut swayipc::Connection) -> Result<Vec<i32>, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    Ok(workspaces.iter().map(|w| w.num).collect())
}

fn main() -> Result<(), swayipc::Error> {
    let mut conn = swayipc::Connection::new()?;
    let next_ws = next_workspace_number(&mut conn)?;
    let active_ws = active_workspace_numbers(&mut conn)?;
    for ws in active_ws {
        println!("WS in use: {}",ws);
    }
    println!("Next free WS: {}", next_ws);
    Ok(())
}
