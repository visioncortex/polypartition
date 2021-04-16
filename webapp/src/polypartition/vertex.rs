use std::{cell::RefCell, rc::Rc};

use visioncortex::PointF64;

// Pointer type that supports interior mutability
pub type PartitionVertexPtr = Rc<RefCell<PartitionVertex>>;

#[derive(Debug, Default)]
pub struct PartitionVertex {
    pub info: PartitionVertexInfo,
    pub previous: Option<PartitionVertexPtr>,
    pub next: Option<PartitionVertexPtr>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartitionVertexInfo {
    pub id: usize, // For debugging
    pub is_active: bool,
    pub is_convex: bool,
    pub is_ear: bool,

    /// The point at the vertex
    pub p: PointF64,
    pub angle: f64,
}

impl PartitionVertex {
    /// Get the information stored in this PartitionVertex
    pub fn get_info(&self) -> PartitionVertexInfo {
        self.info.clone()
    }

    pub fn get_previous_info(&self) -> Option<PartitionVertexInfo> {
        if let Some(prev) = &self.previous {
            Some(prev.borrow().get_info())
        } else {
            None
        }
    }

    pub fn get_next_info(&self) -> Option<PartitionVertexInfo> {
        if let Some(next) = &self.next {
            Some(next.borrow().get_info())
        } else {
            None
        }
    }

    pub fn set_info(&mut self, info: PartitionVertexInfo) {
        self.info = info;
    }

    pub fn set_previous(&mut self, prev: &PartitionVertexPtr) {
        self.previous = Some(Rc::clone(prev));
    }

    pub fn set_next(&mut self, next: &PartitionVertexPtr) {
        self.next = Some(Rc::clone(next));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_vertex_setup_chain() {
        // Create and make sure they are different (for testing)
        let pv1 = PartitionVertexPtr::default();
        pv1.borrow_mut().info.is_active = true;
        let pv2 = PartitionVertexPtr::default();
        pv2.borrow_mut().info.is_convex = true;
        let pv3 = PartitionVertexPtr::default();
        pv3.borrow_mut().info.is_ear = true;

        // Chain 'em up
        pv1.borrow_mut().set_previous(&pv3);
        pv1.borrow_mut().set_next(&pv2);
        pv2.borrow_mut().set_previous(&pv1);
        pv2.borrow_mut().set_next(&pv3);
        pv3.borrow_mut().set_previous(&pv2);
        pv3.borrow_mut().set_next(&pv1);

        assert_eq!(pv1.borrow().get_previous_info().unwrap(), pv3.borrow().get_info());
        assert_eq!(pv1.borrow().get_next_info().unwrap(), pv2.borrow().get_info());
        assert_eq!(pv2.borrow().get_previous_info().unwrap(), pv1.borrow().get_info());
        assert_eq!(pv2.borrow().get_next_info().unwrap(), pv3.borrow().get_info());
        assert_eq!(pv3.borrow().get_previous_info().unwrap(), pv2.borrow().get_info());
        assert_eq!(pv3.borrow().get_next_info().unwrap(), pv1.borrow().get_info());

        // pv1->next->next = pv3
        assert_eq!(pv1.borrow().next.as_ref().unwrap().borrow().get_next_info().unwrap(), pv3.borrow().get_info());
        // pv1->next->prev = pv1
        assert_eq!(pv1.borrow().next.as_ref().unwrap().borrow().get_previous_info().unwrap(), pv1.borrow().get_info());
    }

    #[test]
    fn partition_vertex_remove_from_chain() {
        // Create and make sure they are different (for testing)
        let pv1 = PartitionVertexPtr::default();
        pv1.borrow_mut().info.is_active = true;
        let pv2 = PartitionVertexPtr::default();
        pv2.borrow_mut().info.is_convex = true;
        let pv3 = PartitionVertexPtr::default();
        pv3.borrow_mut().info.is_ear = true;

        // Chain 'em up
        pv1.borrow_mut().set_previous(&pv3);
        pv1.borrow_mut().set_next(&pv2);
        pv2.borrow_mut().set_previous(&pv1);
        pv2.borrow_mut().set_next(&pv3);
        pv3.borrow_mut().set_previous(&pv2);
        pv3.borrow_mut().set_next(&pv1);

        // Remove pv2 which is in the middle
        // Simply ignore the existence of pv2 here for this testing
        pv1.borrow_mut().set_next(&pv3);
        pv3.borrow_mut().set_previous(&pv1);

        assert_eq!(pv1.borrow().get_previous_info().unwrap(), pv3.borrow().get_info());
        assert_eq!(pv1.borrow().get_next_info().unwrap(), pv3.borrow().get_info());
        assert_eq!(pv3.borrow().get_previous_info().unwrap(), pv1.borrow().get_info());
        assert_eq!(pv3.borrow().get_next_info().unwrap(), pv1.borrow().get_info());
        
        // pv1->next->next = pv1
        assert_eq!(pv1.borrow().next.as_ref().unwrap().borrow().get_next_info().unwrap(), pv1.borrow().get_info());
        // pv1->next->prev = pv1
        assert_eq!(pv1.borrow().next.as_ref().unwrap().borrow().get_previous_info().unwrap(), pv1.borrow().get_info());
    }
}