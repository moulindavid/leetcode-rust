struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(list1), None) => Some(list1),
            (None, Some(list2)) => Some(list2),
            (None, None) => None,
            (Some(l1), Some(l2)) => match l1.val <= l2.val {
                true => {
                    return Some(Box::new(ListNode {
                        val: l1.val,
                        next: Self::merge_two_lists(l1.next, Some(l2)),
                    }));
                }

                false => {
                    return Some(Box::new(ListNode {
                        val: l2.val,
                        next: Self::merge_two_lists(Some(l1), l2.next),
                    }));
                }
            },
        }
    }
    pub fn merge_two_lists_non_recursive(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut r = &mut list1;
        while list2.is_some() {
            if r.is_none() || list2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut list2);
            }
            r = &mut r.as_mut()?.next;
        }
        return list1;
    }
}
