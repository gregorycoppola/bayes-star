use std::error::Error;
use crate::{print_red, print_yellow};
use super::inference::Inferencer;

impl Inferencer {
    pub fn send_pi_messages(&mut self) -> Result<(), Box<dyn Error>> {
        let bfs_order = self.bfs_order.clone();
        print_red!("send_pi_messages bfs_order: {:?}", &bfs_order);
        for node in &bfs_order {
            print_yellow!("send pi bfs selects {:?}", node);
            self.pi_visit_node(node)?;
        }
        Ok(())
    }
}