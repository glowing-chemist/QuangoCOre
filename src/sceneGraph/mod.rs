use shapes::*;
use errors::DrawStatus;


pub struct SceneNode<T : Transformable + Draw> {
    tag : String,
    children : Vec<Box<T>>
}



impl<T : Transformable + Draw> SceneNode<T> {
    pub fn new(name : String) -> SceneNode<T> {
        SceneNode{tag : name, children : Vec::<Box<T>>::new()}
    }

    pub fn add_child(&mut self, node : Box<T>) {
        self.children.push(node);
    }
}



impl<T : Transformable + Draw> Draw for SceneNode<T> {
    fn draw(&self) -> DrawStatus {
        let failures  : Vec<DrawStatus> = self.children.iter().map(|drawable| drawable.draw())
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



impl<T : Transformable + Draw> Transformable for SceneNode<T> {

    fn translate(&mut self, posx : f32, posy : f32) {
        for i in 0..self.children.len() {
            self.children[i].translate(posx, posy);
        }
    }

    fn scale(&mut self, amountx : f32, amounty : f32) {
        for i in 0..self.children.len() {
            self.children[i].scale(amountx, amounty);
        }
    }

    fn rotate(&mut self, degrees : f32, axis : Axis) {
        for i in 0..self.children.len() {
            self.children[i].rotate(degrees, axis);
        }
    }

}