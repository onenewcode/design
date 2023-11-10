use std::fmt::Debug;
trait Attackable {
    fn attack<T>(&self, target: &mut T)
    where
        T: Attackable + Debug;
    fn get_ack(&self) -> u32;
    fn get_hp(&self) -> u32;
    fn set_hp(&mut self, hp: u32);
    fn dead(&self) -> bool;
}

#[derive(Debug)]
struct Hero {
    hp: u32,
    ack: u32,
}

#[derive(Debug)]
struct Monster {
    hp: u32,
    ack: u32,
}

macro_rules! impl_attackable {
    ($($str:ident),+) => {
        $(impl Attackable for $str {
            fn attack<T>(&self, target: &mut T)
            where
                T: Attackable + Debug,
            {
                let ack = self.get_ack();
                let target_hp = target.get_hp();
                if target.dead() || self.dead() {
                    return;
                } else if target_hp <= ack {
                    target.set_hp(0);
                    println!("{:?} dead", target);
                    return;
                } else {
                    target.set_hp(target_hp - ack);
                    println!("{:?} attack {:?}, damage {}", self, target, self.get_ack());
                }
            }

            fn get_hp(&self) -> u32 {
                self.hp
            }

            fn set_hp(&mut self, hp: u32) {
                self.hp = hp
            }

            fn get_ack(&self) -> u32 {
                self.ack
            }

            fn dead(&self) -> bool {
                self.hp == 0
            }
        })*
    };
}

impl_attackable!(Hero, Monster);

fn main() {
    let mut master = Hero { hp: 100, ack: 10 };
    let mut sprite = Monster { hp: 20, ack: 5 };

    loop {
        master.attack(&mut sprite);
        sprite.attack(&mut master);
        if master.dead() || sprite.dead() {
            break;
        }
    }
}
