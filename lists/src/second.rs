use std::mem;

pub struct List {
    head: Link,
}

// yay type aliases!
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            //next: mem::replace(&mut self.head, None),
            //用take这个方法替换
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        //mem::replace(&mut self.head, None)
        /*let opt = self.head.take();
        match opt {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }*/

        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        //let mut cur_link = mem::replace(&mut self.head, None);
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            //cur_link = mem::replace(&mut boxed_node.next, None);
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::second::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // 检查空列表行为正确
        assert_eq!(list.pop(), None);

        // 填充列表
        list.push(1);
        list.push(2);
        list.push(3);

        // 检查通常的移除
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // 推入更多元素来确认没有问题
        list.push(4);
        list.push(5);

        // 检查通常的移除
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // 检查完全移除
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
