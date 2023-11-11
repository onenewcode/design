// 设置人类实体类
#[derive(Clone)]
struct Person{
    name:String,
    gender:String,
    marital_status:String
}
impl Person {
 
    fn get_gender(&self)->&str {
        self.gender.as_ref()
    }
    fn get_marital_status(&self)->&str {
        self.marital_status.as_ref()
    }
}
// 设置过滤标准特征
trait Criteria {
  fn meet_criteria(&self,persons:Vec<Person>)->Vec<Person>; 
}
// 设置男性
struct CriteriaMale{}
// 重写男性评判标准
impl   Criteria for CriteriaMale {
    fn meet_criteria(&self,persons:Vec<Person>)->Vec<Person>{
        persons.into_iter().filter(|x| x.get_gender().eq_ignore_ascii_case("MALE")).collect()
    }
}
// 设置女性
struct CriteriaFemale{}
// 设置女性标准
impl   Criteria for CriteriaFemale{
    fn meet_criteria(&self,persons:Vec<Person>)->Vec<Person>{
        persons.into_iter().filter(|x| x.get_gender().eq_ignore_ascii_case("FEMALE")).collect()
    }
}
// 设置单身标准
struct CriteriaSingle{}

impl   Criteria for CriteriaSingle{
    fn meet_criteria(&self,persons:Vec<Person>)->Vec<Person>{
        persons.into_iter().filter(|x| x.get_marital_status().eq_ignore_ascii_case("SINGLE")).collect()
    }
}
// 设置and标准，求交集
struct AndCriteria  {
    criteria:Box<dyn Criteria>,
    other_criteria:Box<dyn Criteria>
}

impl Criteria for AndCriteria {
    fn meet_criteria(&self,persons:Vec<Person>)->Vec<Person> {
        self.other_criteria.meet_criteria(self.criteria.meet_criteria(persons))
    }
}
// struct OrCriteria {
//     criteria:Box<dyn Criteria>,
//     other_criteria:Box<dyn Criteria>
// }

// impl Criteria for OrCriteria{
//     fn meet_criteria(&self,persons:Vec<Person>)->Vec<Person> {
//         let mut first_criteria_items = self.criteria.meet_criteria(persons);
//         let other_criteria_items = self.other_criteria.meet_criteria(persons);
//         for o in other_criteria_items  {
//             if !other_criteria_items.contains(&o) {
//                 first_criteria_items.push(o);
//             }
//         }
//         first_criteria_items

//     }
// }
fn print_persons(persons:Vec<Person>) {
    persons.into_iter().for_each(|x| println!("Person : [NAME: {},Gender : {} ,Maeital Status : {} ]",x.name,x.gender,x.marital_status));
}
fn main() {
    let mut persons=Vec::new();
    persons.push(Person{name:String::from("Robert"),gender:String::from("Male"),marital_status:String::from("Single")});
    persons.push(Person{name:String::from("John"),gender:String::from("Male"),marital_status:String::from("Married")});
    persons.push(Person{name:String::from("Laura"),gender:String::from("Female"),marital_status:String::from("Married")});
    persons.push(Person{name:String::from("Diana"),gender:String::from("Female"),marital_status:String::from("Single")});

    let male=Box::new(CriteriaMale{});
    let female=Box::new(CriteriaFemale{});
    let single_male=AndCriteria{criteria:Box::new(CriteriaSingle{}),other_criteria:Box::new(CriteriaMale{})};
    // let single_or_female=OrCriteria{criteria:Box::new(CriteriaMale{}),other_criteria:Box::new(CriteriaFemale{})};
    println!("男士");
    print_persons(male.meet_criteria(persons.clone()));
    println!("女士");
    print_persons(female.meet_criteria(persons.clone()));
    println!("单身男士");
    print_persons(single_male.meet_criteria(persons))


}