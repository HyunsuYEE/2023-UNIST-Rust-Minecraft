pub struct Player {
    health: u32,
    mana: Option<u32>,
    level: u32,

    
}
impl Player{
    fn revive(&self) -> (){
        if self.health == 0 {
            match self.mana{
                None =>{println!("Player  health: {}, mana: {}, level: {}", 100, "None", self.level);}
                _ => {println!("Player  health: {}, mana: {}, level: {}", 100, self.mana.unwrap(), self.level);}
            }
            
        }else{println!("None");}
    }
    fn cast_spell(&mut self, mana_cost :u32) -> u32 {
        
        // 첫번째, 일단 레벨이 10이 되는지 확인
        // 두번째, 10이 안된다면 체력을 mana_cost만큼 깎음, 10이 된다면
        // mana_cost와 현재 마나를 비교 => 현재마나보다 mana_cost가 크면 0, 사용 가능하면 ~~
        if self.level < 10{ 
            self.health = self.health - mana_cost;
            return 0;
        }else{
            let mut mana: u32 = self.mana.unwrap();
            if mana >= mana_cost{
                mana = mana - mana_cost;
                let mana1 = mana;
                return mana1;
            }else{ return 0; }

        }
        
    }
}

fn main() {
    let mut not_a_wizard_yet = Player { health: 79, mana: None, level: 9 };
    assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
    assert_eq!(not_a_wizard_yet.health, 74);
assert_eq!(not_a_wizard_yet.mana, None);
}
