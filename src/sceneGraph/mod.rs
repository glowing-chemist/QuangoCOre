use shapes::*;
use errors::DrawStatus;

use std::rc::Rc;


pub struct NodeWrapper<T : Transformable + Draw> {
    pub tag : String,
    item : T
}

impl<T : Transformable + Draw> NodeWrapper<T> {
    fn new(name : String, data : T) -> NodeWrapper<T> {
        NodeWrapper{tag : name, item : data}
    }
}


impl<T : Transformable + Draw> Transformable for NodeWrapper<T> {
    fn translate(&mut self, posx : f32, posy : f32) {
        self.item.translate(posx, posy);
    }

    fn scale(&mut self, amountx : f32, amounty : f32) {
        self.item.scale(amountx, amounty);
    }

    fn rotate(&mut self, degrees : f32, axis : Axis) {
        self.item.rotate(degrees, axis);
    }
}



impl<T : Transformable + Draw> Draw for NodeWrapper<T> {
    fn draw(&self) -> DrawStatus {
        self.item.draw()
    }
}



pub enum NodeType<T : Transformable + Draw> {
    Leaf(NodeWrapper<T>),
    Node(SceneNode<T>)
}



impl<T : Transformable + Draw> NodeType<T> {
    fn map_mut<F, R>(&mut self, func : F) -> R
    where F : FnOnce(&mut NodeType<T>) -> R {
        func(self)
    }

    fn map<F, R>(&self, func : F) -> R
    where F : FnOnce(&NodeType<T>) -> R {
        func(self)
    }
}



pub struct SceneNode<T : Transformable + Draw> {
    pub tag : String,
    children : Vec<Rc<NodeType<T>>>,
}



impl<'a, T : Transformable + Draw> SceneNode<T> {
    pub fn new(name : String) -> SceneNode<T> {
        SceneNode{tag : name, children : Vec::<Rc<NodeType<T>>>::new()}
    }

    pub fn add_child(&mut self, name : String, node : Box<T>) {
        self.children.push(Rc::new(NodeType::Leaf(NodeWrapper{tag : name, item : *node})));
    }

    pub fn get_child(&self, name : String) -> Option<Rc<NodeType<T>>> {
        for i in 0..self.children.len() {
            if self.children[i].map(|item| match item {
                &NodeType::Leaf(ref a) => a.tag.as_ref() == name,
                &NodeType::Node(ref e) => e.tag.as_ref() == name
            }) {
                return Some(self.children[i].clone()); // should be cheap as it is a Rc
            } else {

            }
        }
        None
    }
}



impl<'a, T : Transformable + Draw> Draw for SceneNode<T> {
    fn draw(&self) -> DrawStatus {
        let failures  : Vec<DrawStatus> = self.children.iter().map(|drawable : &Rc<NodeType<T>>| drawable.map(|item | match item {
            &NodeType::Leaf(ref i) => i.draw(),
            &NodeType::Node(ref e) => e.draw()
        }))
            .filter(|status| match status {
                &DrawStatus::Failed(_) => true,
                &DrawStatus::Success => false
            }).collect();

        match failures.len() {
            0 => DrawStatus::Success,
            _ => failures[0].clone()
        }
    }
}



impl<'a, T : Transformable + Draw> Transformable for SceneNode<T> {

    fn translate(&mut self, posx : f32, posy : f32) {
        for i in 0..self.children.len() {
            Rc::get_mut(&mut self.children[i]).map(|item : &mut NodeType<T>| item.map_mut(|transable : &mut NodeType<T>| match transable {
                &mut NodeType::Leaf(ref mut i) => i.translate(posx, posy),
                &mut NodeType::Node(ref mut e) => e.translate(posx, posy)
            }));
        }
    }

    fn scale(&mut self, amountx : f32, amounty : f32) {
        for i in 0..self.children.len() {
            Rc::get_mut(&mut self.children[i]).map(|item : &mut NodeType<T>| item.map_mut(|transable| match transable {
                &mut NodeType::Leaf(ref mut i) => i.scale(amountx, amounty),
                &mut NodeType::Node(ref mut e) => e.scale(amountx, amounty)
            }));
        }
    }

    fn rotate(&mut self, degrees : f32, axis : Axis) {
        for i in 0..self.children.len() {
            Rc::get_mut(&mut self.children[i]).map(|item : &mut NodeType<T>| item.map_mut(|transable| match transable {
                &mut NodeType::Leaf(ref mut i) => i.rotate(degrees, axis),
                &mut NodeType::Node(ref mut e) => e.rotate(degrees, axis)
            }));
        }
    }

}