use obj::{ObjData, Object, Group};

use crate::raytrack::{bvh::{GeomInfo, BVHNode}, boundbox::HitRecord};
use crate::raytrack::ray::Ray;
pub struct ModelData{
    pub   data: ObjData,
    pub   bvh_tree:Option< Box<BVHNode>>,
}
impl ModelData{
    pub fn new(path :String,name :String)->Self{
       if let Ok(mut obj)=obj::Obj::load(path.to_owned()+name.as_str()){
            obj.load_mtls();
            let mut this=Self{
                data:obj.data,
                bvh_tree:None,
            };
            this.build_bvh();
            this
       }else{
            panic!("没有这个文件");
       }
    }
    fn build_bvh(&mut self){
        let tree=BVHNode::new(self);
        self.bvh_tree=Some(Box::new(tree));
    }
    pub fn hit(&self,ray:&Ray,t_min:f32,t_max:f32)->Option<HitRecord>{
        match self.bvh_tree{
            Some(ref v)=>{
                v.hit(ray, self, t_min, t_max)
            },
            _=>None,
        }
    }
    pub fn as_shape(&self,index :usize)->Option<&Vec<Group>>{
        let obj = self.data.objects.get(index);
        if let Some(obj)=obj{
            Some(&obj.groups)
        }else{
            None
        }
    }
}
#[cfg(test)]
mod test{
    use crate::object::object::ModelData;
    #[test]
    pub fn test_obj(){
       let model= ModelData::new("obj/".to_string(),String::from("untitled.obj"));
       
    }
}